use std::println;

use crate::data::hypergraph::Hypergraph;
use crate::ml::group::Group;
use crate::utils::bitmap::BitmapLen;
use rand::distr::{Distribution, Uniform};
use rand::rng;
use rand::seq::SliceRandom;
use rayon::prelude::*;

#[derive(Clone)]
pub struct Individual {
    solution: Vec<usize>,
    min_students_per_group: usize,
    group_amount: usize,
    residual_students: usize,
    base_indices: Vec<usize>,
    fitness: f64,
}

impl Individual {
    pub fn new(group_amount: usize, hypergraph: &Hypergraph) -> Self {
        let mut individual = Individual {
            solution: get_random_permutation(hypergraph.get_student_count()),
            min_students_per_group: hypergraph.get_student_count() / group_amount,
            residual_students: hypergraph.get_student_count() % group_amount,
            group_amount,
            base_indices: Vec::new(),
            fitness: 0.0,
        };

        individual.make_base_indices();
        individual.calculate_fitness(hypergraph);
        return individual;
    }

    fn make_base_indices(&mut self) {
        /*
         * Creates a vector of base indices for each group in the solution.
         * Each base index represents the starting index of a group in the solution vector.
         */

        self.base_indices = vec![0];
        for group_idx in 1..self.group_amount {
            let base_idx = self.base_indices[group_idx - 1]
                + self.min_students_per_group
                + if group_idx <= self.residual_students {
                    1
                } else {
                    0
                };
            self.base_indices.push(base_idx);
            if cfg!(debug_assertions) {
                println!("Base index for group {}: {}", group_idx, base_idx);
            }
        }
        self.base_indices.push(self.solution.len());
    }

    pub fn get_fitness(&self) -> f64 {
        return self.fitness;
    }

    pub fn get_solution(&self) -> Vec<Vec<usize>> {
        /*
         * Returns the solution of the individual as a vector of groups,
         * where each group is represented as a vector of student indices.
         */

        let mut groups = Vec::new();

        for group_idx in 0..self.group_amount {
            let mut group = Vec::new();
            let mut idx = self.base_indices[group_idx];
            while idx < self.base_indices[group_idx + 1] {
                group.push(self.solution[idx]);
                idx += 1;
            }
            groups.push(group);
        }
        return groups;
    }

    pub fn calculate_fitness(&mut self, hypergraph: &Hypergraph) {
        /*
         * Calculates individual's fitness as the MAE of the discardabilities of each group,
         * where the discardability of a group is calculated from the hypergraph.
         * This function is parallelized for each group.
         */

        let fitness_sum: f64 = self
            .get_groups()
            .par_iter()
            .map(|group| 11f64 - group.calculate_discardability(hypergraph))
            .sum();

        self.fitness = fitness_sum / self.group_amount as f64;
    }

    fn get_groups(&self) -> Vec<Group> {
        /*
         * Creates groups from the individual's solution.
         * Each group contains a set of students.
         */

        return (0..self.group_amount)
            .into_par_iter()
            .map(|group_idx| {
                let mut bitmap = BitmapLen::new(self.solution.len());

                let mut idx = self.base_indices[group_idx];
                while idx < self.base_indices[group_idx + 1] {
                    bitmap.set_bit(self.solution[idx]).unwrap();
                    idx += 1;
                }
                return Group::new(bitmap);
            })
            .collect::<Vec<Group>>();
    }

