//! This module provides a Python interface for optimizing algorithms needed by the GENIA project
//! 
//! # Features
//! - Hypergraph creation from a Polars DataFrame
//! - Genetic Algorithm implementation for optimization
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
    use std::{path::Path, unreachable};

    /// Creates a hypergraph from a Polars DataFrame and saves it to a specified output file.
    ///
    /// # Arguments
    /// 
    /// * `py_df` - The input Polars DataFrame
    /// * `output_file` - The path to the output file where the hypergraph will be saved
    /// # Returns
    /// 
    /// * `Ok(())` if the hypergraph is created and saved successfully
    /// * `Err(PyErr)` if an error occurs during the process
    #[pyfunction]
    fn hypergraph_from_dataframe(py_df: PyDataFrame, output_file: String) -> PyResult<()> {
        // Validate path
        let path = Path::new(&output_file);
        let parent_dir = path.parent().ok_or_else(|| {
            PyErr::new::<PyTypeError, _>(format!(
                "No se pudo obtener el directorio padre del archivo {}",
                output_file
            ))
        })?;

        if !parent_dir.exists() {
            return Err(PyErr::new::<PyTypeError, _>(format!(
                "El directorio padre del archivo {} no existe",
                output_file
            )));
        }

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

    /// A Python class representing a Genetic Algorithm for optimization.
    /// 
    /// # Attributes
    /// 
    /// * `population_size` - The number of individuals in the population
    /// * `generations` - The number of generations to run the algorithm
    /// * `spins` - The number of spins to perform in each generation
    /// * `elites` - The number of elite individuals to keep in each generation
    /// * `mutation_rate` - The rate at which mutations occur
    /// * `crossover_rate` - The rate at which crossovers occur
    /// * `log_file_path` - The path to the log file
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
        /// Creates a new instance of the GeneticAlgorithm class.
        ///
        /// # Arguments
        /// * `population_size` - The number of individuals in the population
        /// * `generations` - The number of generations to run the algorithm
        /// * `spins` - The number of spins to perform in each generation
        /// * `elites` - The number of elite individuals to keep in each generation
        /// * `crossover_rate` - The rate at which crossovers occur
        /// * `log_file_path` - The path to the log file
        /// 
        /// # Panics
        /// 
        /// * If the crossover rate is not between 0.1 and 1.0
        #[new]
        pub fn new(
            population_size: usize,
            generations: usize,
            spins: usize,
            elites: usize,
            crossover_rate: f32,
            log_file_path: Option<String>,
        ) -> Self {
            if crossover_rate < 0.1 || crossover_rate > 1.0 {
                panic!("La tasa de cruce debe estar entre 0.1 y 1");
            }
            return GeneticAlgorithm {
                population_size,
                spins,
                elites,
                generations: generations + 1, // Sum 1 to include the last generation in the loop
                mutation_rate: 25u8,
                crossover_rate: (crossover_rate * 100.0) as u8,
                log_file_path,
            };
        }

        /// Runs the genetic algorithm on a hypergraph loaded from a specified input file.
        /// 
        /// # Arguments
        /// 
        /// * `num_groups` - The number of groups to form in the solution
        /// * `input_file` - The path to the input file containing the hypergraph
        ///
        /// # Returns
        ///
        /// * `Vec<Vec<usize>>` - A vector of vectors representing each group and the student IDs assigned to that group in the best solution found
        /// 
        /// # Panics
        /// 
        /// * If the input file does not exist or cannot be read.
        /// * If the hypergraph cannot be loaded from the input file.
        pub fn run(&self, num_groups: usize, input_file: String) -> Vec<Vec<usize>> {
            const CONVERGENCE_THRESHOLD: f64 = 6.6;
            const MAX_NO_CHANGE_GENERATIONS: usize = 1000;
            const MAX_LIFES: u8 = 3;

            let hg_bar = ProgressBar::new_spinner();
            hg_bar.set_message("[1] Cargando el hipergrafo de características...");
            hg_bar.enable_steady_tick(std::time::Duration::from_millis(100));
            let hypergraph_result = Hypergraph::load_from_file(&input_file);
            if let Err(e) = hypergraph_result {
                hg_bar.println(format!(
                    "{} Error al cargar el hipergrafo: {}",
                    style("[ERROR]").bold().red(),
                    e
                ));
                hg_bar.finish_and_clear();
                panic!("Error al cargar el hipergrafo: {}", e);
            }
            let hypergraph = hypergraph_result.unwrap();
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
                ProgressStyle::with_template(&(format!("{}: ", style("Explorando").blue()) + " [{wide_bar:.white}] Generación: {pos}/{len} | Tiempo transcurrido: {elapsed_precise}\n{msg}"))
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
            let mut curr_mutation_rate = self.mutation_rate; // Current mutation rate, which can change during execution
            let mut curr_crossover_rate = self.crossover_rate; // Current crossover rate, which can change during execution
            for generation in 0..self.generations {
                running_bar.inc(1);

                let mean_fitness: f64 = population
                    .par_iter()
                    .map(|ind| ind.get_fitness())
                    .sum::<f64>()
                    / population.len() as f64;
                let ratio = mean_fitness / best_fitness;

                running_bar.set_message(format!(
                    "| Tasa de mutación: {}% | Tasa de cruce: {}% | Mejor fitness: {:.4} | Tasa de convergencia: {:.2}",
                    curr_mutation_rate, curr_crossover_rate, best_fitness, ratio
                ));

                if population[0].get_fitness() > best_fitness {
                    best_fitness = population[0].get_fitness();
                    change_counter = 1; // Reset the counter if the best fitness improves
                } else {
                    change_counter += 1; // Increment the counter if the best fitness doesn't improve
                }

                if ratio > 0.95 && used_lifes < MAX_LIFES && change_counter > 500 {
                    used_lifes += 1;
                    change_counter = 1; // Reset the counter after re-initializing the population
                    last_incident_gen = generation; // Update the last incident generation
                    match used_lifes {
                        1 => {
                            curr_mutation_rate = 75; // Increase mutation rate to 75% to increase diversity in the population
                        }
                        2 => {
                            curr_mutation_rate = 100; // Increase mutation rate to 100% to increase diversity in the population
                            curr_crossover_rate = 40; // Decrease crossover rate to 40% to increase diversity in the population
                        }
                        3 => {
                            running_bar.println(format!(" {} La población se está estancando, re-inicializando | Último intento de salvación", style("[WARNING]").bold().yellow()));
                            let begining_purge_idx =
                                (0.2 * population.len() as f64).floor() as usize;
                            for i in begining_purge_idx..population.len() {
                                population[i] = Individual::new(num_groups, &hypergraph);
                            }
                        }
                        _ => unreachable!(
                            "Se ha alcanzado un número de vidas no esperado: {}",
                            used_lifes
                        ),
                    }
                } else if used_lifes > 0 && last_incident_gen + 750 < generation && ratio <= 0.95 {
                    match used_lifes {
                        1 => {
                            curr_mutation_rate = self.mutation_rate; // Decrease mutation rate back to the original value
                        }
                        2 => {
                            curr_mutation_rate = 75; // Reset mutation rate to 75% to keep some diversity in the population
                            curr_crossover_rate = self.crossover_rate; // Reset crossover rate to the original value
                        }
                        3 => {
                            running_bar.println(format!(
                                " {} La varianza de la población ha mejorado",
                                style("[INFO]").bold().blue()
                            ));
                        }
                        _ => unreachable!(
                            "Se ha alcanzado un número de vidas no esperado: {}",
                            used_lifes
                        ),
                    }
                    used_lifes -= 1; // Reset lifes if the population variance improves after re-initialization
                } else if used_lifes == MAX_LIFES && change_counter > MAX_NO_CHANGE_GENERATIONS {
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
                population = create_new_population(curr_mutation_rate, curr_crossover_rate, self.spins, self.elites, &mut population, &hypergraph);
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

        /// Displays the configuration of the genetic algorithm in a formatted manner.
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

    /// Calculates the probabilities for each individual in the population based on their fitness.
    /// 
    /// # Arguments
    /// 
    /// * `population` - A reference to a vector of individuals in the population
    /// 
    /// # Returns
    /// 
    /// * `Vec<f64>` - A vector of probabilities for each individual in the population
    fn make_probabilities(population: &Vec<Individual>) -> Vec<f64> {
        let total_fitness: f64 = population.par_iter().map(|ind| ind.get_fitness()).sum();

        // Calculate the cumulative probabilities for each individual based on their fitness
        let mut probabilities = vec![population[0].get_fitness() / total_fitness];

        for i in 1..population.len() {
            probabilities.push(probabilities[i - 1] + population[i].get_fitness() / total_fitness);
        }

        return probabilities;
    }

    /// Selects an individual from the population using the roulette wheel selection method.
    ///
    /// # Arguments
    /// 
    /// * `probabilities` - A reference to a vector of cumulative probabilities for each individual in the population
    /// 
    /// # Returns
    /// 
    /// * `usize` - The index of the selected individual
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

    /// Selects the top `num_elites` individuals from the population based on their fitness.
    /// 
    /// # Arguments
    /// 
    /// * `population` - A reference to a vector of individuals in the population
    /// * `num_elites` - The number of elites to select
    /// 
    /// # Returns
    /// 
    /// * `Vec<Individual>` - A vector of the top `num_elites` individuals from the population
    fn elitism(population: &Vec<Individual>, num_elites: usize) -> Vec<Individual> {
        return population.iter().take(num_elites).cloned().collect();
    }

    /// Creates a new population of individuals by selecting parents, performing crossover and mutation.
    ///
    /// # Arguments
    /// 
    /// * `mutation_rate` - The rate of mutation for the new individuals
    /// * `crossover_rate` - The rate of crossover for the new individuals
    /// * `spins` - The number of spins to perform
    /// * `elites` - The number of elites to keep
    /// * `population` - A mutable reference to a vector of individuals in the population
    /// * `hypergraph` - A reference to the hypergraph
    /// 
    /// # Returns
    /// 
    /// * `Vec<Individual>` - A vector of the new individuals in the population
    /// 
    /// # Panics
    ///
    /// This function will panic if the crossover or mutation operations fail for any individual.
    fn create_new_population(
        mutation_rate: u8,
        crossover_rate: u8,
        spins : usize,
        elites : usize,
        population: &mut Vec<Individual>,
        hypergraph: &Hypergraph,
    ) -> Vec<Individual> {
        /*
           This function runs in parallel for each spin of the roulette. Each spin generates 4 new individuals
           from 2 parents selected by the roulette,
           performing crossover and mutation in parallel for each group of the individuals.
        */
        let mut new_population = elitism(population, elites);

        let probabilities = make_probabilities(population);

        let children = (0..spins)
            .into_par_iter()
            .flat_map(|_| {
                // Select two parents using the roulette wheel selection method
                let parent1_idx = roulette_wheel_selection(&probabilities);
                let parent2_idx = roulette_wheel_selection(&probabilities);

                let parent1 = &population[parent1_idx];
                let parent2 = &population[parent2_idx];

                // Perform crossover to create two children from the selected parents
                let (mut child1, mut child2) = parent1
                    .crossover(parent2, crossover_rate)
                    .unwrap_or_else(|e| panic!("Error en la cruza: {}", e));

                child1 = child1
                    .mutate(mutation_rate)
                    .unwrap_or_else(|e| panic!("Error en la mutación del hijo 1: {}", e));

                child2 = child2
                    .mutate(mutation_rate)
                    .unwrap_or_else(|e| panic!("Error en la mutación del hijo 2: {}", e));

                let (mut child3, mut child4) = parent2
                    .crossover(parent1, crossover_rate)
                    .unwrap_or_else(|e| panic!("Error en la cruza: {}", e));

                child3 = child3
                    .mutate(mutation_rate)
                    .unwrap_or_else(|e| panic!("Error en la mutación del hijo 3: {}", e));

                child4 = child4
                    .mutate(mutation_rate)
                    .unwrap_or_else(|e| panic!("Error en la mutación del hijo 4: {}", e));

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

    /// Creates an initial population of individuals.
    /// 
    /// # Arguments
    /// 
    /// * `population_size` - The number of individuals in the population
    /// * `num_groups` - The number of groups to create
    /// * `hypergraph` - A reference to the hypergraph
    /// 
    /// # Returns
    /// 
    /// * `Vec<Individual>` - A vector of the initial individuals in the population
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
    use crate::data::hypergraph::{Hypergraph, HypergraphError};
    use crate::ml::genetics::Individual;

    #[test]
    fn hypergraph_adds_and_reads_subhypergraphs() {
        let mut hypergraph = Hypergraph::new(4);

        hypergraph.add_student_to_hyperedge("math_0", 0).unwrap();
        hypergraph.add_student_to_hyperedge("math_1", 1).unwrap();
        hypergraph.add_student_to_hyperedge("science_0", 2).unwrap();

        let math = hypergraph.get_subhypergraph_by_prefix("math").unwrap();
        assert_eq!(math.len(), 2);
        assert_eq!(math[0].get_id(), "math_0");
        assert_eq!(math[1].get_id(), "math_1");

        let science = hypergraph.get_subhypergraph_by_prefix("science").unwrap();
        assert_eq!(science.len(), 1);
    }

    #[test]
    fn hypergraph_rejects_invalid_names_and_out_of_bounds_students() {
        let mut hypergraph = Hypergraph::new(2);

        let invalid = hypergraph.add_student_to_hyperedge("", 0).unwrap_err();
        assert!(matches!(invalid, HypergraphError::InvalidHyperedgeError));

        let out_of_bounds = hypergraph.add_student_to_hyperedge("math_0", 10).unwrap_err();
        assert!(matches!(out_of_bounds, HypergraphError::StudentOutOfBoundsError(10, 2)));
    }

    #[test]
    fn individual_initialization_produces_valid_solution() {
        let hypergraph = Hypergraph::new(8);
        let individual = Individual::new(3, &hypergraph);

        let solution = individual.get_solution();
        assert_eq!(solution.len(), 3);
        assert!(solution.iter().flatten().all(|student_id| *student_id < hypergraph.get_student_count()));
        assert!(individual.get_fitness().is_finite());
    }
}
