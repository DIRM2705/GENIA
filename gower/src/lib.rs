#[pyo3::pymodule]
mod gower_distance {
    use numpy::ndarray;
    use polars::prelude::*;
    use pyo3::prelude::*;
    use pyo3_polars::PyDataFrame;

    #[pyclass]
    struct GowerMatrix {
        data: ndarray::Array<f64, ndarray::Dim<[usize; 1]>>,
        size: usize,
    }

    impl GowerMatrix {
        fn new(size: usize) -> Self {
            //Create a new GowerMatrix with given size

            //The array only needs to store the upper triangular matrix thus, its max size is n*(n+1)/2 according
            //to the formula for the sum of the first n natural numbers
            let array_size = size * (size + 1) / 2; //max size of the array to store the upper triangular matrix
            return GowerMatrix {
                data: ndarray::Array::zeros((array_size,)), //Initialize the array with zeros
                size, //number of rows/columns in the square matrix
            };
        }

        fn get(&self, i: usize, j: usize) -> f64 {
            //Get the value at position (i,j) in the Gower matrix
            let i = i as i32;
            let j = j as i32;
            let index = (i * (i - 1) / 2 + j) as usize + self.size - 1; //Calculate the index in the 1D array
            return self.data[index];
        }

        fn set(&mut self, i: usize, j: usize, value: f64) {
            //Set the value at position (i,j) in the Gower matrix
            let i = i as i32;
            let j = j as i32;
            let index = (i * (i - 1) / 2 + j) as usize + self.size - 1; //Calculate the index in the 1D array
            self.data[index] = value; //Set the value
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
        let students_data: DataFrame = pydf.into(); // Convert PyDataFrame to Polars DataFrame
        let columns = students_data.get_columns(); // Get columns of the DataFrame
        let row_count = students_data.height(); // Get number of rows in the DataFrame
        let mut gower_matrix = GowerMatrix::new(row_count);

        for student1_index in 0..row_count {
            for student2_index in (student1_index + 1)..row_count {
                let distance =
                    get_distance_between_students(student1_index, student2_index, columns)?;
                gower_matrix.set(student1_index, student2_index, distance);
            }
        }
        Ok(gower_matrix)
    }

    fn get_distance_between_students(
        student1_index: usize,
        student2_index: usize,
        columns: &[Column],
    ) -> PyResult<f64> {
        //Calculate the Gower distance between two students
        let mut total_distance = 0.0;
        for col in columns {
            let distance = match col.dtype() {
                //Handle numerical columns
                DataType::Float64 => {
                    let s1_value = col.f64().unwrap().get(student1_index).unwrap();
                    let s2_value = col.f64().unwrap().get(student2_index).unwrap();
                    let rank =
                        col.f64().unwrap().max().unwrap() - col.f64().unwrap().min().unwrap();
                    get_distance_numerical(s1_value, s2_value, rank)
                }
                //Handle categorical columns
                DataType::Categorical(_, _) => {
                    let s1_value = col.get(student1_index).unwrap().to_string();
                    let s2_value = col.get(student2_index).unwrap().to_string();
                    get_distance_categorical(s1_value, s2_value)
                }
                _ => {
                    //Error handling for unsupported data types
                    return Err(PyErr::new::<pyo3::exceptions::PyTypeError, _>(format!(
                        "Unsupported data type in column {}",
                        col.name()
                    )))?;
                }
            };
            total_distance += distance;
        }
        return Ok(total_distance);
    }

    fn get_distance_categorical(s1_value: String, s2_value: String) -> f64 {
        //Get gower similarity between two categorical values
        if s1_value == s2_value {
            return 1.0;
        } else {
            return 0.0;
        }
    }

    fn get_distance_numerical(s1_value: f64, s2_value: f64, rank: f64) -> f64 {
        //Get gower similarity bewteen two numerical values
        let diff = 1.0 - ((s1_value - s2_value).abs() / rank);
        return diff;
    }
}
