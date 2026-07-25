mod data;
mod ml;
mod utils;

#[pyo3::pymodule]
mod genia_libs {
    use crate::data::hypergraph::Hypergraph;
    use crate::ml::genetics::Individual;
    use crate::utils::logging::log;
    use console::{Term, style};
    use core::panic;
    use indicatif::{ProgressBar, ProgressStyle};
    use polars::{frame::DataFrame, prelude::*};
    use pyo3::PyErr;
    use pyo3::exceptions::PyTypeError;
    use pyo3::prelude::*;
    use pyo3_polars::PyDataFrame;
    use rand::distr::{Distribution, Uniform};
    use rayon::prelude::*;
    use std::path::Path;

    #[pyfunction]
    fn hypergraph_from_dataframe(py_df: PyDataFrame, output_file: String) -> PyResult<()> {
        // Transform the PyDataFrame into a Polars DataFrame
        let df: DataFrame = py_df.into();
        let mut hypergraph = Hypergraph::new(df.height());

        for column in df.get_columns() {
            let name = column.name().to_string();

            match column.dtype() {
                DataType::UInt8 => {
                    for (student_id, value) in column.u8().unwrap().into_iter().enumerate() {
                        if let Some(value) = value {
                            let hyperedge_name = format!("{}_{}", name, value);
                            hypergraph
                                .add_student_to_hyperedge(&hyperedge_name, student_id)
                                .map_err(|e| {
                                    PyErr::new::<PyTypeError, _>(format!(
                                        "Error al agregar el estudiante a la hiperarista '{}': {}",
                                        hyperedge_name, e
                                    ))
                                })?;
                        }
                    }
                }
                DataType::List(list_type) => {
                    if **list_type != DataType::UInt8 {
                        return Err(PyErr::new::<PyTypeError, _>(format!(
                            "Error al procesar la columna '{}': se esperaba una lista de UInt8",
                            name
                        )));
                    }
                    for (student_id, value) in column.list().unwrap().into_iter().enumerate() {
                        if let Some(value) = value {
                            for item in value.u8().unwrap().into_no_null_iter() {
                                let hyperedge_name = format!("{}_{}", name, item);
                                hypergraph.add_student_to_hyperedge(&hyperedge_name, student_id).map_err(|e| {
                                    PyErr::new::<PyTypeError, _>(format!(
                                        "Error al agregar el estudiante a la hiperarista '{}': {}",
                                        hyperedge_name, e
                                    ))
                                })?;
                            }
                        }
                    }
                }
                _ => {
                    return Err(PyErr::new::<PyTypeError, _>(format!(
                        "Error al procesar la columna {}",
                        name
                    )));
                }
            }
        }

        hypergraph.save_to_file(&output_file).map_err(|e| {
            PyErr::new::<PyTypeError, _>(format!(
                "Error al guardar el hypergraph en el archivo: {}",
                e
            ))
        })?;

        return Ok(());
    }

    #[pyclass]
    pub struct GeneticAlgorithm {
        population_size: usize,
        generations: usize,
        spins: usize,
        elites: usize,
        mutation_rate: u8,
        crossover_rate: u8,
        log_file_path: Option<String>,
    }

    #[pymethods]
    impl GeneticAlgorithm {
        #[new]
        pub fn new(
            population_size: usize,
            generations: usize,
            spins: usize,
            elites: usize,
            mutation_rate: f32,
            crossover_rate: f32,
            log_file_path: Option<String>,
        ) -> Self {
            if mutation_rate < 0.1 || mutation_rate > 1.0 {
                panic!("La tasa de mutación debe estar entre 0.1 y 1");
            }

            if crossover_rate < 0.1 || crossover_rate > 1.0 {
                panic!("La tasa de cruce debe estar entre 0.1 y 1");
            }
            return GeneticAlgorithm {
                population_size,
                spins,
                elites,
                generations: generations + 1, // Sum 1 to include the last generation in the loop
                mutation_rate : (mutation_rate * 100.0) as u8,
                crossover_rate : (crossover_rate * 100.0) as u8,
                log_file_path,
            };
        }

