#[pyo3::pymodule]
mod group_enhancer {
    use kruscal::apply_kruscal;
    use kruscal::MST;
    use pyo3::exceptions::PyRuntimeError;
    use pyo3::prelude::*;
    use pyo3_polars::PyDataFrame;
    use std::collections::VecDeque;
    use symmetric_matrix::SymmetricMatrix;

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

    #[pyclass]
    struct PyMST {
        inner: MST,
        bfs_queue: VecDeque<usize>
    }

    #[pymethods]
    impl PyMST {
        #[new]
        fn new(inner: MST) -> Self {
            return PyMST {
                inner,
                bfs_queue: VecDeque::from([0usize]), //Start BFS from node 0
            };
        }

        fn next(&mut self) -> Option<usize> {
            //If there is no current pointer and the queue is empty, we have finished the traversal
            if self.bfs_queue.is_empty()
            {
                return Option::None;
            }

            let return_value = self.bfs_queue.pop_front().unwrap(); //Get the next node from the queue

            for neighbor in &self.inner.adj_list[return_value] {
                self.bfs_queue.push_back(*neighbor); //Add all neighbors to the queue
            }

            return Option::Some(return_value); //Return the current node
        }
    }

    #[pyfunction]
    fn make_gower_matrix(pydf: PyDataFrame) -> PyResult<PySymmetricMatrix> {
        let dataframe = pydf.into();
        let gower_matrix = gower::make_matrix(dataframe)
            .map_err(|e| PyErr::new::<PyRuntimeError, _>(format!("{}", e)))?;
        return Ok(PySymmetricMatrix {
            inner: gower_matrix,
        });
    }

    #[pyfunction]
    fn make_bias_matrix(pydf: PyDataFrame) -> PyResult<PySymmetricMatrix> {
        let dataframe = pydf.into();
        let bias_matrix = biases::make_matrix(dataframe)
            .map_err(|e| PyErr::new::<PyRuntimeError, _>(format!("{}", e)))?;
        return Ok(PySymmetricMatrix { inner: bias_matrix });
    }

    #[pyfunction]
    fn kruscal_minimum_spanning_tree(matrix: &PySymmetricMatrix) -> PyResult<PyMST> {
        let edges = apply_kruscal(&matrix.inner);
        return Ok(PyMST::new(edges));
    }
}
