mod data;
mod ml;
mod utils;

#[pyo3::pymodule]
mod genia_libs {
    use crate::data::hypergraph::Hypergraph;
    use crate::ml::genetics::Individual;
    use polars::{frame::DataFrame, prelude::*};
    use pyo3::PyErr;
    use pyo3::exceptions::PyTypeError;
    use pyo3::prelude::*;
    use pyo3_polars::PyDataFrame;
    use rand::distr::{Distribution, Uniform};
    use rayon::prelude::*;
    use core::panic;
    use std::path::Path;
    use crate::utils::logging::log;

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
            mutation_rate: u8,
            crossover_rate: u8,
            log_file_path: Option<String>,
        ) -> Self {
            return GeneticAlgorithm {
                population_size,
                spins,
                elites,
                generations : generations + 1, // Sum 1 to include the last generation in the loop
                mutation_rate,
                crossover_rate,
                log_file_path
            };
        }

        pub fn run(&self, num_groups: usize, input_file: String) -> Vec<Vec<usize>> {
            println!("------------------------ALGORITMO GENÉTICO--------------------------------------------");
            let hypergraph = load_hypergraph_from_file(&input_file);

            // Create the initial population of individuals in parallel
            let mut population =
                create_initial_population(self.population_size, num_groups, &hypergraph);

            log(format!("Población inicial creada con {} individuos", self.population_size), self.log_file_path.as_deref());

            let mut best_fitness = 0.0;
            let mut change_counter = 1000; // Counter to halt if the best fitness doesn't change for 1000 generations
            for generation in 0..self.generations {
                //Ordena la población por fitness de mayor a menor
                population.sort_by(|a, b| b.get_fitness().partial_cmp(&a.get_fitness()).unwrap());

                log(format!("Generación {} --- Mejor fitness: {}", generation, population[0].get_fitness()), self.log_file_path.as_deref());
                if population[0].get_fitness() > best_fitness {
                    best_fitness = population[0].get_fitness();
                    change_counter = 1000;
                } else {
                    change_counter -= 1;
                }

                if best_fitness >= 6.6 || change_counter == 0 {
                    log(format!("Convergencia detectada: {}", population[0].get_fitness()), self.log_file_path.as_deref());
                    break;
                }

                // In parallel, create a new population by selecting parents, performing crossover and mutation
                population = create_new_population(self, &population, &hypergraph);
                log (format!("Creada siguiente generación con {} individuos", population.len()), self.log_file_path.as_deref());
            }

            // Return the solution of the best individual in the final population
            let best_individual = population
                .iter()
                .max_by(|a, b| a.get_fitness().partial_cmp(&b.get_fitness()).unwrap())
                .unwrap();
            return best_individual.get_solution();
        }
    }

    fn load_hypergraph_from_file(file_path: &str) -> Hypergraph {
        if !Path::new(file_path).exists() {
            panic!("El archivo {} no existe", file_path);
        }

        let hg = Hypergraph::load_from_file(file_path);

        if hg.is_err() {
            panic!(
                "Error al cargar el hypergraph desde el archivo {}: {}",
                file_path,
                hg.err().unwrap()
            );
        }

        return hg.unwrap();
    }

    fn make_probabilities(population: &Vec<Individual>) -> Vec<f64> {
        let total_fitness: f64 = population.iter().map(|ind| ind.get_fitness()).sum();

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
        return population.iter()
            .take(num_elites)
            .cloned()
            .collect();
    }

    fn create_new_population(
        config: &GeneticAlgorithm,
        population: &Vec<Individual>,
        hypergraph: &Hypergraph,
    ) -> Vec<Individual> {
        /*
           This function runs in parallel for each spin of the roulette. Each spin generates 4 new individuals
           from 2 parents selected by the roulette,
           performing crossover and mutation in parallel for each group of the individuals.
        */

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
                if let Err(e) = crossover_result {
                    panic!("Error en la cruza: {}", e);
                }

                let (mut child1, mut child2) = crossover_result.unwrap();
                // Calculate the fitness of the new individuals
                child1.calculate_fitness(hypergraph);
                child2.calculate_fitness(hypergraph);

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
                }

                // Mutate the new individuals to create two more children
                let mutation_result = child1.mutate(config.mutation_rate);

                if let Err(e) = mutation_result {
                    panic!("Error en la mutación: {}", e);
                }

                let mut child3 = mutation_result.unwrap();

                let mutation_result = child2.mutate(config.mutation_rate);

                if let Err(e) = mutation_result {
                    panic!("Error en la mutación: {}", e);
                }

                let mut child4 = mutation_result.unwrap();

                child3.calculate_fitness(hypergraph);
                child4.calculate_fitness(hypergraph);

                if cfg!(debug_assertions) {
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
        let mut new_population = elitism(population, config.elites);
        new_population.extend(children);
        return new_population;
    }

    fn create_initial_population(
        population_size: usize,
        num_groups: usize,
        hypergraph: &Hypergraph,
    ) -> Vec<Individual> {
        return (0..population_size)
            .map(|_| Individual::new(num_groups, hypergraph))
            .collect::<Vec<Individual>>();
    }
}