        pub fn run(&self, num_groups: usize, input_file: String) -> Vec<Vec<usize>> {
            const CONVERGENCE_THRESHOLD: f64 = 6.6;
            const MAX_NO_CHANGE_GENERATIONS: usize = 1000;
            const MAX_LIFES: u8 = 3;

            let hg_bar = ProgressBar::new_spinner();
            hg_bar.set_message("[1] Cargando el hipergrafo de características...");
            hg_bar.enable_steady_tick(std::time::Duration::from_millis(100));
            let hypergraph = load_hypergraph_from_file(&input_file);
            hg_bar.println(format!(
                "{} Hipergrafo cargado exitosamente.",
                style("[INFO]").bold().blue()
            ));
            hg_bar.finish_and_clear();

            // Create the initial population of individuals in parallel
            let pop_bar = ProgressBar::new_spinner();
            pop_bar.set_message("[2] Creando la población inicial...");
            pop_bar.enable_steady_tick(std::time::Duration::from_millis(100));
            let mut population =
                create_initial_population(self.population_size, num_groups, &hypergraph);
            pop_bar.println(format!(
                "{} Población inicial creada exitosamente.",
                style("[INFO]").bold().blue()
            ));
            pop_bar.finish_and_clear();

            log(
                format!(
                    "Población inicial creada con {} individuos",
                    self.population_size
                ),
                &self.log_file_path,
                false,
            );

            let running_bar = ProgressBar::new((self.generations - 1) as u64);
            running_bar.set_style(
                ProgressStyle::with_template(&(format!("{}: ", style("Explorando").blue()) + " [{wide_bar:.white}] Generación: {pos}/{len} | {msg} | Tiempo transcurrido: {elapsed_precise}"))
                .unwrap_or(ProgressStyle::default_bar())
                .progress_chars("##-")
            );
            running_bar.println(format!(
                "[3] Ejecutando algoritmo genético con {} individuos y {} generaciones",
                self.population_size,
                self.generations - 1
            ));

            let mut best_fitness = f64::EPSILON; // Initialize best fitness to the minimum possible value
            let mut change_counter = 1; // Counter to halt if the best fitness doesn't change for 1000 generations
            let mut used_lifes = 0; // Lifes to re-initialize the population if it gets stuck
            let mut last_incident_gen = 0; // Last generation where population was re-initialized due to stagnation
            for generation in 0..self.generations {
                running_bar.inc(1);

                let mean_fitness: f64 = population
                    .par_iter()
                    .map(|ind| ind.get_fitness())
                    .sum::<f64>()
                    / population.len() as f64;
                let ratio = mean_fitness / best_fitness;

                running_bar.set_message(format!(
                    "Mejor fitness: {:.4} | Tasa de convergencia: {:.2}",
                    best_fitness, ratio
                ));

                if population[0].get_fitness() > best_fitness {
                    best_fitness = population[0].get_fitness();
                    change_counter = 1; // Reset the counter if the best fitness improves
                } else {
                    change_counter += 1; // Increment the counter if the best fitness doesn't improve
                }

                if ratio >= 0.95 && used_lifes < MAX_LIFES && change_counter > 500 {
                    used_lifes += 1;
                    change_counter = 1; // Reset the counter after re-initializing the population
                    last_incident_gen = generation; // Update the last incident generation
                    running_bar.println(format!(" {} La población se está estancando, re-inicializando | Intento de salvación {}", 
                        style("[WARNING]").bold().yellow(), used_lifes));

                    let begining_purge_idx = ((1f64 - 0.3 * (used_lifes as f64))* population.len() as f64).floor() as usize;
                    for i in begining_purge_idx..population.len() {
                        population[i] = Individual::new(num_groups, &hypergraph);
                    }
                } else if used_lifes > 0 && last_incident_gen + 750 < generation && ratio < 0.95
                {
                    running_bar.println(format!(
                        " {} Ha aumentado la varianza de la población",
                        style("[INFO]").bold().blue()
                    ));
                    used_lifes = 0; // Reset lifes if the population variance improves after re-initialization
                }
                else if used_lifes == MAX_LIFES && change_counter > MAX_NO_CHANGE_GENERATIONS {
                    running_bar.println(format!(
                        " {} La población se ha estancado, terminando la ejecución",
                        style("[FORCED]").bold().magenta()
                    ));
                    running_bar.abandon();
                    break;
                }

                log(
                    format!("{},{},{}", generation, population[0].get_fitness(), ratio),
                    &self.log_file_path,
                    false,
                );

                if best_fitness >= CONVERGENCE_THRESHOLD {
                    running_bar.println(format!(
                    " {} El algoritmo genético alcanzó un valor de convergencia en {} segundos",
                    style("[ÉXITO]").bold().green(),
                    running_bar.elapsed().as_secs_f64()
                ));
                    running_bar.abandon();
                    break;
                }

                // In parallel, create a new population by selecting parents, performing crossover and mutation
                population = create_new_population(self, &mut population, &hypergraph);
            }

            if !running_bar.is_finished() {
                running_bar.println(format!(
                    " {} El algoritmo genético alcanzó su máximo de generaciones",
                    style("[ÉXITO]").bold().green()
                ));
                running_bar.finish();
            }

            log(
                format!(
                    "Solución encontrada en: {} segundos",
                    running_bar.elapsed().as_secs_f64()
                ),
                &self.log_file_path,
                false,
            );

            // Return the solution of the best individual in the final population
            let best_individual = population
                .par_iter()
                .max_by(|a, b| a.get_fitness().partial_cmp(&b.get_fitness()).unwrap())
                .unwrap();
            return best_individual.get_solution();
        }

