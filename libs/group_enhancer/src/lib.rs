#[pyo3::pymodule]
mod group_enhancer {
    use gower::calculate_gower_matrix;
    use hypergraph::Hypergraph;
    use pyo3::prelude::*;
    use pyo3_polars::PyDataFrame;
    use polars::prelude::{DataFrame, IntoLazy};
    use symmetric_matrix::SymmetricMatrix;
    use genetics::Individual;

    #[pyclass]
    struct PyIndividual {
        inner: Individual,
    }

    #[pymethods]
    impl PyIndividual {
        #[new]
        fn new(students_id: Vec<Vec<u32>>, df: PyDataFrame) -> Self {
            let df : DataFrame = df.into();
            return PyIndividual {
                inner: Individual::new(students_id, df.lazy()),
            };
        }

        fn fit(&mut self) {
            self.inner.calculate_fitness();
        }
    }

    #[pyclass]
    struct CharacteristicHG
    {
        hypergraph : Vec<u64>,
        mi_matrix : SymmetricMatrix,
        vark_matrix : SymmetricMatrix,
        am_matrix : SymmetricMatrix,
        rm_matrix : SymmetricMatrix,
        cm_matrix : SymmetricMatrix,
        be_matrix : SymmetricMatrix,
        ee_matrix : SymmetricMatrix,
        ce_matrix : SymmetricMatrix,
    }

    #[pymethods]
    impl CharacteristicHG {
        #[new]
        fn new(students: usize) -> Self {
            return CharacteristicHG {
                hypergraph: Hypergraph::new(students),
                mi_matrix: SymmetricMatrix::new(students),
                vark_matrix: SymmetricMatrix::new(students),
                am_matrix: SymmetricMatrix::new(students),
                rm_matrix: SymmetricMatrix::new(students),
                cm_matrix: SymmetricMatrix::new(students),
                be_matrix: SymmetricMatrix::new(students),
                ee_matrix: SymmetricMatrix::new(students),
                ce_matrix: SymmetricMatrix::new(students),
            };
        }

        fn add_to_hyperedge(&mut self, student_idx: usize, hyperedge_idx: usize) {
            self.hypergraph.add_to_hyperedge(student_idx, hyperedge_idx);
        }

        fn print(&self) {
            self.hypergraph.print();
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
