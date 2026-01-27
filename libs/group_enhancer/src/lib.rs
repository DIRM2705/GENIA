#[pyo3::pymodule]
mod group_enhancer {
    use gower::calculate_gower_distance;
    use hypergraph::{Hypergraph, Student, CharacteristicType};
    use pyo3::prelude::*;
    use pyo3_polars::PyDataFrame;
    use pyo3::exceptions::PyValueError;
    use symmetric_matrix::SymmetricMatrix;

    #[pyclass]
    struct PyCharacteristicType {
        inner: CharacteristicType
    }
    /*enum PyCharacteristicType
    {
        Chronotype(CharacteristicType::Chronotype(u8)),
        AUTSIM(CharacteristicType::AUTSIM),
        DISCALCULIA(CharacteristicType::DISCALCULIA),
        ADHD(CharacteristicType::ADHD),
        DISGRAFIA(CharacteristicType::DISGRAFIA),
        MI_1(CharacteristicType::MI_1(u8)),
        MI_2(CharacteristicType::MI_2(u8)),
        MI_3(CharacteristicType::MI_3(u8)),
        VARK_VISUAL(CharacteristicType::VARK_VISUAL(f64)),
        VARK_AURAL(CharacteristicType::VARK_AURAL(f64)),
        VARK_RW(CharacteristicType::VARK_RW(f64)),
        VARK_KINESTHETIC(CharacteristicType::VARK_KINESTHETIC(f64)),
        BE(CharacteristicType::BE(f64)),
        EE(CharacteristicType::EE(f64)),
        CE(CharacteristicType::CE(f64)),
        AM(CharacteristicType::AM(f64)),
        CM(CharacteristicType::CM(f64)),
        RM(CharacteristicType::RM(f64)),
    }*/

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