        pub fn show_config(&self) {
            let (_, width) = Term::stdout().size_checked().unwrap_or((0, 80));
            let title = " Configuración del Algoritmo Genético ";
            let title_len = title.len();
            let border = "=".repeat((width as usize - title_len) / 2);

            println!(
                "{}{}{}",
                style(&border).green(),
                style(title).bold().green(),
                style(&border).green()
            );
            println!(
                "{}",
                style("[Configuración de la población]").bold().magenta()
            );
            println!(
                "  {} {}: {}",
                style("+").green(),
                style("Tamaño de la población inicial").bold(),
                self.population_size
            );
            println!(
                "  {} {}: {}",
                style("+").green(),
                style("Número de generaciones").bold(),
                self.generations - 1
            );
            println!(
                "  {} {}: {}",
                style("+").green(),
                style("Número de élites").bold(),
                self.elites
            );
            println!(
                "{}",
                style("[Configuración de la evolución]").bold().magenta()
            );
            println!(
                "  {} {}: {}",
                style("+").green(),
                style("Número de spins").bold(),
                self.spins
            );
            println!(
                "  {} {}: {}%",
                style("+").green(),
                style("Tasa de cruce").bold(),
                self.crossover_rate
            );
            println!(
                "  {} {}: {}%",
                style("+").green(),
                style("Tasa de mutación").bold(),
                self.mutation_rate
            );
            println!("{}", style("[Configuración de logging]").bold().magenta());
            match &self.log_file_path {
                Some(path) => println!(
                    " {} {}: {}",
                    style("+").green(),
                    style("Ruta del archivo de log").bold(),
                    path
                ),
                None => println!(
                    "  {} {}",
                    style("+").green(),
                    style("Logging desactivado").bold().dim()
                ),
            }
            println!("{}\n", style("=".repeat(width as usize)).green());
        }
    }

    fn load_hypergraph_from_file(file_path: &str) -> Hypergraph {
        if !Path::new(file_path).exists() {
            panic!("El archivo {} no existe", file_path);
        }

        let hg = Hypergraph::load_from_file(file_path);

        if hg.is_err() {
            panic!(
                "Error al cargar el hipergrafo desde el archivo {}: {}",
                file_path,
                hg.err().unwrap()
            );
        }

        return hg.unwrap();
    }

    fn make_probabilities(population: &Vec<Individual>) -> Vec<f64> {
        let total_fitness: f64 = population.par_iter().map(|ind| ind.get_fitness()).sum();

        // Calculate the cumulative probabilities for each individual based on their fitness
        let mut probabilities = vec![population[0].get_fitness() / total_fitness];

        for i in 1..population.len() {
            probabilities.push(probabilities[i - 1] + population[i].get_fitness() / total_fitness);
        }

        return probabilities;
    }

    fn roulette_wheel_selection(probabilities: &Vec<f64>) -> usize {
        if let Ok(rng) = Uniform::new(0.0, 1.0) {
            let mut index = 0;

            // Generate a uniformly distributed random number between 0 and 1
            let random_value = rng.sample(&mut rand::rng());

            // Looks for the first index where the cumulative probability exceeds the random value
            while index < probabilities.len() && random_value > probabilities[index] {
                index += 1;
            }
            return index;
        } else {
            return 0;
        }
    }

