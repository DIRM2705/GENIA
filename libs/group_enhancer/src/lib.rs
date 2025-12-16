#[pyo3::pymodule]
mod group_enhancer 
{
    use symmetric_matrix::SymmetricMatrix;
    use kruscal::MST;
    use pyo3::prelude::*;
    use pyo3::exceptions::PyRuntimeError;
    use pyo3_polars::PyDataFrame;
    use kruscal::apply_kruscal;

    #[pyclass]
    struct PySymmetricMatrix
    {
        inner : SymmetricMatrix
    }

    #[pymethods]
    impl PySymmetricMatrix
    {
        fn __len__(&self) -> PyResult<usize>
        {
            return Ok(self.inner.size);
        }

        fn get(&self, i: usize, j: usize) -> PyResult<f64>
        {
            return Ok(self.inner.get(i, j));
        }
    }

    #[pyclass]
    struct PyMST
    {
        inner : MST
    }

    #[pyfunction]
    fn make_gower_matrix(pydf : PyDataFrame) -> PyResult<PySymmetricMatrix>
    {
        let dataframe = pydf.into();
        let gower_matrix = gower::make_matrix(dataframe).map_err(|e| PyErr::new::<PyRuntimeError, _>(format!("{}", e)))?;
        return Ok(PySymmetricMatrix { inner: gower_matrix })
    }

    #[pyfunction]
    fn make_bias_matrix(pydf : PyDataFrame) -> PyResult<PySymmetricMatrix>
    {
        let dataframe = pydf.into();
        let bias_matrix = biases::make_matrix(dataframe).map_err(|e| PyErr::new::<PyRuntimeError, _>(format!("{}", e)))?;
        return Ok(PySymmetricMatrix { inner: bias_matrix })
    }

    #[pyfunction]
    fn kruscal_minimum_spanning_tree(matrix : &PySymmetricMatrix) -> PyResult<PyMST>
    {
        let edges = apply_kruscal(&matrix.inner);
        return Ok(PyMST { inner: edges });
    }
}
