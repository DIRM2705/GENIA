use numpy::ndarray::{ArrayViewMut1, Array2};

const CHRONOTYPE_IDX: usize = 0;
const AM_IDX: usize = 1;
const CM_IDX: usize = 2;
const RM_IDX: usize = 3;
const BE_IDX: usize = 4;
const EE_IDX: usize = 5;
const CE_IDX: usize = 6;

const DELTA_CALCULATIONS: [usize; 3] = [RM_IDX, CE_IDX, EE_IDX];
const EPSILON_CALCULATIONS: [usize; 3] = [AM_IDX, CM_IDX, BE_IDX];
pub struct Group {
    students: Vec<usize>,
}

impl Group {
    pub fn new(students: Vec<usize>) -> Self {
        return Group { students };
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

    pub fn calculate_discartability(
        &self,
        mut student_data: Vec<Vec<f64>>,
        mi_data: Vec<Vec<u8>>,
        vark_data: Vec<Vec<u8>>,
    ) -> f64 {
        let mut discartability = self.calculate_mi_weight(mi_data);
        discartability += self.calculate_vark_weight(vark_data);

        for i in 0..DELTA_CALCULATIONS.len() {
            let idx = DELTA_CALCULATIONS[i];
            let col = &mut student_data[idx];
            col.sort_by(|a, b| a.partial_cmp(b).unwrap());
            discartability += calculate_gini(&col);
        }

        for i in 0..EPSILON_CALCULATIONS.len() {
            let idx = EPSILON_CALCULATIONS[i];
            let col = &mut student_data[idx];
            col.sort_by(|a, b| a.partial_cmp(b).unwrap());
            discartability += (0.5 - calculate_gini(&col)).abs();
        }

        return discartability;
    }

    fn calculate_mi_weight(&self, students_data: Vec<Vec<u8>>) -> f64 {
        let mut frecuencies = vec![0.0; 9 as usize];
        for student in students_data {
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

    fn calculate_vark_weight(&self, students_data: Vec<Vec<u8>>) -> f64 {
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

fn calculate_entropy(frecuencies: &Vec<f64>) -> f64 {
    let total: f64 = frecuencies.iter().sum();

    let mut entropy = 0.0;

    for i in 0..frecuencies.len() {
        let p_i: f64 = frecuencies[i] / total; // Calculate the probability for the i-th characteristic

        if p_i > 0.0 {
            entropy -= p_i * p_i.log2();
        }
    }

    return entropy / (total.log2() as f64); // Normalize the entropy to be between 0 and 1
}

fn calculate_gini(data: &Vec<f64>) -> f64 {
    const COEFICIENTS: [f64; 9] = [
        1642.0,
        1396.0,
        1462.0,
        1444.0,
        1450.0,
        1444.0,
        1462.0,
        1396.0,
        1642.0
    ];

    let mut sum: f64 = -6669.0;
    let n = data.len() as f64;

    for i in 1..10 {
        let mut decil = interpolate_decil(data, n, (i + 1) as f64);
        print!("{}, ", decil);
        decil *= COEFICIENTS[i - 1];
        sum += decil;
    }

    println!();

    println!("GINI: {}", -(sum / 7240.0));

    return -(sum / 7240.0);
}

fn interpolate_decil(data: &Vec<f64>, n: f64, decil: f64) -> f64 {
    let idx_down = (decil * (n-1.0) / 10.0).floor() as usize;
    let idx_up = (decil * (n-1.0) / 10.0).ceil() as usize;

    return data[idx_down] + (data[idx_up] - data[idx_down]) * (data[idx_up] - data[idx_down]);
}
