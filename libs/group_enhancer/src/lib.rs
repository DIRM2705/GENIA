#[pyo3::pymodule]
mod group_enhancer {
    use gower::calculate_gower_distance;
    use hypergraph::{Hypergraph, Student};
    use pyo3::prelude::*;
    use pyo3_polars::PyDataFrame;
    use pyo3::exceptions::PyValueError;
    use symmetric_matrix::SymmetricMatrix;

    #[pyclass]
    struct PyHypergraph {
        inner: Hypergraph,
    }

    #[pymethods]
    impl PyHypergraph {
        #[new]
        fn new(py_df : PyDataFrame) -> Self {
            return PyHypergraph {
                inner: Hypergraph::new(py_df.into()),
            };
        }

        fn add_hyperedges_from_classes(&mut self, class_count: usize, class_base_name: &str) {
            for class in 0..class_count {
                let class_name = format!("{}{}", class_base_name, class);
                self.inner.add_hyperedge(class_name);
            }
        }

        fn add_hyperedge(&mut self, hyperedge_id : &str) {
            self.inner.add_hyperedge(hyperedge_id.to_string());
        }

        fn add_student_to_hyperedge(&mut self, student_id: usize, hyperedge_id: &str) -> PyResult<()>
        {
            self.inner
                .add_student_to_hyperedge(hyperedge_id, student_id).map_err(|e| PyErr::new::<PyValueError, _>(e))?;
            return Ok(());
        }

        pub fn print(&self) {
            self.inner.print();
        }
    }

    #[pyclass]
    struct PyStudent {
        inner: Student,
    }

    #[pymethods]
    impl PyStudent {
        fn get_distance_to(&self, other: &PyStudent, ranks: Vec<f64>) -> f64 {
            return calculate_gower_distance(&self.inner, &other.inner, ranks.as_ref());
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
