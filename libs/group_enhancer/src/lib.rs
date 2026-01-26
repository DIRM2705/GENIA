#[pyo3::pymodule]
mod group_enhancer {
    use gower::calculate_gower_distance;
    use hypergraph::{Hypergraph, Student};
    use ordered_f64::OrderedF64;
    use pyo3::prelude::*;
    use symmetric_matrix::SymmetricMatrix;

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

        fn add_hyperedges_from_classes(&mut self, class_count: usize, class_base_name: &str) {
            for class in 0..class_count {
                let class_name = format!("{}{}", class_base_name, class);
                self.inner.add_hyperedge(class_name);
            }
        }

        fn add_hyperedge(&mut self, hyperedge_id : &str) {
            self.inner.add_hyperedge(hyperedge_id.to_string());
        }

        fn add_node_to_hyperedge(&mut self, student: &PyStudent, hyperedge_id: &str) {
            self.inner
                .add_node_to_hyperedge(hyperedge_id, &student.inner);
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
        #[new]
        fn new(
            id: usize,
            cronotype: u8,
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
                    cronotype,
                    ndd,
                    mi_order,
                    vark_scores : [
                        OrderedF64(vark_scores[0]),
                        OrderedF64(vark_scores[1]),
                        OrderedF64(vark_scores[2]),
                        OrderedF64(vark_scores[3]),
                    ],
                    be : OrderedF64(be),
                    ee : OrderedF64(ee),
                    ce : OrderedF64(ce),
                    autonomous_motivation : OrderedF64(autonomous_motivation),
                    competitive_motivation : OrderedF64(competitive_motivation),
                    relationship_motivation : OrderedF64(relationship_motivation),
                    gpa : OrderedF64(gpa),
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
}
