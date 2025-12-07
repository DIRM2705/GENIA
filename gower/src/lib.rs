#[pyo3::pymodule]
mod gower_distance {
    use numpy::ndarray;
    use polars::prelude::AnyValue;
    use polars::prelude::*;
    use pyo3::PyResult;
    use pyo3::exceptions::*;
    use pyo3::prelude::*;
    use pyo3_polars::PyDataFrame;
    use std::cmp::{max, min};
    use std::sync::{Arc, Mutex};
    use std::thread;

    #[pyclass]
    struct GowerMatrix {
        data: ndarray::Array<f64, ndarray::Dim<[usize; 1]>>,
        #[pyo3(get)]
        size: usize,
        row_starts: Vec<usize>,
    }

    #[pymethods]
    impl GowerMatrix {
        #[new]
        fn new(size: usize) -> Self {
            //Create a new GowerMatrix with given size

            //The array only needs to store the upper triangular matrix thus, its max size is n*(n+1)/2 according
            //to the formula for the sum of the first n natural numbers
            let array_size = size * (size + 1) / 2; //max size of the array to store the upper triangular matrix

            //Precompute the starting indices of each row in the 1D array
            let mut row_starts = Vec::with_capacity(size);
            row_starts.push(0);
            for i in 1..size {
                let start_index = row_starts[i - 1] + size - 2*i + i;
                row_starts.push(start_index);
            }
            return GowerMatrix {
                data: ndarray::Array::zeros((array_size,)), //Initialize the array with zeros
                size, //number of rows/columns in the square matrix
                row_starts,
            };
        }

        fn get(&self, i1: usize, j1: usize) -> f64 {
            assert!(i1 < self.size && j1 < self.size, "Index out of bounds");
            //Get the value at position (i,j) in the Gower matrix
            let i = min(i1, j1);
            let j = max(i1, j1);
            let index = self.row_starts[i] + j; //Calculate the index in the 1D array
            return self.data[index];
        }

        fn set(&mut self, i1: usize, j1: usize, value: f64) {
            assert!(i1 < self.size && j1 < self.size, "Index out of bounds");
            //Set the value at position (i,j) in the Gower matrix
            let i = min(i1, j1);
            let j = max(i1, j1);
            let index = self.row_starts[i] + j; //Calculate the index in the 1D array
            self.data[index] = value; //Set the value
        }
    }
    struct StudentData {
        data_frame: DataFrame,
        rows: usize,
        columns: usize,
        dtypes: Vec<DataType>,
    }

    impl StudentData {
        fn new(data_frame: DataFrame) -> Self {
            let column_count = data_frame.width();
            let row_count = data_frame.height();
            let dtypes = data_frame
                .get_columns()
                .iter()
                .map(|col| col.dtype().clone())
                .collect();
            StudentData {
                columns: column_count,
                rows: row_count,
                dtypes,
                data_frame,
            }
        }

        fn get_distance_between_students(&self, s1_index: usize, s2_index: usize) -> PyResult<f64> {
            //Calculate the Gower distance between two students
            let mut total_distance = 0.0;
            let row_1 = self.data_frame.get(s1_index);
            let row_2 = self.data_frame.get(s2_index);

            if row_1.is_none() || row_2.is_none() {
                return Err(PyErr::new::<PyIndexError, _>("Student index out of bounds"))?;
            }

            let s1_row = row_1.unwrap();
            let s2_row = row_2.unwrap();
            let ranks = self.get_column_ranks();

            for i in 0..self.columns {
                let student_1 = s1_row.get(i).unwrap();
                let student_2 = s2_row.get(i).unwrap();
                let distance = match self.dtypes[i] {
                    //Handle numerical columns
                    DataType::Float64 => get_distance_numerical(student_1, student_2, ranks[i]),
                    //Handle categorical columns
                    DataType::Categorical(_, _) => get_distance_categorical(student_1, student_2),
                    _ => {
                        //Error handling for unsupported data types
                        return Err(PyErr::new::<PyTypeError, _>(format!(
                            "Unsupported data type in column {}",
                            self.data_frame.get_column_names()[i]
                        )))?;
                    }
                };
                total_distance += distance;
            }
            return Ok(total_distance);
        }

