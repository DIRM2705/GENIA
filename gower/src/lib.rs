use numpy::PyArray2;
use pyo3_polars::PyDataFrame;
use pyo3::prelude::*;
use polars::prelude::*;

#[pyfunction]
fn gower(pydf: PyDataFrame) -> PyResult<()> {
    /*
        Gower distance calculation
        ARGS:
            df: DataFrame that includes both numerical and categorical variables
        RETURNS:
            A 2D numpy array with Gower distances
     */
    let students_data: DataFrame = pydf.into(); // Convert PyDataFrame to Polars DataFrame
    let columns = students_data.get_columns();
    println!("Columns: {:?}", columns);
    Ok(())
}