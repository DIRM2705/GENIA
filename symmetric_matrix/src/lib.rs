#[pyo3::pymodule]
pub mod symmetric_matrix {
    use numpy::ndarray;
    use pyo3::prelude::*;

    #[pyclass]
    pub struct SymmetricMatrix {
        pub data: ndarray::Array<f64, ndarray::Dim<[usize; 1]>>, //1D array to store upper triangular matrix values
        #[pyo3(get)]
        pub size: usize,
        pub row_starts: Vec<i32>,
    }

    #[pymethods]
    impl SymmetricMatrix {
        #[new]
        pub fn new(size: usize) -> Self {
            //Create a new SymmetricMatrix with given size

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
            return SymmetricMatrix {
                data: ndarray::Array::zeros((array_size,)), //Initialize the array with zeros
                size,       //number of rows/columns in the square matrix
                row_starts, //starting indices of each virtual row representation
            };
        }

        pub fn get_indices(&self, index: usize) -> (usize, usize) {
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

        pub fn get(&self, i1: usize, j1: usize) -> f64 {
            assert!(i1 < self.size && j1 < self.size, "Index out of bounds");
            if i1 == j1 {
                return 0.0; //Distance to self is zero
            }
            //else
            let (i, j) = if i1 < j1 { (i1, j1) } else { (j1, i1) }; //Ensure i < j for upper triangular access
            let index = (self.row_starts[i] + j as i32) as usize; //Calculate the linear index
            return self.data[index]; //Return the stored value
        }
    }

    #[cfg(test)]
    mod tests {
        use super::*;

        #[test]
        fn test_symmetric_matrix() {
            let size = 4;
            let mut sym_matrix = SymmetricMatrix::new(size);
            // Manually set some values in the upper triangular part
            sym_matrix.data[0] = 1.0; // (0,1)
            sym_matrix.data[1] = 2.0; // (0,2)
            sym_matrix.data[2] = 3.0; // (0,3)
            sym_matrix.data[3] = 4.0; // (1,2)
            sym_matrix.data[4] = 5.0; // (1,3)
            sym_matrix.data[5] = 6.0; // (2,3)
            // Test retrieval of values
            assert_eq!(sym_matrix.get(0, 1), 1.0);
            assert_eq!(sym_matrix.get(1, 0), 1.0); // Symmetry check
            assert_eq!(sym_matrix.get(0, 2), 2.0);
            assert_eq!(sym_matrix.get(2, 0), 2.0); //
            assert_eq!(sym_matrix.get(1, 3), 5.0);
            assert_eq!(sym_matrix.get(3, 1), 5.0); //
            assert_eq!(sym_matrix.get(2, 2), 0.0); // Distance to self
        }
    }
}
