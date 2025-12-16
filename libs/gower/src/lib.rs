mod student_data;

use crate::student_data::StudentData;
use polars::prelude::DataFrame;
use std::sync::mpsc::Sender;
use std::sync::{Arc, Mutex, mpsc};
use std::thread;
use std::time::Duration;
use std::vec;
use symmetric_matrix::SymmetricMatrix;

pub fn make_matrix(dataframe: DataFrame) -> Result<SymmetricMatrix, Box<dyn std::error::Error>> {
    /*
       Gower distance calculation
       ARGS:
           df: DataFrame that includes both numerical and categorical variables
       RETURNS:
           struct SymmetricMatrix that includes the gower distance matrix and its size
    */

    let students_data = Arc::new(Mutex::new(StudentData::new(dataframe)));
    let student_count = students_data.lock().unwrap().rows;
    let mut gower_matrix = SymmetricMatrix::new(student_count);
    let (tx, rx) = mpsc::channel();
    let threads = create_threads(Arc::clone(&students_data), &gower_matrix, tx);

    // Wait for all threads to finish and collect results
    while let Ok((idx, distance)) = rx.recv_timeout(Duration::from_millis(500)) {
        gower_matrix.data[idx] = distance;
    }
    for handle in threads {
        handle.join().expect("Thread panicked");
    }

    return Ok(gower_matrix);
}

fn create_threads(
    student_data: Arc<Mutex<StudentData>>,
    gower_matrix: &SymmetricMatrix,
    sender: Sender<(usize, f64)>,
) -> Vec<thread::JoinHandle<()>> {
    let mut threads: Vec<thread::JoinHandle<()>> = vec![]; //Vector to hold thread handles
    let rows = student_data.lock().unwrap().rows;
    //Determine the number of threads to use
    let max_threads = num_cpus::get() - 2;

    //Calculate number of operations between students per thread
    let ops_per_thread = (rows * (rows - 1) / 2) / max_threads + 1;

    //Create threads to compute Gower distances in parallel
    for thread_index in 0..max_threads {
        let begin_index = thread_index * ops_per_thread;
        if begin_index >= gower_matrix.data.len() {
            break;
        }
        threads.push(create_thread(
            begin_index,
            ops_per_thread,
            gower_matrix,
            Arc::clone(&student_data),
            sender.clone(),
        ));
    }

    return threads;
}

fn create_thread(
    mut op_idx: usize,
    mut ops_per_thread: usize,
    gower_matrix: &SymmetricMatrix,
    student_data: Arc<Mutex<StudentData>>,
    sender: Sender<(usize, f64)>,
) -> thread::JoinHandle<()> {
    let (mut i, mut j) = gower_matrix.get_indices(op_idx);
    let size = gower_matrix.size;
    return thread::spawn(move || {
        let student_data_guard = student_data.lock().unwrap();
        let total_categories = student_data_guard.category_count;
        let mut i_num_distances = student_data_guard.get_row_numerical(i);
        let mut i_cat_distances = student_data_guard.get_row_categorical(i);
        let ranks = student_data_guard.ranks.clone();
        drop(student_data_guard); //Release the lock before entering the loop

        while ops_per_thread > 0 {
            let student_data_guard = student_data.lock().unwrap();
            let j_num_distances = student_data_guard.get_row_numerical(j);
            let j_cat_distances = student_data_guard.get_row_categorical(j);
            drop(student_data_guard); //Release the lock before calculating distances

            //Calculate distances for numerical columns
            let num_distance =
                calculate_distances_numerical(&i_num_distances, &j_num_distances, &ranks);
            let cat_distance = calculate_distances_categorical(&i_cat_distances, &j_cat_distances);

            let total_distance = 1.0 - (num_distance + cat_distance) / total_categories;
            sender.send((op_idx, total_distance)).unwrap();

            //Move to the next pair (i, j)
            ops_per_thread -= 1;
            op_idx += 1;

            if j + 1 >= size {
                i += 1;
                if i >= size - 1 {
                    break; //All pairs processed
                }
                let student_data_guard = student_data.lock().unwrap();
                i_num_distances = student_data_guard.get_row_numerical(i);
                i_cat_distances = student_data_guard.get_row_categorical(i);
                drop(student_data_guard); //Release the lock before continuing
                j = i + 1;
            } else {
                j += 1;
            }
        }
    });
}

pub fn calculate_distances_numerical(
    s1_row: &Vec<f64>,
    s2_row: &Vec<f64>,
    ranks: &Vec<f64>,
) -> f64 {
    //Get gower similarity bewteen two numerical values
    let mut distance = 0.0;
    for k in 0..s1_row.len() {
        let diff = (s1_row[k] - s2_row[k]).abs();
        distance += 1.0 - diff / ranks[k];
    }
    return distance;
}

pub fn calculate_distances_categorical(s1_row: &Vec<String>, s2_row: &Vec<String>) -> f64 {
    //Get gower similarity bewteen two categorical values
    let mut distance = 0.0;
    for k in 0..s1_row.len() {
        if s1_row[k] == s2_row[k] {
            distance += 1.0;
        }
    }
    return distance;
}
