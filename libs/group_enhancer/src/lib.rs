#[pyo3::pymodule]
mod group_enhancer {
    use hypergraph::{Hypergraph, Student};
    use pyo3::exceptions::PyRuntimeError;
    use pyo3::prelude::*;
    use pyo3_polars::PyDataFrame;
    use gower::calculate_gower_distance;
    use symmetric_matrix::SymmetricMatrix;

    #[pyclass]
    struct PyHypergraph
    {
        inner: Hypergraph
    }

    #[pyclass]
    struct PyStudent
    {
        inner: Student
    }

    #[pymethods]
    impl PyStudent
    {
        #[new]
        fn new(
            id: usize,
            ndd: u8,
            mi_order: [u8; 8],
            vark_scores: [f64; 4],
            be: f64,
            ee: f64,
            ce: f64,
            autonomous_motivation: f64,
            competitive_motivation: f64,
            relationship_motivation: f64,
            gpa: f64,
        ) -> Self {
            return PyStudent {
                inner: Student {
                    id,
                    ndd,
                    mi_order,
                    vark_scores,
                    be,
                    ee,
                    ce,
                    autonomous_motivation,
                    competitive_motivation,
                    relationship_motivation,
                    gpa,
                },
            };
        }

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

    #[pyfunction]
    fn make_bias_matrix(pydf: PyDataFrame) -> PyResult<PySymmetricMatrix> {
        let dataframe = pydf.into();
        let bias_matrix = biases::make_matrix(dataframe)
            .map_err(|e| PyErr::new::<PyRuntimeError, _>(format!("{}", e)))?;
        return Ok(PySymmetricMatrix { inner: bias_matrix });
    }
}
