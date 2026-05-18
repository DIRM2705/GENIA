mod data;
mod ml;
mod utils;

#[pyo3::pymodule]
mod transformations
{
    use numpy::PyArray2;
    use pyo3::prelude::*;

}

#[pyo3::pymodule]
mod ml_genetics {
    use crate::data::hypergraph::Hypergraph;
    use crate::ml::genetics::Individual;
    use pyo3::prelude::*;
    use std::path::Path;
    use rayon::prelude::*;
    use rand::distr::{Distribution, Uniform};

    #[pyclass]
    pub struct GeneticAlgorithm {
        population_size: usize,
        generations: usize,
        spins: usize,
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
            mutation_rate: u8,
            crossover_rate: u8,
        ) -> Self {
            return GeneticAlgorithm {
                population_size,
                spins,
                generations,
                mutation_rate,
                crossover_rate,
            };
        }

        pub fn run(&self, num_groups: usize) {
            let hypergraph = load_hypergraph_from_file();

            // Genera la población inicial de individuos
            let mut population =
                create_initial_population(self.population_size, num_groups, &hypergraph);

            for _ in 0..self.generations {
                //En paralelo realiza la selección, crossover y mutación para generar la nueva población
                population =
                    create_new_population(self, &population, &hypergraph);
            }
        }
    }

    fn load_hypergraph_from_file() -> Hypergraph {

        if !Path::new("characteristics.hg").exists() {
            panic!("El archivo characteristics.hg no existe");
        }

        if let Ok(hypergraph) = Hypergraph::load_from_file("characteristics.hg") {
            return hypergraph;
        }
        else {
            panic!("Error al cargar el hypergraph desde el archivo characteristics.hg");
        }
    }

    fn make_probabilities(population: &Vec<Individual>) -> Vec<f64> {
        // Calcula la suma total de los fitness de todos los individuos en la población
        let total_fitness: f64 = population.iter().map(|ind| ind.get_fitness()).sum();

        // Calcula la probabilidad acumulada para cada individuo en la población
        let mut probabilities = vec![population[0].get_fitness() / total_fitness];

        let cumulative_prob = (1..population.len()).into_par_iter().map(|i| {
            (population[i].get_fitness() / total_fitness) + probabilities[i - 1]
        }).collect::<Vec<f64>>();

        probabilities.extend(cumulative_prob);
        
        return probabilities;
    }

    fn roulette_wheel_selection(probabilities: &Vec<f64>) -> usize {
        if let Ok(rng) = Uniform::new(0.0, 1.0){
            let mut index = 0;

            // Genera un número aleatorio entre 0 y 1 uniformemente distribuido
            let random_value = rng.sample(&mut rand::rng());

            // Busca al primer individuo cuya probabilidad acumulada sea mayor que el número aleatorio generado
            while index < probabilities.len() && random_value > probabilities[index] {
                index += 1;
            }
            return index;
        }
        else {
            return 0;
        }
    }

    fn create_new_population(
        config: &GeneticAlgorithm,
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
                child1.check_constraints();
                child2.check_constraints();

                //Calcula fitness de los nuevos individuos
                child1.calculate_fitness(hypergraph);
                child2.calculate_fitness(hypergraph);

                // Aplica mutación y crea 2 individuos más
                let child3 = child1.mutate(config.mutation_rate);
                let child4 = child2.mutate(config.mutation_rate);

                return vec![child1, child2, child3, child4];
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
