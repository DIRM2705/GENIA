mod group;

use group::Group;
use polars::prelude::*;
use std::thread;

pub struct Individual {
    groups: Vec<Group>,
    fitness: f32,
}

impl Individual {
    pub fn new(student_ids: Vec<Vec<u32>>, df: LazyFrame) -> Self {
        let mut groups = Vec::new();
        for student_id in student_ids {
            let group = df
                .clone()
                .filter(col("Id").is_in(lit(Series::from_iter(student_id)).implode(), false))
                .collect()
                .unwrap();

            groups.push(Group::new(group));
        }
        return Individual {
            groups,
            fitness: 0.0,
        };
    }

    pub fn get_fitness(&self) -> f32 {
        return self.fitness;
    }

    pub fn calculate_fitness(&mut self) {
        //Creates a scoped thread to calculate the fitness of each group in parallel
        thread::scope(|t| {
            //Creates a vector to store the handles of the threads
            let mut handles = Vec::new();
        
            //Spawns a thread for each group to calculate its discartability
            for group in &self.groups {
                let inner_handle = t.spawn(move || {
                    return group.calculate_discartability();
                });
                handles.push(inner_handle);
            }

            //Waits for all threads to finish and sums the discartability of each group to calculate the fitness of the individual
            self.fitness = 0.0;
            for handle in handles {
                self.fitness += handle.join().unwrap();
            }
            return self.fitness;
        });
    }
}
