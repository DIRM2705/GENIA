#[pyo3::pymodule]
mod group_enhancer {
    //use gower::calculate_gower_distance;
    use hypergraph::{Hypergraph, Student, CharacteristicType};
    use pyo3::prelude::*;
    use pyo3_polars::PyDataFrame;
    use pyo3::exceptions::PyValueError;
    use symmetric_matrix::SymmetricMatrix;

    #[pyclass]
    struct PyCharacteristicType {
        inner: CharacteristicType
    }

    #[pyclass]
    struct PyHypergraph {
        inner: Hypergraph,
    }

    #[pymethods]
    impl PyHypergraph {
        #[new]
        fn new() -> Self {
            return PyHypergraph {
                inner: Hypergraph::new(),
            };
        }

        fn add_students_to_characteristic(&mut self, characteristic : &PyCharacteristicType, student_ids: Vec<usize>) {
            for student_id in student_ids
            {
                self.inner.add_student_to_characteristic(&characteristic.inner, student_id);
            }
        }

        fn print(&self) {
            self.inner.print();
        }
    }

    #[pyclass]
    struct PyStudent {
        inner: Student,
    }

    /*#[pymethods]
    impl PyStudent {
        fn get_distance_to(&self, other: &PyStudent, ranks: Vec<f64>) -> f64 {
            return calculate_gower_distance(&self.inner, &other.inner, ranks.as_ref());
        }
    }*/

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
