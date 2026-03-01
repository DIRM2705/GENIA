mod group;

use group::Group;
use polars::prelude::*;
pub struct Individual {
    groups: Vec<group::Group>,
    fitness: f32,
}

impl Individual {
    pub fn new(student_ids: Vec<Vec<u32>>, df: LazyFrame) -> Self {
        let mut groups = Vec::new();
        for student_id in student_ids {
            let series = Series::new("IDs".into(), student_id);
            let group = df
                .clone()
                .filter(col("Id").is_in(lit(series), false))
                .collect()
                .unwrap();
            groups.push(Group::new(group));
        }
        return Individual {
            groups,
            fitness: 0.0,
        };
    }

    pub fn calculate_fitness(&mut self) {
        self.fitness = 0.0;
        for group in &self.groups {
            self.fitness += group.calculate_discartability();
        }
    }
}