#[cfg(test)]
mod tests {
    use std::collections::HashSet;
    use crate::utils::bitmap::BitmapLen;
    use crate::data::hypergraph::Hypergraph;
    use crate::ml::genetics::Individual;

    #[test]
    pub fn test_bitmap_index_out_of_bounds() {
        let mut bitmap = BitmapLen::new(16);
        assert!(bitmap.get_chunk_mut(3).is_err());
        assert!(bitmap.set_bit(19).is_err());
    }

    #[test]
    pub fn test_bitmap_get_chunk_mut() {
        let mut bitmap = BitmapLen::new(16);
        assert!(bitmap.get_chunk_mut(0).is_ok());
        assert!(bitmap.get_chunk_mut(1).is_ok());
    }

    #[test]
    pub fn test_bitmap_set_and_get_bits() {
        let mut bitmap = BitmapLen::new(16);
        assert!(bitmap.set_bit(3).is_ok());
        assert!(bitmap.set_bit(7).is_ok());
        assert!(bitmap.set_bit(15).is_ok());
    }

    #[test]
    pub fn test_hypergraph_no_students()
    {
        let mut hg = Hypergraph::new(0);
        assert_eq!(hg.get_student_count(), 0);

        assert!(hg.get_subhypergraph_by_prefix("Unexistent").is_err());
        assert!(hg.add_student_to_hyperedge("MI_5", 3).is_err());
    }

    #[test]
    pub fn test_hypergraph()
    {
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
            for student  in group {
                assert!(all_students.insert(student), "Duplicate student found: {}", student);
            }
        }
    }

    #[test]
    pub fn test_crossover()
    {
        let hypergraph = Hypergraph::new(10);
        let parent1 = Individual::new(3, &hypergraph);
        let parent2 = Individual::new(3, &hypergraph);

        parent1.get_solution();
        parent2.get_solution();

        let crossover_result = parent1.crossover(&parent2, 50);
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

        // Check that all students are only in one group for child 1
        let mut unseen_students = HashSet::<usize>::from_iter(0..10);
        for child1_group in child1.get_solution() {
            let mut seen_students = HashSet::<usize>::new();
            for student in child1_group {
                assert!(seen_students.insert(student), "Duplicate student found in child 1: {}", student);
                assert!(unseen_students.remove(&student), "Student {} is in multiple groups in child 1", student);
            }
        }

        assert!(unseen_students.is_empty(), "Some students are not assigned to any group in child 1: {:?}", unseen_students);

        unseen_students = HashSet::<usize>::from_iter(0..10);
        // Check that all students are only in one group for child 2
        for child2_group in child2.get_solution() {
            let mut seen_students = HashSet::<usize>::new();
            for student in child2_group {
                assert!(seen_students.insert(student), "Duplicate student found in child 2: {}", student);
                assert!(unseen_students.remove(&student), "Student {} is in multiple groups in child 2", student);
            }
        }

        assert!(unseen_students.is_empty(), "Some students are not assigned to any group in child 2: {:?}", unseen_students);
    }

    #[test]
    pub fn test_mutation() {
        let hypergraph = Hypergraph::new(10);
        let individual = Individual::new(3, &hypergraph);

        let mutation_result = individual.mutate(70);
        let individual = match mutation_result {
            Ok(individual) => individual,
            Err(e) => {
                eprintln!("Error en la mutación: {}", e);
                individual
            }
        };

        let mutated_solution = individual.get_solution();

        // Check that the mutated solution has the correct number of groups
        assert_eq!(mutated_solution.len(), 3);

        // Check that all students are only in one group
        let mut unseen_students = HashSet::<usize>::from_iter(0..10);
        for group in mutated_solution {
            let mut seen_students = HashSet::<usize>::new();
            for student in group {
                assert!(seen_students.insert(student), "Duplicate student found after mutation: {}", student);
                assert!(unseen_students.remove(&student), "Student {} is in multiple groups after mutation", student);
            }
        }

        assert!(unseen_students.is_empty(), "Some students are not assigned to any group after mutation: {:?}", unseen_students);
    }
}