        fn get_column_ranks(&self) -> Vec<f64> {
            //Calculate the rank (max - min) for each numerical column
            let mut ranks: Vec<f64> = vec![];
            for col in self.data_frame.get_columns() {
                match col.dtype() {
                    DataType::Float64 => {
                        let series = col.f64().unwrap();
                        let min_value = series.min().unwrap_or(0.0);
                        let max_value = series.max().unwrap_or(0.0);
                        let rank = max_value - min_value;
                        ranks.push(rank);
                    }
                    _ => {
                        //For non-numerical columns, push a dummy value (not used)
                        ranks.push(0.0);
                    }
                }
            }
            return ranks;
        }
    }

    #[pyfunction]
    fn make_gower_matrix(pydf: PyDataFrame) -> PyResult<GowerMatrix> {
        /*
           Gower distance calculation
           ARGS:
               df: DataFrame that includes both numerical and categorical variables
           RETURNS:
               struct GowerMatrix that includes the gower distance matrix and its size
        */

        let students_data = Arc::new(Mutex::new(StudentData::new(pydf.into())));
        let student_count = students_data.lock().unwrap().rows;
        let gower_matrix = Arc::new(Mutex::new(GowerMatrix::new(student_count)));
        let threads = create_threads(Arc::clone(&students_data), Arc::clone(&gower_matrix));

        // Wait for all threads to finish
        for handle in threads {
            handle.join().expect("Thread panicked");
        }

        // Unwrap the Arc<Mutex<GowerMatrix>> to return the GowerMatrix
        let matrix = Arc::try_unwrap(gower_matrix)
            .map_err(|_| PyErr::new::<PyRuntimeError, _>("Failed to unwrap Arc"))?
            .into_inner()
            .map_err(|_| {
                PyErr::new::<PyRuntimeError, _>("Failed to lock Mutex")
            })?;

        return Ok(matrix);
    }

    fn create_threads(
        student_data: Arc<Mutex<StudentData>>,
        matrix: Arc<Mutex<GowerMatrix>>,
    ) -> Vec<thread::JoinHandle<()>> {
        let mut threads: Vec<thread::JoinHandle<()>> = vec![]; //Vector to hold thread handles
        let max_threads = num_cpus::get(); //Get the number of available CPU cores
        let rows = student_data.lock().unwrap().rows;
        let students_per_thread = rows / max_threads + 1; //Calculate number of students per thread
        
        //Create threads to compute Gower distances in parallel
        let mut thread_index = 0;
        loop
        {
            let student_index = thread_index * students_per_thread;
            if student_index >= rows {
                break;
            }
            threads.push(create_thread(
                student_index,
                students_per_thread,
                Arc::clone(&student_data),
                Arc::clone(&matrix),
            ));

            thread_index += 1;
        }

        return threads;
    }

    fn create_thread(
        mut student_index: usize,
        students_per_thread: usize,
        student_data: Arc<Mutex<StudentData>>,
        matrix: Arc<Mutex<GowerMatrix>>,
    ) -> thread::JoinHandle<()> {
        return thread::spawn(move || {
            let student_data_guard = student_data.lock().unwrap();
            let mut matrix_guard = matrix.lock().unwrap();
            let rows = student_data_guard.rows;
            let columns = student_data_guard.columns;
            let end_index = min(student_index + students_per_thread, rows);
            while student_index < end_index
            {
                for j in (student_index + 1)..rows {
                    let distance = match student_data_guard.get_distance_between_students(student_index, j) {
                        Ok(dist) => 1.0 - dist / columns as f64,
                        Err(_) => return,
                    };
                    matrix_guard.set(student_index, j, distance);
                }
                student_index += 1;
            }
        });
    }

    fn get_distance_categorical(s1_value: &AnyValue, s2_value: &AnyValue) -> f64 {
        let s1_value = s1_value.to_string();
        let s2_value = s2_value.to_string();   
        //Get gower similarity between two categorical values
        if s1_value == s2_value {
            return 1.0;
        } else {
            return 0.0;
        }
    }

    fn get_distance_numerical(s1_value: &AnyValue, s2_value: &AnyValue, rank: f64) -> f64 {
        let s1_value = match s1_value {
            AnyValue::Float64(v) => v,
            _ => panic!("Expected Float64 value"),
        };
        let s2_value = match s2_value {
            AnyValue::Float64(v) => v,
            _ => panic!("Expected Float64 value"),
        };
        //Get gower similarity bewteen two numerical values
        let diff = 1.0 - ((s1_value - s2_value).abs() / rank);
        return diff;
    }
}
