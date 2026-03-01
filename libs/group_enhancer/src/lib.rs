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
