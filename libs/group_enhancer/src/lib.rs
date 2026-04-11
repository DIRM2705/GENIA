#[pyo3::pymodule]
mod group_enhancer {
    use hypergraph::Hypergraph;
    use pyo3::prelude::*;
    use pyo3::types::PyList;
    use numpy::PyArray2;
    use symmetric_matrix::SymmetricMatrix;
    use genetics::Individual;
    use genetics::student_data::StudentsData;

    #[pyclass]
    pub struct GeneticAlgorithmConfig
    {
        population_size: usize,
        generations: usize,
        mutation_rate: u8,
        crossover_rate: u8,
        pub data : StudentsData,
    }

    #[pymethods]
    impl GeneticAlgorithmConfig {
        #[new]
        pub fn new(
            population_size: usize,
            generations: usize,
            mutation_rate: u8,
            crossover_rate: u8,
            students_data: Py<PyArray2<f64>>,
            students_vark_data: Py<PyList>,
            students_mi_data: Py<PyList>
        ) -> Self {
            return GeneticAlgorithmConfig {
                population_size,
                generations,
                mutation_rate,
                crossover_rate,
                data: StudentsData::new(students_data, students_vark_data, students_mi_data),
            };
        }
    }

    #[pyclass]
    pub struct PyIndividual {
        inner: Individual,
    }

    #[pymethods]
    impl PyIndividual {
        #[new]
        fn new(config : &GeneticAlgorithmConfig, group_amount: usize) -> Self {
            return PyIndividual { inner: Individual::new(&config.data, group_amount) };
        }

        pub fn crossover(&mut self, other: &mut PyIndividual, config: &GeneticAlgorithmConfig) -> (PyIndividual, PyIndividual) {
            println!("Crossover entre individuos con fitness {} y {}", self.inner.get_fitness(), other.inner.get_fitness());
            let (mut child1, mut child2) = self.inner.crossover(&mut other.inner, config.crossover_rate);
            child1.calculate_fitness(&config.data);
            child2.calculate_fitness(&config.data);
            println!("Fitness de los hijos después del crossover: {} y {}", child1.get_fitness(), child2.get_fitness());
            return (PyIndividual { inner: child1 }, PyIndividual { inner: child2 });
        }

        pub fn get_fitness(&self) -> f32 {
            return self.inner.get_fitness();
        }
    }

    #[pyclass]
    struct CharacteristicHG {
        hypergraph: Vec<u64>,
        mi_matrix: SymmetricMatrix,
        vark_matrix: SymmetricMatrix,
        am_matrix: SymmetricMatrix,
        rm_matrix: SymmetricMatrix,
        cm_matrix: SymmetricMatrix,
        be_matrix: SymmetricMatrix,
        ee_matrix: SymmetricMatrix,
        ce_matrix: SymmetricMatrix,
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