    fn elitism(population: &Vec<Individual>, num_elites: usize) -> Vec<Individual> {
        return population.iter().take(num_elites).cloned().collect();
    }

    fn create_new_population(
        config: &GeneticAlgorithm,
        population: &mut Vec<Individual>,
        hypergraph: &Hypergraph,
    ) -> Vec<Individual> {
        /*
           This function runs in parallel for each spin of the roulette. Each spin generates 4 new individuals
           from 2 parents selected by the roulette,
           performing crossover and mutation in parallel for each group of the individuals.
        */
        let mut new_population = elitism(population, config.elites);

        let probabilities = make_probabilities(population);

        let children = (0..config.spins)
            .into_par_iter()
            .flat_map(|_| {
                // Select two parents using the roulette wheel selection method
                let parent1_idx = roulette_wheel_selection(&probabilities);
                let parent2_idx = roulette_wheel_selection(&probabilities);

                let parent1 = &population[parent1_idx];
                let parent2 = &population[parent2_idx];

                // Perform crossover to create two children from the selected parents
                let crossover_result = parent1.crossover(parent2, config.crossover_rate);
                //let crossover2_result = parent2.crossover(parent1, config.crossover_rate);
                if let Err(e) = crossover_result {
                    panic!("Error en la cruza: {}", e);
                }
                let (mut child1, mut child2) = crossover_result.unwrap();
            
                let mut child3 = child1.mutate(config.mutation_rate).unwrap_or_else(|e|
                {
                    panic!("Error en la mutación del hijo 1: {}", e)
                });
                let mut child4 = child2.mutate(config.mutation_rate).unwrap_or_else(|e| {
                    panic!("Error en la mutación del hijo 2: {}", e)
                });

                // Calculate the fitness of the new individuals
                child1.calculate_fitness(hypergraph);
                child2.calculate_fitness(hypergraph);
                child3.calculate_fitness(hypergraph);
                child4.calculate_fitness(hypergraph);

                if cfg!(debug_assertions) {
                    println!(
                        "Child1 solution: {:?}, fitness: {}",
                        child1.get_solution(),
                        child1.get_fitness()
                    );
                    println!(
                        "Child2 solution: {:?}, fitness: {}",
                        child2.get_solution(),
                        child2.get_fitness()
                    );
                    println!(
                        "Child3 solution: {:?}, fitness: {}",
                        child3.get_solution(),
                        child3.get_fitness()
                    );
                    println!(
                        "Child4 solution: {:?}, fitness: {}",
                        child4.get_solution(),
                        child4.get_fitness()
                    );
                }

                return vec![child1, child2, child3, child4];
            })
            .collect::<Vec<Individual>>();

        // New population is formed by the elites and the children generated in parallel
        new_population.extend(children);
        //Ordena la población por fitness de mayor a menor
        new_population.par_sort_by(|a, b| b.get_fitness().partial_cmp(&a.get_fitness()).unwrap());
        return new_population;
    }

    fn create_initial_population(
        population_size: usize,
        num_groups: usize,
        hypergraph: &Hypergraph,
    ) -> Vec<Individual> {
        let mut population = (0..population_size)
            .map(|_| Individual::new(num_groups, hypergraph))
            .collect::<Vec<Individual>>();
        population.par_sort_by(|a, b| b.get_fitness().partial_cmp(&a.get_fitness()).unwrap());
        return population;
    }
}

#[cfg(test)]
mod tests {
    use crate::data::hypergraph::Hypergraph;
    use crate::ml::genetics::Individual;
    use std::collections::HashSet;
    use std::{assert_ne, println};

    #[test]
    pub fn test_hypergraph_no_students() {
        let mut hg = Hypergraph::new(0);
        assert_eq!(hg.get_student_count(), 0);

        assert!(hg.get_subhypergraph_by_prefix("Unexistent").is_err());
        assert!(hg.add_student_to_hyperedge("MI_5", 3).is_err());
    }

    #[test]
    pub fn test_hypergraph() {
        let mut hg = Hypergraph::new(10);

        assert!(hg.add_student_to_hyperedge("NoPrefix", 5).is_err());
        assert!(hg.add_student_to_hyperedge("MI_3", 4).is_ok());
        assert!(hg.get_subhypergraph_by_prefix("VARK").is_err());
        assert!(hg.get_subhypergraph_by_prefix("MI").is_ok());
    }

