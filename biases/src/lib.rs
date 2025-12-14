#[pyo3::pymodule]
mod biases {
    use numpy::ndarray;
    use pyo3::prelude::*;
    use pyo3_polars::PyDataFrame;
    use polars::prelude::*;
    use std::cmp::{max, min};
    use std::sync::mpsc::Sender;
    use std::sync::{Arc, Mutex, mpsc};
    use std::thread;
    use std::time::Duration;
    use std::vec;

    #[pyclass]
    struct BiasMatrix {
        data: ndarray::Array<f64, ndarray::Dim<[usize; 1]>>, //1D array to store upper triangular matrix values
        #[pyo3(get)]
        size: usize,
        row_starts: Vec<i32>,
    }

    #[pymethods]
    impl BiasMatrix {
        #[new]
        fn new(size: usize) -> Self {
            //Create a new BiasMatrix with given size

            //The array only needs to store the upper triangular matrix thus, its max size is n*(n+1)/2 according
            //to the formula for the sum of the first n natural numbers
            let array_size = size * (size - 1) / 2; //max size of the array to store the upper triangular matrix

            //Precompute the starting indices of each row in the 1D array
            let mut row_starts = Vec::with_capacity(size);
            let n = size as i32;
            row_starts.push(-1);
            for i in 1..size as i32 {
                let start_index = row_starts[(i - 1) as usize] - 1 + n - i;
                row_starts.push(start_index);
            }
            return BiasMatrix {
                data: ndarray::Array::zeros((array_size,)), //Initialize the array with zeros
                size,       //number of rows/columns in the square matrix
                row_starts, //starting indices of each virtual row representation
            };
        }

        fn get_indices(&self, index: usize) -> (usize, usize) {
            /*
               Given a linear index in the 1D array representation of the upper triangular matrix,
               return the corresponding (i, j) indices in the 2D matrix.
            */
            let p = index as f64;
            let n = self.size as f64;
            assert!(index < self.data.len(), "Index out of bounds");
            let b = 2.0 * n - 3.0; //Coefficient for the quadratic equation
            let c = 8.0 * (p - n + 2.0); //Constant term for the quadratic equation

            //Solve the quadratic equation to find the row index i
            let i = ((b - (b * b - c).sqrt()) / 2.0).ceil() as usize;
            //Calculate the column index j
            let j = (p - self.row_starts[i] as f64) as usize;
            return (i, j);
        }

        fn get(&self, i1: usize, j1: usize) -> f64 {
            assert!(i1 < self.size && j1 < self.size, "Index out of bounds");
            if i1 == j1 {
                return 0.0; //Distance to self is zero
            }
            
            //else
            //Get the value at position (i,j) in the Gower matrix
            let i = min(i1, j1);
            let j = max(i1, j1);
            let index = self.row_starts[i] + j as i32; //Calculate the index in the 1D array
            return self.data[index as usize];
        }

        fn set(&mut self, index : usize, value: f64) {
            assert!(index < self.data.len(), "Index out of bounds");
            self.data[index] = value; //Set the value in the 1D array
        }
    }
    #[pyfunction]
    fn make_bias_matrix(pydf: PyDataFrame) -> PyResult<BiasMatrix> {
        /*
           Gower distance calculation
           ARGS:
               df: DataFrame that includes both numerical and categorical variables
           RETURNS:
               struct GowerMatrix that includes the gower distance matrix and its size
        */

        let dataframe : DataFrame = pydf.into();
        let student_count = dataframe.height();
        let avg = dataframe.column("media").unwrap().f64().unwrap().mean().unwrap();
        let gpas = dataframe.column("media").unwrap().f64().unwrap().into_no_null_iter().collect::<Vec<f64>>();
        let gpa_ref = Arc::new(Mutex::new(gpas));
        drop(dataframe); //Free up memory
        let mut bmatrix = BiasMatrix::new(student_count);
        let (tx, rx) = mpsc::channel();
        let threads = create_threads(avg, gpa_ref, &bmatrix, tx);

        // Wait for all threads to finish and collect results
        while let Ok((idx, distance)) = rx.recv_timeout(Duration::from_millis(500)) {
            bmatrix.set(idx, distance);
        }
        for handle in threads {
            handle.join().expect("Thread panicked");
        }

        return Ok(bmatrix);
    }

    fn create_threads(
        avg : f64,
        gpa_ref : Arc<Mutex<Vec<f64>>>,
        bmatrix: &BiasMatrix,
        sender: Sender<(usize, f64)>,
    ) -> Vec<thread::JoinHandle<()>> {
        let mut threads: Vec<thread::JoinHandle<()>> = vec![]; //Vector to hold thread handles
        let rows = bmatrix.size;
        //Determine the number of threads to use
        let max_threads = num_cpus::get() - 2;

        //Calculate number of operations between students per thread
        let ops_per_thread = (rows * (rows - 1) / 2) / max_threads+1;
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
        avg : f64,
        gpa_ref : Arc<Mutex<Vec<f64>>>,
        mut op_idx: usize,
        mut ops_per_thread: usize,
        bmatrix: &BiasMatrix,
        sender: Sender<(usize, f64)>,
    ) -> thread::JoinHandle<()> {
        let (mut i, mut j) = bmatrix.get_indices(op_idx);
        let size = bmatrix.size;
        return thread::spawn(move || {
            let student_data_guard = gpa_ref.lock().unwrap();
            let mut i_value = student_data_guard[i];
            drop(student_data_guard); //Release the lock before entering the loop

            while ops_per_thread > 0 
            {
                let student_data_guard = gpa_ref.lock().unwrap();
                let j_value = student_data_guard[j];
                drop(student_data_guard); //Release the lock before calculating distances

                //Calculate distances for numerical columns
                let total_distance = (avg - (i_value + j_value)/2.0).abs();
                sender.send((op_idx, total_distance)).unwrap();

                //Move to the next pair (i, j)
                ops_per_thread -= 1;
                op_idx += 1;
                
                if j + 1 >= size {
                    i += 1;
                    if i >= size-1 {
                        break; //All pairs processed
                    }
                    let student_data_guard = gpa_ref.lock().unwrap();
                    i_value = student_data_guard[i];
                    drop(student_data_guard); //Release the lock before continuing
                    j = i+1;
                }
                else 
                {
                    j += 1;
                }
            }
        });
    }
}
