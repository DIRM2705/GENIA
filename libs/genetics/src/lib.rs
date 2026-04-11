mod group;
pub mod student_data;

use group::Group;
use student_data::StudentsData;
use rand::rng;
use rand::seq::SliceRandom;
use rand::distr::{Distribution, Uniform};
use std::thread;

pub struct Individual {
    groups: Vec<Group>,
    fitness: f32,
}

impl Individual {
    pub fn new(student_data: &StudentsData, group_amount: usize) -> Self
    {
        let mut individual = Individual {
            groups: Vec::new(),
            fitness: 0.0,
        };

        individual.generate_random_groups(student_data.get_student_count() as u32, group_amount);
        individual.calculate_fitness(&student_data);
        return individual;
    }

    pub fn get_fitness(&self) -> f32 {
        return self.fitness;
    }

    fn generate_random_groups(&mut self, student_total: u32, group_amount: usize){
        let queue = get_random_permutation(student_total);
        let max_students_per_group = (student_total as f32 / group_amount as f32).ceil() as usize;
        let mut i: usize = 0;
        let mut group = Vec::new();
        while i < queue.len() {
            //The group is full
            if group.len() > max_students_per_group {
                self.groups.push(Group::new(group));
                group = Vec::new();
            }

            //Adds the student to the group
            group.push(queue[i] as usize);
            i += 1;
        }
    }

    pub fn calculate_fitness(&mut self, student_data: &StudentsData)
    {
        //Creates a scoped thread to calculate the fitness of each group in parallel
        return thread::scope(|t| {
            //Creates a vector to store the handles of the threads
            let mut handles = Vec::new();

            //Spawns a thread for each group to calculate its discartability
            for group in &self.groups {
                let data = student_data.get_mi_rows(group.get_students());
                let vark_data = student_data.get_vark_rows(group.get_students());
                let inner_handle = t.spawn(move || {
                    return group.calculate_discartability(data, vark_data);
                });
                handles.push(inner_handle);
            }

            //Waits for all threads to finish and sums the discartability of each group to calculate the fitness of the individual
            let mut fitness = 0.0f32;
            for handle in handles {
                fitness += handle.join().unwrap();
            }
            self.fitness = fitness;
        });
    }

    pub fn crossover(&mut self, other: &mut Individual, crossover_rate: u8) -> (Individual, Individual) {
        //Creates a new individual by crossing over the groups of the two parents
        let generator = Uniform::new_inclusive(0, 100).unwrap();
        let mut child1 = Vec::new();
        let mut child2 = Vec::new();

        for group_idx in 0..self.groups.len() {
            let mut new_group1 = Vec::new();
            let mut new_group2 = Vec::new();

            let group1 = self.groups[group_idx].get_students();
            let group2 = other.groups[group_idx].get_students();

            for student_idx in 0..group1.len() {
                let student1 = group1[student_idx];
                let student2 = group2[student_idx];

                if generator.sample(&mut rand::rng()) <= crossover_rate {
                    //This student is swapped between the two children
                    new_group1.push(student2);
                    new_group2.push(student1);
                }
                else
                {
                    //This student is not swapped between the two children
                    new_group1.push(student1);
                    new_group2.push(student2);
                }
            }

            child1.push(Group::new(new_group1));
            child2.push(Group::new(new_group2));
        }

        return (Individual { groups: child1, fitness: 0.0 }, Individual { groups: child2, fitness: 0.0 });
    }

    /*pub fn select_student(&self) -> u32 {
        //Selects a student randomly from the individual
        let mut rng = rand::thread_rng();
        let group_idx = rng.gen_range(0..self.groups.len());
        let student_idx = rng.gen_range(0..self.groups[group_idx].students.len());
        return self.groups[group_idx].students[student_idx];
    }*/
}

fn get_random_permutation(n: u32) -> Vec<u32> {
    let mut perm: Vec<u32> = (0..n).collect();
    perm.shuffle(&mut rng());
    return perm;
}