    pub fn crossover(
        &self,
        other: &Individual,
        crossover_rate: u8,
    ) -> Result<(Individual, Individual), GeneticAlgorithmError> {
        /*
         * Performs crossover between two individuals, generating two new individuals (children).
         * The crossover is done by the PMX method (Partially Mapped Crossover)
         * This function is parallelized for each group.
         */

        if crossover_rate < 1 || crossover_rate > 100 {
            return Err(GeneticAlgorithmError::InvalidCrossoverRate);
        }

        let generator = Uniform::new_inclusive(0, 100)?;

        /*
         * The crossover determines whether to perform crossover or not based on the crossover rate.
         * If the random value is less than the crossover rate,
         * the crossover is performed, otherwise the parents are returned as children.
         */
        
        if generator.sample(&mut rng()) >= crossover_rate {
            if cfg!(debug_assertions) {
                println!("Crossover not performed, returning parents as children");
            }
            return Ok((self.clone(), other.clone()));
        }

        let student_count = self.solution.len();
        //Generate two crossover points
        let limit_generator = Uniform::new(1, student_count / 2)?;
        // First crossover point must be in the first half of the solution
        let cx_point1 = limit_generator.sample(&mut rng());
        // Second crossover point must be after the first crossover point
        let cx_point2 = limit_generator.sample(&mut rng()) + cx_point1;

        // Init the offspring solutions
        let mut offspring1 = Individual {
            solution: vec![0; student_count],
            min_students_per_group: self.min_students_per_group,
            residual_students: self.residual_students,
            group_amount: self.group_amount,
            base_indices: self.base_indices.clone(),
            fitness: 0.0,
        };

        let mut offspring2 = Individual {
            solution: vec![0; student_count],
            min_students_per_group: self.min_students_per_group,
            residual_students: self.residual_students,
            group_amount: self.group_amount,
            base_indices: self.base_indices.clone(),
            fitness: 0.0,
        };

        if cfg!(debug_assertions) {
            println!(
                "Crossover points: {} - {}",
                cx_point1, cx_point2
            );

            println!("Parent 1: {:?}", self.solution);
            println!("Parent 2: {:?}", other.solution);
        }

        let mut offspring1_bitmap = BitmapLen::new(student_count);
        let mut offspring2_bitmap = BitmapLen::new(student_count);

        // Swap the crossover segment from the parents to the offspring
        for i in cx_point1..cx_point2 {
            offspring1.solution[i] = other.solution[i];
            offspring2.solution[i] = self.solution[i];

            offspring1_bitmap.set_bit(other.solution[i])?;
            offspring2_bitmap.set_bit(self.solution[i])?;
        }

        let offsprings_result = rayon::join(
            || {
                for i in 0..cx_point1 {
                    let student_idx = self.solution[i];
                    if !offspring1_bitmap.get_bit(student_idx)? {
                        offspring1.solution[i] = student_idx;
                        continue;
                    }
                    offspring1.partial_map(student_idx, i, self, cx_point1, cx_point2)?;
                }

                for i in cx_point2..student_count {
                    let student_idx = self.solution[i];
                    if !offspring1_bitmap.get_bit(student_idx)? {
                        offspring1.solution[i] = student_idx;
                        continue;
                    }
                    offspring1.partial_map(student_idx, i, self, cx_point1, cx_point2)?;
                }

                if cfg!(debug_assertions) {
                    println!(
                        "Offspring 1 solution: {:?}", offspring1.solution
                    );
                }

                return Ok(());
            },
            || {
                for i in 0..cx_point1 {
                    let student_idx = other.solution[i];
                    if !offspring2_bitmap.get_bit(student_idx)? {
                        offspring2.solution[i] = student_idx;
                        continue;
                    }
                    offspring2.partial_map(student_idx, i, other, cx_point1, cx_point2)?;
                }

                for i in cx_point2..student_count {
                    let student_idx = other.solution[i];
                    if !offspring2_bitmap.get_bit(student_idx)? {
                        offspring2.solution[i] = student_idx;
                        continue;
                    }
                    offspring2.partial_map(student_idx, i, other, cx_point1, cx_point2)?;
                }

                if cfg!(debug_assertions) {
                    println!(
                        "Offspring 2 solution: {:?}", offspring2.solution
                    );
                }

                return Ok(());
            },
        );

        return match offsprings_result {
            (Ok(_), Ok(_)) => Ok((offspring1, offspring2)),
            (Err(e), _) => Err(e),
            (_, Err(e)) => Err(e),
        };
    }

