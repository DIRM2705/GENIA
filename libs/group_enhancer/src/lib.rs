#[pyo3::pymodule]
mod group_enhancer {
    //use gower::calculate_gower_distance;
    use hypergraph::Hypergraph;
    use pyo3::prelude::*;
    use symmetric_matrix::SymmetricMatrix;
    
    #[pyclass]
    struct CharacteristicHG {
        inner: Hypergraph<u32>,
    }

    #[pymethods]
    impl CharacteristicHG {

        #[new]
        fn new(num_students: usize) -> Self {
            return CharacteristicHG {
                inner: Hypergraph::new(num_students),
            };
        }

        fn add_students_to_characteristic(&mut self, student_ids: Vec<usize>, characteristic : &str) {
            for student_id in student_ids
            {
                self.inner.add_id_to_hyperedge(student_id, &characteristic.to_string());
            }
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
