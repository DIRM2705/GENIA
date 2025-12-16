use polars::prelude::DataFrame;
use polars::prelude::*;
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
           struct SymmetricMatrix that includes the value matrix and its size
    */
    let student_count = dataframe.height();
    let avg = dataframe
        .column("media")
        .unwrap()
        .f64()
        .unwrap()
        .mean()
        .unwrap();
    let gpas = dataframe
        .column("media")
        .unwrap()
        .f64()
        .unwrap()
        .into_no_null_iter()
        .collect::<Vec<f64>>();
    let gpa_ref = Arc::new(Mutex::new(gpas));
    drop(dataframe); //Free up memory
    let mut bmatrix = SymmetricMatrix::new(student_count);
    let (tx, rx) = mpsc::channel();
    let threads = create_threads(avg, gpa_ref, &bmatrix, tx);

    // Wait for all threads to finish and collect results
    while let Ok((idx, distance)) = rx.recv_timeout(Duration::from_millis(500)) {
        bmatrix.data[idx] = distance;
    }
    for handle in threads {
        handle.join().expect("Thread panicked");
    }

    return Ok(bmatrix);
}

fn create_threads(
    avg: f64,
    gpa_ref: Arc<Mutex<Vec<f64>>>,
    bmatrix: &SymmetricMatrix,
    sender: Sender<(usize, f64)>,
) -> Vec<thread::JoinHandle<()>> {
    let mut threads: Vec<thread::JoinHandle<()>> = vec![]; //Vector to hold thread handles
    let rows = bmatrix.size;
    //Determine the number of threads to use
    let max_threads = num_cpus::get() - 2;

    //Calculate number of operations between students per thread
    let ops_per_thread = (rows * (rows - 1) / 2) / max_threads + 1;
    //Create threads to compute Gower distances in parallel
    for thread_index in 0..max_threads {
        let begin_index = thread_index * ops_per_thread;
        if begin_index >= bmatrix.data.len() {
            break;
        }
        threads.push(create_thread(
            avg,
            gpa_ref.clone(),
            begin_index,
            ops_per_thread,
            bmatrix,
            sender.clone(),
        ));
    }

    return threads;
}

fn create_thread(
    avg: f64,
    gpa_ref: Arc<Mutex<Vec<f64>>>,
    mut op_idx: usize,
    mut ops_per_thread: usize,
    bmatrix: &SymmetricMatrix,
    sender: Sender<(usize, f64)>,
) -> thread::JoinHandle<()> {
    let (mut i, mut j) = bmatrix.get_indices(op_idx);
    let size = bmatrix.size;
    return thread::spawn(move || {
        let student_data_guard = gpa_ref.lock().unwrap();
        let mut i_value = student_data_guard[i];
        drop(student_data_guard); //Release the lock before entering the loop

        while ops_per_thread > 0 {
            let student_data_guard = gpa_ref.lock().unwrap();
            let j_value = student_data_guard[j];
            drop(student_data_guard); //Release the lock before calculating distances

            //Calculate distances for numerical columns
            let total_distance = (avg - (i_value + j_value) / 2.0).abs();
            sender.send((op_idx, total_distance)).unwrap();

            //Move to the next pair (i, j)
            ops_per_thread -= 1;
            op_idx += 1;

            if j + 1 >= size {
                i += 1;
                if i >= size - 1 {
                    break; //All pairs processed
                }
                let student_data_guard = gpa_ref.lock().unwrap();
                i_value = student_data_guard[i];
                drop(student_data_guard); //Release the lock before continuing
                j = i + 1;
            } else {
                j += 1;
            }
        }
    });
}