    #[test]
    pub fn test_random_group_generator() {
        let individual = Individual::new(3, &Hypergraph::new(30));
        let solution = individual.get_solution();
        assert_eq!(solution.len(), 3); // Check number of groups

        // Check duplicate students
        let mut all_students = HashSet::new();
        for group in solution {
            for student in group {
                assert!(
                    all_students.insert(student),
                    "Duplicate student found: {}",
                    student
                );
            }
        }

        // Check all students are present
        let expected_students: HashSet<usize> = (0..30).collect();
        assert_eq!(all_students, expected_students);

    }

    #[test]
    pub fn test_crossover() {
        let hypergraph = Hypergraph::new(30);
        let parent1 = Individual::new(3, &hypergraph);
        let parent2 = Individual::new(3, &hypergraph);

        parent1.get_solution();
        parent2.get_solution();

        let crossover_result = parent1.crossover(&parent2, 100);
        let (child1, child2) = match crossover_result {
            Ok((child1, child2)) => (child1, child2),
            Err(e) => {
                eprintln!("Error en la cruza: {}", e);
                (parent1.clone(), parent2.clone())
            }
        };

        // Check that the children have the correct number of groups
        assert_eq!(child1.get_solution().len(), 3);
        assert_eq!(child2.get_solution().len(), 3);

        // Check that the children are different from the parents
        assert_ne!(child1.get_solution(), parent1.get_solution());
        assert_ne!(child2.get_solution(), parent2.get_solution());

        // Check that all students are only in one group for child 1
        let mut unseen_students = HashSet::<usize>::from_iter(0..30);
        for child1_group in child1.get_solution() {
            let mut seen_students = HashSet::<usize>::new();
            for student in child1_group {
                assert!(
                    seen_students.insert(student),
                    "Duplicate student found in child 1: {}",
                    student
                );
                assert!(
                    unseen_students.remove(&student),
                    "Student {} is in multiple groups in child 1",
                    student
                );
            }
        }

        assert!(
            unseen_students.is_empty(),
            "Some students are not assigned to any group in child 1: {:?}",
            unseen_students
        );

        unseen_students = HashSet::<usize>::from_iter(0..30);
        // Check that all students are only in one group for child 2
        for child2_group in child2.get_solution() {
            let mut seen_students = HashSet::<usize>::new();
            for student in child2_group {
                assert!(
                    seen_students.insert(student),
                    "Duplicate student found in child 2: {}",
                    student
                );
                assert!(
                    unseen_students.remove(&student),
                    "Student {} is in multiple groups in child 2",
                    student
                );
            }
        }

        assert!(
            unseen_students.is_empty(),
            "Some students are not assigned to any group in child 2: {:?}",
            unseen_students
        );
    }

    #[test]
    pub fn test_mutation() {
        let hypergraph = Hypergraph::new(30);
        let individual = Individual::new(3, &hypergraph);

        let original_solution = individual.get_solution().clone();

        let mutation_result = individual.mutate(100);
        let individual = match mutation_result {
            Ok(individual) => individual,
            Err(e) => {
                eprintln!("Error en la mutación: {}", e);
                individual
            }
        };

        let mutated_solution = individual.get_solution();

        println!("Original solution: {:?}", original_solution);
        println!("Mutated solution: {:?}", mutated_solution);

        // Check that the mutated solution is different from the original solution
        let is_different = original_solution != mutated_solution;
        assert!(
            is_different,
            "The mutated solution is the same as the original solution"
        );

        // Check that the mutated solution has the correct number of groups
        assert_eq!(mutated_solution.len(), 3);

        // Check that all students are only in one group
        let mut unseen_students = HashSet::<usize>::from_iter(0..30);
        for group in mutated_solution {
            let mut seen_students = HashSet::<usize>::new();
            for student in group {
                assert!(
                    seen_students.insert(student),
                    "Duplicate student found after mutation: {}",
                    student
                );
                assert!(
                    unseen_students.remove(&student),
                    "Student {} is in multiple groups after mutation",
                    student
                );
            }
        }

        assert!(
            unseen_students.is_empty(),
            "Some students are not assigned to any group after mutation: {:?}",
            unseen_students
        );
    }
}
