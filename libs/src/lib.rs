mod data;
mod ml;
mod utils;

#[pyo3::pymodule]
mod py_optimizer {
    use pyo3::prelude::*;
    use crate::data::hypergraph::Hypergraph;
    use crate::ml::genetics::Individual;
    use pyo3::types::PyList;
    use numpy::PyArray2;

    #[pyclass]
    pub struct GeneticAlgorithm
    {
        population_size: usize,
        generations: usize,
        mutation_rate: u8,
        crossover_rate: u8,
        pub data : StudentsData,
    }

    #[pymethods]
    impl GeneticAlgorithm {
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
            return GeneticAlgorithm {
                population_size,
                generations,
                mutation_rate,
                crossover_rate,
                data: StudentsData::new(students_data, students_vark_data, students_mi_data),
            };
        }

        pub fn initialize_population(&self, num_groups: usize) -> Vec<PyIndividual> {
            let mut population = Vec::new();
            for _ in 0..self.population_size {
                population.push(PyIndividual { inner: Individual::new(&self.data, num_groups) });
            }
            return population;
        }

        pub fn crossover(&self, ind1: &mut PyIndividual, ind2: &mut PyIndividual) -> (PyIndividual, PyIndividual) {
            println!("Crossover entre individuos con fitness {} y {}", ind1.inner.get_fitness(), ind2.inner.get_fitness());
            let (mut child1, mut child2) = ind1.inner.crossover(&mut ind2.inner, self.crossover_rate);
            child1.calculate_fitness(&self.data);
            child2.calculate_fitness(&self.data);
            println!("Fitness de los hijos después del crossover: {} y {}", child1.get_fitness(), child2.get_fitness());
            return (PyIndividual { inner: child1 }, PyIndividual { inner: child2 });
        }
    }

    #[pyclass]
    pub struct PyIndividual {
        inner: Individual
    }

    #[pymethods]
    impl PyIndividual {
        #[new]
        fn new(config : &GeneticAlgorithm, group_amount: usize) -> Self {
            return PyIndividual { inner: Individual::new(&config.data, group_amount) };
        }

        pub fn get_fitness(&self) -> f64 {
            return self.inner.get_fitness();
        }
    }

    #[pyclass]
    struct PyHypergraph {
        inner: Hypergraph
    }

    #[pymethods]
    impl PyHypergraph {
        #[new]
        fn new(students: usize) -> Self {
            return PyHypergraph {
                inner: Hypergraph::new(students),
            };
        }

        fn load_from_file(&mut self, path: String) -> PyResult<()> {
            if let Ok(hypergraph) = Hypergraph::load_from_file(&path) {
                self.inner = hypergraph;
                Ok(())
            } else {
                Err(PyErr::new::<pyo3::exceptions::PyIOError, _>("Failed to load hypergraph from file"))
            }
        }

        fn add_hyperedge(&mut self, id : String, students_idx: Vec<usize>) -> PyResult<()> {
            if let Ok(_) = self.inner.add_hyperedge(id, students_idx) {
                Ok(())
            } else {
                Err(PyErr::new::<pyo3::exceptions::PyValueError, _>("Failed to add hyperedge"))
            }
        }

        fn add_student(&mut self) -> PyResult<()> {
            if let Ok(_) = self.inner.add_student() {
                Ok(())
            } else {
                Err(PyErr::new::<pyo3::exceptions::PyValueError, _>("Failed to add student"))
            }
        }
    }
}
