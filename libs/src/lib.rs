mod data;
mod ml;
mod utils;

#[pyo3::pymodule]
mod genia_libs {
    use crate::data::hypergraph::Hypergraph;
    use crate::ml::genetics::Individual;
    use polars::{frame::DataFrame, prelude::*};
    use pyo3::prelude::*;
    use pyo3::PyErr;
    use pyo3::exceptions::PyTypeError;
    use pyo3_polars::PyDataFrame;
    use rand::distr::{Distribution, Uniform};
    use rayon::prelude::*;
    use std::path::Path;

    #[pyfunction]
    fn hypergraph_from_dataframe(py_df: PyDataFrame) -> PyResult<()> {
        // Convierte el DataFrame de Polars a un dataframe de Rust
        let df: DataFrame = py_df.into();
        let mut hypergraph = Hypergraph::new(df.height());

        for column in df.get_columns() {
            let name = column.name().to_string();

            match column.dtype() {
                DataType::UInt8 => 
                {
                    for (student_id, value) in column.u8().unwrap().into_iter().enumerate() {
                        if let Some(value) = value {
                            let hyperedge_name = format!("{}_{}", name, value);
                            hypergraph.add_student_to_hyperedge(&hyperedge_name, student_id).map_err(|e| {
                                PyErr::new::<PyTypeError, _>(format!(
                                    "Error al agregar el estudiante a la hiperarista '{}': {}",
                                    hyperedge_name, e
                                ))
                            })?;
                        }
                    }
                },
                DataType::List(list_type) => 
                {
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
                },
                _ => return Err(PyErr::new::<PyTypeError, _>(format!("Error al procesar la columna {}", name))),
            }
        }

        hypergraph.save_to_file("characteristics.hg").map_err(|e| {
            PyErr::new::<PyTypeError, _>(format!("Error al guardar el hypergraph en el archivo: {}", e))
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
        ) -> Self {
            return GeneticAlgorithm {
                population_size,
                spins,
                elites,
                generations,
                mutation_rate,
                crossover_rate,
            };
        }

        pub fn run(&self, num_groups: usize) ->  Vec<Vec<usize>> {
            let hypergraph = load_hypergraph_from_file();

            // Genera la población inicial de individuos
            let mut population =
                create_initial_population(self.population_size, num_groups, &hypergraph);

            for generation in 0..self.generations {
                //En paralelo realiza la selección, crossover y mutación para generar la nueva población
                population = create_new_population(self, num_groups, &population, &hypergraph);

                if cfg!(debug_assertions) {
                    //Imprime datos de la generación actual para debuggear
                    let mut best_individual = &population[0];
                    println!("Generación {}", generation);
                    for (i, individual) in population.iter().enumerate() {
                        println!("Individuo {}: Fitness = {}", i, individual.get_fitness());
                        if individual.get_fitness() > best_individual.get_fitness() {
                            best_individual = individual;
                        }
                    }
                    // Imprime el fitness del mejor individuo de la población en cada generación
                    println!("Mejor fitness en esta generación: {}", best_individual.get_fitness());
                }
            }
            
            // Devuelve la mejor solución encontrada después de todas las generaciones
            let best_individual = population.iter().max_by(|a, b| a.get_fitness().partial_cmp(&b.get_fitness()).unwrap()).unwrap();
            return best_individual.get_solution();
        }
    }

    fn load_hypergraph_from_file() -> Hypergraph {
        if !Path::new("characteristics.hg").exists() {
            panic!("El archivo characteristics.hg no existe");
        }

        if let Ok(hypergraph) = Hypergraph::load_from_file("characteristics.hg") {
            return hypergraph;
        } else {
            panic!("Error al cargar el hypergraph desde el archivo characteristics.hg");
        }
    }

    fn make_probabilities(population: &Vec<Individual>) -> Vec<f64> {
        // Calcula la suma total de los fitness de todos los individuos en la población
        let total_fitness: f64 = population.iter().map(|ind| ind.get_fitness()).sum();

        // Calcula la probabilidad acumulada para cada individuo en la población
        let mut probabilities = vec![population[0].get_fitness() / total_fitness];

        for i in 1..population.len() {
            probabilities.push(probabilities[i - 1] + population[i].get_fitness() / total_fitness);
        }

        return probabilities;
    }

    fn roulette_wheel_selection(probabilities: &Vec<f64>) -> usize {
        if let Ok(rng) = Uniform::new(0.0, 1.0) {
            let mut index = 0;

            // Genera un número aleatorio entre 0 y 1 uniformemente distribuido
            let random_value = rng.sample(&mut rand::rng());

            // Busca al primer individuo cuya probabilidad acumulada sea mayor que el número aleatorio generado
            while index < probabilities.len() && random_value > probabilities[index] {
                index += 1;
            }
            return index;
        } else {
            return 0;
        }
    }

    fn elitism(population: &Vec<Individual>, num_elites: usize) -> Vec<Individual> {
        let mut elites = population.clone();
        elites.sort_by(|a, b| b.get_fitness().partial_cmp(&a.get_fitness()).unwrap());
        return elites.into_iter().take(num_elites).collect();
    }

    fn create_new_population(
        config: &GeneticAlgorithm,
        num_groups: usize,
        population: &Vec<Individual>,
        hypergraph: &Hypergraph,
    ) -> Vec<Individual> {
        let probabilities = make_probabilities(population);

        /*
           Esta función corre en paralelo por cada spin de la ruleta. Cada spin genera 4 nuevos individuos
           a partir de 2 padres seleccionados por la ruleta,
           realizando crossover y mutación en paralelo para cada grupo de los individuos.
        */
        return (0..config.spins)
            .into_par_iter()
            .flat_map(|_| {
                // Selecciona dos individuos utilizando la selección por ruleta
                let parent1_idx = roulette_wheel_selection(&probabilities);
                let parent2_idx = roulette_wheel_selection(&probabilities);

                let parent1 = &population[parent1_idx];
                let parent2 = &population[parent2_idx];

                // Realiza el crossover entre los dos padres para generar un nuevo individuo
                let (mut child1, mut child2) = parent1.crossover(parent2, config.crossover_rate);

                // Forza el cumplimiento de las restricciones en los nuevos individuos
                child1.check_constraints(num_groups);
                child2.check_constraints(num_groups);

                //Calcula fitness de los nuevos individuos
                child1.calculate_fitness(hypergraph);
                child2.calculate_fitness(hypergraph);

                if cfg!(debug_assertions) {
                    println!("Child1 solution: {:?}, fitness: {}", child1.get_solution(), child1.get_fitness());
                    println!("Child2 solution: {:?}, fitness: {}", child2.get_solution(), child2.get_fitness());
                }

                // Aplica mutación y crea 2 individuos más
                let child3 = child1.mutate(config.mutation_rate);
                let child4 = child2.mutate(config.mutation_rate);

                if cfg!(debug_assertions) {
                    println!("Child3 solution: {:?}, fitness: {}", child3.get_solution(), child3.get_fitness());
                    println!("Child4 solution: {:?}, fitness: {}", child4.get_solution(), child4.get_fitness());
                }

                let mut children = vec![child1, child2, child3, child4];
                children.extend(elitism(population, config.elites));

                return children;
            })
            .collect::<Vec<Individual>>();
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
