use crate::data::hypergraph::Hypergraph;
use crate::utils::bitmap::BitmapLen;
use crate::utils::math::{balance_metric, homogeneity_metric};
use rayon::prelude::*;

const DELTA_CALCULATIONS: [&'static str; 6] = ["CE", "BE", "EE", "AM", "RM", "Cronotipo"];
const EPSILON_CALCULATIONS: [&'static str; 1] = ["CM"];
const REPLACEMENT_CALCULATIONS: [&'static str; 4] = ["MI1", "MI2", "VARK1", "VARK2"];

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

    pub fn calculate_discartability(&self, hypergraph: &Hypergraph) -> f64 {
        /*
        La discartabilidad de un grupo se calcula a partir de tres métricas:
        - Delta Discartability: Mide la homogeneidad del grupo con respecto a las
                                características delta. Se calcula utilizando una métrica de homogeneidad.
        - Epsilon Discartability: Mide el balance del grupo con respecto a las
                                  características epsilon. Se calcula utilizando una métrica de balance.
        - Replacement Discartability: Mide el balance del grupo con respecto a las
                                      características de reemplazo. Se calcula utilizando una métrica de balance.

        La discartabilidad total del grupo es la suma de las tres métricas anteriores.
        El cálculo de cada métrica se realiza de manera paralela utilizando Rayon para mejorar el rendimiento.
        */

        let calculations = [
            Self::calculate_delta_discartability,
            Self::calculate_epsilon_discartability,
            Self::calculate_replacement_discartability,
        ];

        return calculations
            .into_par_iter()
            .map(|calc| calc(self, hypergraph))
            .sum();
    }

    fn calculate_delta_discartability(&self, hypergraph: &Hypergraph) -> f64 {
        let mut probabilities = Vec::new();
        let mut discartability = 0.0;

        //Calcular la homogeneidad de cada grupo con respecto a las características delta
        for id in DELTA_CALCULATIONS.iter() {
            if let Ok(subhypergraph) = hypergraph.get_subhypergraph_by_prefix(id) {
                // Cuantos estudiantes del grupo cumplen con el valor x de la característica
                for hyperedge in subhypergraph {
                    let incident_students =
                        hyperedge.apply_mask(&self.students).count_ones() as f64;

                    // La probabilidad de que un estudiante del grupo cumpla con el valor x de la característica
                    probabilities.push(incident_students / self.student_count as f64);
                }
            } else {
                println!("No se encontró el subhipergrafo con prefijo '{}'", id);
            }

            discartability += homogeneity_metric(&probabilities);
            probabilities.clear();
        }

        return discartability;
    }

    fn calculate_epsilon_discartability(&self, hypergraph: &Hypergraph) -> f64 {
        let mut probabilities = Vec::new();
        let mut discartability = 0.0;

        //Calcular el balance de cada grupo con respecto a las características epsilon
        for id in EPSILON_CALCULATIONS.iter() {
            if let Ok(subhypergraph) = hypergraph.get_subhypergraph_by_prefix(id) {
                // Cuantos estudiantes del grupo cumplen con el valor x de la característica
                for hyperedge in subhypergraph {
                    let incident_students =
                        hyperedge.apply_mask(&self.students).count_ones() as f64;

                    // La probabilidad de que un estudiante del grupo cumpla con el valor x de la característica
                    probabilities.push(incident_students / self.student_count as f64);
                }
            } else {
                println!("No se encontró el subhipergrafo con prefijo '{}'", id);
            }

            discartability += balance_metric(&probabilities, probabilities.len() as f64);
            probabilities.clear();
        }

        return discartability;
    }

    fn calculate_replacement_discartability(&self, hypergraph: &Hypergraph) -> f64 {
        let mut probabilities = Vec::new();
        let mut discartability = 0.0;

        //Calcular el balance de cada grupo con respecto a las características epsilon
        for id in REPLACEMENT_CALCULATIONS.iter() {
            let mut total_incidences = 0.0;
            if let Ok(subhypergraph) = hypergraph.get_subhypergraph_by_prefix(id) {
                // Cuantos estudiantes del grupo cumplen con el valor x de la característica
                for hyperedge in subhypergraph {
                    let incident_students =
                        hyperedge.apply_mask(&self.students).count_ones() as f64;
                    total_incidences += incident_students;

                    // Por ahora, solo guardar la cantidad de estudiantes que cumplen con el valor x de la característica,
                    // para luego calcular la probabilidad de que un estudiante del grupo cumpla con el valor x de la característica
                    probabilities.push(incident_students);
                }
            } else {
                println!("No se encontró el subhipergrafo con prefijo '{}'", id);
            }

            for p in probabilities.iter_mut() {
                // La probabilidad de que un estudiante del grupo cumpla con el valor x de la característica
                *p /= total_incidences;
            }

            //Todas las caracteristicas de reemplazo son epsilon
            discartability += balance_metric(&probabilities, probabilities.len() as f64);
            probabilities.clear();
        }

        return discartability;
    }
}
