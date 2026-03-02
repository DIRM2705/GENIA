use polars_core::prelude::*;
use polars_lazy::prelude::*;
use numpy::ndarray::*;
use symmetric_matrix::SymmetricMatrix;

pub fn calculate_gower_matrix(df: DataFrame) -> SymmetricMatrix {
    //Calculate Gower distance between two students
    let student_count = df.height() as usize;
    let lazy_df = df.clone().lazy();

    let mut distance_matrix = SymmetricMatrix::new(student_count);
    let ranks = lazy_df.clone()
        .select(&[all().exclude_cols(["Id"]).as_expr()])
        .select([col("*").max() - col("*").min()])
        .collect().unwrap().to_ndarray::<Float64Type>(IndexOrder::C).unwrap();

    for i in 0..student_count {
        for j in i+1..student_count {
            let students = lazy_df.clone()
                .filter(col("Id").eq(i as u32).or(col("Id").eq(j as u32))).collect().unwrap();

            let distance = calculate_distance_numerical(&students, &ranks);
            distance_matrix.set(i, j, distance);
        }
    }

    return distance_matrix;
}

fn calculate_distance_numerical(clean_frame : &DataFrame, ranks: &Array2<f64>) -> f64 {
    //Get gower similarity bewteen two numerical values
    let mut distance = 0.0;
    let matrix = clean_frame.to_ndarray::<Float64Type>(IndexOrder::C).unwrap();
    for col in 1.. matrix.ncols()
    {
        let s1_value = matrix[[0, col]];
        let s2_value = matrix[[1, col]];
        let rank = ranks[[0, col-1]]; // Assuming ranks is a 2D array with shape (n_cols, 1)
        let diff : f64 = (s1_value - s2_value).abs();
        distance += 1.0 - diff / rank;
    }
    return distance/clean_frame.width() as f64;
}