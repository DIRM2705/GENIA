use numpy::ndarray::{ArrayView2};
use crate::student_data::StudentsData;

const CHRONOTYPE_IDX : usize = 0;
const AM_IDX : usize = 1;
const CM_IDX : usize = 2;
const RM_IDX : usize = 3;
const BE_IDX : usize = 4;
const EE_IDX : usize = 5;
const CE_IDX : usize = 6;

const DELTA_CALCULATIONS : [usize; 4] = [CHRONOTYPE_IDX, RM_IDX, CE_IDX, EE_IDX];
const EPSILON_CALCULATIONS : [usize; 3] = [AM_IDX, CM_IDX, BE_IDX];
pub struct Group {
   students: Vec<usize>
}

impl Group {
    pub fn new(students : Vec<usize>) -> Self {
        return Group {students};
    }

    pub fn len(&self) -> usize {
        return self.students.len();
    }

    pub fn get_students(&self) -> &Vec<usize> {
        return &self.students;
    }

    pub fn get_students_mut(&mut self) -> &mut Vec<usize> {
        return &mut self.students;
    }

    pub fn calculate_discartability(&self, mi_data: Vec<Vec<u8>>, vark_data: Vec<Vec<u8>>) -> f32 {
        let mi_weight = self.calculate_mi_weight(mi_data);
        let vark_weight = self.calculate_vark_weight(vark_data);
        //let motivations_weight = self.calculate_motivations_weight();
        //let engagement_weight = self.calculate_engagement_weight();

        println!(
            "MI weight: {}, VARK weight: {}",
            mi_weight, vark_weight
        );

        return mi_weight + vark_weight;
    }

    fn calculate_mi_weight(&self, students_data: Vec<Vec<u8>>) -> f32 {
        let mut frecuencies = vec![0.0; 9 as usize];
        for student in students_data
        {
            for i in 0..student.len() {
                let class = student[i];
                frecuencies[class as usize] += 1.0;
            }
            drop(student);
        }

        //Calculates the distance from 0.5 which represents an equilibritated distribution of characteristics
        let epsilon_value = (0.5 - calculate_entropy(&frecuencies)).abs();
        return epsilon_value; // Return the calculated MI weight
    }

    fn calculate_vark_weight(&self, students_data: Vec<Vec<u8>>) -> f32 {
        let mut frecuencies = vec![0.0; 4 as usize];
        for student in students_data {
            for i in 0..student.len() {
                let class = student[i];
                frecuencies[class as usize] += 1.0;
            }
            drop(student);
        }
        //Calculates the distance from 0.5 which represents an equilibritated distribution of characteristics
        let epsilon_value = (0.5 - calculate_entropy(&frecuencies)).abs();
        return epsilon_value; // Return the calculated VARK weight
    }
}

fn calculate_entropy(frecuencies : &Vec<f32>) -> f32 {
    let total : f32 = frecuencies.iter().sum();

    let mut entropy = 0.0;

    for i in 0..frecuencies.len() {
        let p_i: f32 = frecuencies[i] / total; // Calculate the probability for the i-th characteristic

        if p_i > 0.0 {
            entropy -= p_i * p_i.log2();
        }
    }

    return entropy/total.log2(); // Normalize the entropy to be between 0 and 1
}