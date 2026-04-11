use pyo3::types::PyList;
use pyo3::prelude::{Py, PyAnyMethods, PyListMethods};
use pyo3::Python;
use numpy::PyArray2;

pub struct StudentsData {
    students_data: Py<PyArray2<f64>>,
    students_vark_data: Py<PyList>,
    students_mi_data: Py<PyList>,
}

impl StudentsData {
    pub fn new(
        students_data: Py<PyArray2<f64>>,
        students_vark_data: Py<PyList>,
        students_mi_data: Py<PyList>,
    ) -> Self {
        return StudentsData {
            students_data,
            students_vark_data,
            students_mi_data,
        };
    }

    pub fn get_student_count(&self) -> usize {
        return Python::attach(|py| {
            let bind = self.students_vark_data.bind(py);
            let count = bind.len();
            return count as usize;
        });
    }

    pub fn get_vark_rows(&self, i: &Vec<usize>) -> Vec<Vec<u8>> {
        return Python::attach(|py| {
            let bind = self.students_vark_data.bind(py);
            let mut rows = Vec::new();
            for idx in i {
                let row_unconverted = bind.get_item(*idx).unwrap();
                let row = row_unconverted.extract().unwrap();
                rows.push(row);
            }
            return rows;
        });
    }

    pub fn get_mi_rows(&self, i: &Vec<usize>) -> Vec<Vec<u8>> {
        return Python::attach(|py| {
            let bind = self.students_mi_data.bind(py);
            let mut rows = Vec::new();
            for idx in i {
                let row_unconverted = bind.get_item(*idx).unwrap();
                let row = row_unconverted.extract().unwrap();
                rows.push(row);
            }
            return rows;
        });
    }
}
