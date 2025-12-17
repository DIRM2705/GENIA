use polars::prelude::*;

pub struct StudentData {
    numerical_df: DataFrame,
    categorical_df: DataFrame,
    pub category_count : f64,
    pub rows: usize,
    pub ranks: Vec<f64>,
}

impl StudentData {
    pub fn new(data_frame: DataFrame) -> Self {
        let row_count = data_frame.height(); //Number of students
        let category_count = data_frame.width() as f64; //Number of columns

        let mut numerical_names : Vec<Expr> = Vec::new();
        let mut categorical_names : Vec<Expr> = Vec::new();

        for column in data_frame.get_columns() {
            match column.dtype() {
                DataType::Float64 => {
                    numerical_names.push(col(column.name().to_string()));
                }
                DataType::Categorical(_, _) => {
                    categorical_names.push(col(column.name().to_string()));
                }
                _ => {}
            }
        }

        //Separate numerical and categorical columns
        let numerical = data_frame.clone()
            .lazy().select(numerical_names).collect().unwrap();
        let categorical = data_frame.clone()
            .lazy()
            .select(categorical_names).collect().unwrap();

        drop(data_frame); //Release the original DataFrame

        //Compute ranks for numerical columns
        let mut ranks: Vec<f64> = Vec::with_capacity(numerical.width());
        for i in 0..numerical.width() {
            
            let column = numerical.get_columns().get(i).unwrap();
            let min_value = column.f64().unwrap().min().unwrap_or_default();
            let max_value = column.f64().unwrap().max().unwrap_or_default();
            let rank = max_value - min_value;
            ranks.push(rank);
        }

        //Return the StudentData instance
        return StudentData {
            rows: row_count,
            category_count: category_count,
            numerical_df: numerical,
            categorical_df: categorical,
            ranks: ranks
        };
    }

    pub fn get_row_numerical(&self, student_idx: usize) -> Vec<f64> 
    {
        //Create a vector to hold the row values
        let mut row_values: Vec<f64> = Vec::with_capacity(self.numerical_df.width());
        //Get the row from the numerical DataFrame
        let row = self.numerical_df.get_row(student_idx).unwrap().0;
        for cell in row {
            row_values.push(cell.try_extract().unwrap_or_default()); //Convert cell to f64 and push to vector
        }
        return row_values;
    }

    pub fn get_row_categorical(&self, student_idx: usize) -> Vec<String> 
    {
        //Create a vector to hold the row values
        let mut row_values: Vec<String> = Vec::with_capacity(self.categorical_df.width());
        //Get the row from the categorical DataFrame
        let row = self.categorical_df.get_row(student_idx).unwrap().0;
        for cell in row {
            row_values.push(cell.to_string()); //Convert cell to String and push to vector
        }
        return row_values;
    }
}