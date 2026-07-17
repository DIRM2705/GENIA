use crate::data::hypergraph::Hypergraph;
use crate::utils::bitmap::BitmapLen;
use crate::utils::math::{balance_metric, homogeneity_metric};
use rayon::prelude::*;

const DELTA_CALCULATIONS: [&'static str; 9] = ["CE", "BE", "EE", "AN", "RN", "Chronotype", "PL", "HS", "CN"];
const REPLACEMENT_CALCULATIONS: [&'static str; 2] = ["MI", "VARK"];

#[derive(Clone)]
pub struct Group {
    students: BitmapLen,  // Bitmap representing the students in the group
    student_count: usize, // Number of students in the group
}

impl Group {
    pub fn new(students: BitmapLen) -> Self {
        return Group {
            students: students.clone(),
            student_count: students.count_ones() as usize,
        };
    }

    pub fn get_students(&self) -> BitmapLen {
        return self.students.clone();
    }

    pub fn calculate_discardability(&self, hypergraph: &Hypergraph) -> f64 {
        /*
         * A group's discardability is calculated from two metrics:
         * - Delta Discardability: Measures the group's homogeneity with respect to delta characteristics.
         * - Replacement Discardability: Measures the group's balance with respect to replacement characteristics.
         * A group's total discardability is the sum of the two metrics above.
         * The calculation of each metric is done in parallel using Rayon to improve performance.
         */

        let calculations = [
            Self::calculate_delta_discardability,
            Self::calculate_replacement_discardability,
        ];

        return calculations
            .into_par_iter()
            .map(|calc| calc(self, hypergraph))
            .sum();
    }

    fn calculate_delta_discardability(&self, hypergraph: &Hypergraph) -> f64 {
        let mut probabilities = Vec::new();
        let mut discardability = 0.0;

        //Calculate the homogeneity of each group with respect to the delta characteristics
        for id in DELTA_CALCULATIONS.iter() {
            if let Ok(subhypergraph) = hypergraph.get_subhypergraph_by_prefix(id) {
                // How many students of the group meet the value of the characteristic
                for hyperedge in subhypergraph {
                    let incident_students =
                        hyperedge.apply_mask(&self.students).count_ones() as f64;

                    // Probability of a student in the group meeting the value x of the characteristic
                    probabilities.push(incident_students / self.student_count as f64);
                }
            } else {
                // If the characteristic is not found, the group is considered completely discardable
                println!("No se encontró el subhipergrafo con prefijo '{}'", id);
                discardability += 1.0;
                continue;
            }

            discardability += homogeneity_metric(&probabilities);
            probabilities.clear();
        }

        return discardability;
    }

    fn calculate_replacement_discardability(&self, hypergraph: &Hypergraph) -> f64 {
        let mut probabilities = Vec::new();
        let mut discardability = 0.0;

        //Calculate the balance of each group with respect to the replacement characteristics
        for id in REPLACEMENT_CALCULATIONS.iter() {
            let mut total_incidences = 0.0;
            if let Ok(subhypergraph) = hypergraph.get_subhypergraph_by_prefix(id) {
                // How many students of the group meet the value of the characteristic
                for hyperedge in subhypergraph {
                    let incident_students =
                        hyperedge.apply_mask(&self.students).count_ones() as f64;
                    total_incidences += incident_students;

                    // Firstly, just store the number of students in the group that meet the value
                    // of the characteristic, to later calculate the probability
                    probabilities.push(incident_students);
                }
            } else {
                // If the characteristic is not found, the group is considered completely discardable
                println!("No se encontró el subhipergrafo con prefijo '{}'", id);
                discardability += 1.0;
                continue;
            }

            for p in probabilities.iter_mut() {
                // Calculate the probability of a student in the group meeting the value x of the characteristic
                *p /= total_incidences;
            }

            discardability += balance_metric(&probabilities, probabilities.len() as f64);
            probabilities.clear();
        }

        return discardability;
    }
}