    fn partial_map(
        &mut self,
        mut student_idx : usize,
        position: usize,
        other: &Individual,
        cx_point1: usize,
        cx_point2: usize,
    ) -> Result<(), GeneticAlgorithmError> {
        // While the student is in offspring
        while let Some(mapped_idx) = self.get_index_of_student(student_idx, cx_point1, cx_point2) {
            student_idx = other.solution[mapped_idx];
        }

        //Found a student that is not in offspring, set it
        self.solution[position] = student_idx;
        return Ok(());
    }

    pub fn mutate(&self, mutation_rate: u8) -> Result<Individual, GeneticAlgorithmError> {
        /*
         * Mutates the individual changing students between groups according to a mutation rate.
         * This function is parallelized for each group.
         */

        if mutation_rate < 1 || mutation_rate > 100 {
            return Err(GeneticAlgorithmError::InvalidMutationRate);
        }

        let uniform_rng = Uniform::new_inclusive(0, 100)?;

        // Create a new individual as a clone of the current one, to apply the mutations
        let mut new_individual = self.clone();

        for i in 0..self.min_students_per_group {
            // If the gene should be mutated
            if uniform_rng.sample(&mut rng()) < mutation_rate {
                // Swap the student at i position with the student at the next group
                let mut student_idx = new_individual.solution[i];
                for group_idx in 0..self.group_amount - 1 {
                    let idx = new_individual.base_indices[group_idx] + i;
                    let actual_student_idx = new_individual.solution[idx];
                    new_individual.solution[idx] = student_idx;
                    student_idx = actual_student_idx;
                }
                new_individual.solution[i] = student_idx;
            }
        }

        return Ok(new_individual);
    }

    fn get_index_of_student(
        &self,
        student_idx: usize,
        cx_point1: usize,
        cx_point2: usize,
    ) -> Option<usize> {
        /*
         * Returns the index of the student in the solution vector.
         * This function is used to find the index of a student in the solution vector,
         * excluding positions that have not been assigned yet (between the crossover points).
         */

        return (cx_point1..cx_point2)
            .into_par_iter()
            .find_any(|&i| self.solution[i] == student_idx);
    }
}

//Generates a random permutation of the numbers from 0 to n-1
fn get_random_permutation(n: usize) -> Vec<usize> {
    let mut perm: Vec<usize> = (0..n).collect();
    perm.shuffle(&mut rng());
    return perm;
}

#[derive(Debug)]
pub enum GeneticAlgorithmError {
    InvalidCrossoverRate,
    InvalidMutationRate,
    RNGError(rand::distr::uniform::Error),
    BitmapError(crate::utils::bitmap::BitMapError),
}

impl std::fmt::Display for GeneticAlgorithmError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        return match self {
            GeneticAlgorithmError::InvalidCrossoverRate => write!(f, "La tasa de crossover debe estar entre 1 y 100"),
            GeneticAlgorithmError::InvalidMutationRate => write!(f, "La tasa de mutación debe estar entre 1 y 100"),
            GeneticAlgorithmError::RNGError(e) => write!(f, "Error en el generador de números aleatorios: {}", e),
            GeneticAlgorithmError::BitmapError(e) => write!(f, "Error en el bitmap: {}", e),
        };
    }
}

impl From<rand::distr::uniform::Error> for GeneticAlgorithmError {
    fn from(err: rand::distr::uniform::Error) -> GeneticAlgorithmError {
        return GeneticAlgorithmError::RNGError(err);
    }
}

impl From<crate::utils::bitmap::BitMapError> for GeneticAlgorithmError {
    fn from(err: crate::utils::bitmap::BitMapError) -> GeneticAlgorithmError {
        return GeneticAlgorithmError::BitmapError(err);
    }
}
