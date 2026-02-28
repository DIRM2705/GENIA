#[pyo3::pymodule]
mod group_enhancer {
    use gower::calculate_gower_matrix;
    use hypergraph::Hypergraph;
    use pyo3::prelude::*;
    use pyo3_polars::PyDataFrame;
    use symmetric_matrix::SymmetricMatrix;
    
    #[pyclass]
    struct GroupHG {
        inner: Hypergraph<u32>,
    }

    #[pymethods]
    impl GroupHG {

        #[new]
        fn new(num_students: usize) -> Self {
            return GroupHG {
                inner: Hypergraph::new(num_students),
            };
        }

        fn add_students_to_hyperedge(&mut self, students : PyDataFrame, characteristic : &str) {
            self.inner.add_students_to_hyperedge(students.into(), &characteristic.to_string());
        }

        fn print(&self) {
            self.inner.print();
        }
    }

    #[pyclass]
    struct PySymmetricMatrix {
        inner: SymmetricMatrix,
    }

    #[pymethods]
    impl PySymmetricMatrix {
        fn __len__(&self) -> PyResult<usize> {
            return Ok(self.inner.size);
        }

        fn get(&self, i: usize, j: usize) -> PyResult<f64> {
            return Ok(self.inner.get(i, j));
        }
    }
}
