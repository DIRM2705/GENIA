#[pyo3::pymodule]
mod group_enhancer {
    //use gower::calculate_gower_distance;
    use hypergraph::{Hypergraph, Student, CharacteristicType};
    use pyo3::prelude::*;
    use pyo3_polars::PyDataFrame;
    use pyo3::exceptions::PyValueError;
    use symmetric_matrix::SymmetricMatrix;

    #[pyclass]
    enum PyCharacteristicType {
        Chronotype,
        ADHD,
        AUTSIM,
        DISLEXIA,
        DISGRAFIA,
        DISCALCULIA,
        MIKin,
        MIExis,
        MIInter,
        MIIntra,
        MILog,
        MIMus,
        MINat,
        MIVer,
        MIVis,  
        VarkVisual,
        VarkAural,
        VarkRW,
        VarkKinesthetic,
        BE,
        EE,
        CE,
        AM,
        CM,
        RM
    }

    impl PyCharacteristicType
    {
        fn to_characteristic_type(&self, value: u8) -> CharacteristicType {
            return match self {
                PyCharacteristicType::Chronotype => CharacteristicType::Chronotype(value),
                PyCharacteristicType::ADHD => CharacteristicType::ADHD,
                PyCharacteristicType::AUTSIM => CharacteristicType::Autism,
                PyCharacteristicType::DISLEXIA => CharacteristicType::Dislexia,
                PyCharacteristicType::DISGRAFIA => CharacteristicType::Disgrafia,
                PyCharacteristicType::DISCALCULIA => CharacteristicType::Discalculia,
                PyCharacteristicType::MIKin => CharacteristicType::MIKin(value),
                PyCharacteristicType::MIExis => CharacteristicType::MIExis(value),
                PyCharacteristicType::MIInter => CharacteristicType::MIInter(value),
                PyCharacteristicType::MIIntra => CharacteristicType::MIIntra(value),
                PyCharacteristicType::MILog => CharacteristicType::MILog(value),
                PyCharacteristicType::MIMus => CharacteristicType::MIMus(value),
                PyCharacteristicType::MINat => CharacteristicType::MINat(value),    
                PyCharacteristicType::MIVer => CharacteristicType::MIVer(value),
                PyCharacteristicType::MIVis => CharacteristicType::MIVis(value),
                PyCharacteristicType::VarkVisual => CharacteristicType::VarkVisual(value),
                PyCharacteristicType::VarkAural => CharacteristicType::VarkAural(value),
                PyCharacteristicType::VarkRW => CharacteristicType::VarkRW(value),
                PyCharacteristicType::VarkKinesthetic => CharacteristicType::VarkKinesthetic(value),
                PyCharacteristicType::BE => CharacteristicType::BE(value),
                PyCharacteristicType::EE => CharacteristicType::EE(value),
                PyCharacteristicType::CE => CharacteristicType::CE(value),
                PyCharacteristicType::AM => CharacteristicType::AM(value),
                PyCharacteristicType::CM => CharacteristicType::CM(value),
                PyCharacteristicType::RM => CharacteristicType::RM(value),
            }
        }
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

        fn add_students_to_characteristic(&mut self, student_ids: Vec<usize>, characteristic : &PyCharacteristicType, value: Option<u8>) {
            for student_id in student_ids
            {
                let characteristic = characteristic.to_characteristic_type(value.unwrap_or(0));
                self.inner.add_student_to_characteristic(&characteristic, student_id);
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
