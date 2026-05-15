use pyo3::types::PyList;
use pyo3::prelude::{Py, PyAnyMethods, PyListMethods};
use pyo3::Python;
use numpy::{PyArray2, PyArrayMethods};

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

    pub fn get_group_data(&self, i : &Vec<usize>) -> Vec<Vec<f64>> {
        return Python::attach(|py| {
            let bind = self.students_data.bind(py);
            
            let array_unconverted = bind.readonly();
            let array = array_unconverted.as_array();
            let mut new_array = Vec::new();
            for col in array.columns() {
                let mut col_total = 0.0;
                let mut new_col = Vec::new();

                //For each student in the group, we add their value in the column to the new column,
                //and we also keep track of the total of the column for the students in the group
                for idx in i {
                    new_col.push(col[*idx]);
                    col_total += col[*idx];
                }

                //Save the accumulative apportations of the students in the group to the column,
                //normalized by the total of the column for the students in the group,
                //so we can calculate the GINI coefficient later
                new_col[0] /= col_total;
                for idx in 1..new_col.len() {
                    new_col[idx] = (new_col[idx] / col_total) + new_col[idx - 1];
                }
                new_array.push(new_col);
            }
            return new_array;
        });
    }
}