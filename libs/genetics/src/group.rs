use polars::prelude::*;
use polars::lazy::dsl::sum_horizontal;

pub struct Group {
    student_data: DataFrame,
}

impl Group {
    pub fn new(student_data: DataFrame) -> Self {
        return Group { student_data };
    }

    pub fn calculate_discartability(&self) -> f32 {
        let mi_weight = self.calculate_mi_weight();
        let vark_weight = self.calculate_vark_weight();
        let motivations_weight = self.calculate_motivations_weight();
        let engagement_weight = self.calculate_engagement_weight();

        println!(
            "MI weight: {}, VARK weight: {}, Motivations weight: {}, Engagement weight: {}",
            mi_weight, vark_weight, motivations_weight, engagement_weight
        );

        return (mi_weight + vark_weight + motivations_weight + engagement_weight) as f32;
    }

    fn calculate_mi_weight(&self) -> f32 {
        // This factor can be adjusted based on how much you want to weight entropy for secundary intelligences
        const SECOND_TERM_FACTOR: f32 = 0.15;

        let mi_df = self.student_data.clone().lazy().select([col("^MI.*$")]);

        let nombres = mi_df
            .clone()
            .collect_schema()
            .unwrap()
            .iter_names()
            .map(|s| s.to_string())
            .collect::<Vec<String>>();

        let mut mi_principal: LazyFrame = mi_df.clone();
        for nombre in &nombres {
            mi_principal = mi_principal.with_columns([when(col(nombre).neq(lit(1)))
                .then(lit(0))
                .otherwise(col(nombre))
                .sum()
                .alias(nombre)]);
        }

        mi_principal =
            mi_principal.with_columns([sum_horizontal([col("*")], true).unwrap().alias("total")]);

        let mut mi_secondary: LazyFrame = mi_df.clone();
        for nombre in &nombres {
            mi_secondary = mi_secondary.with_columns([when(col(nombre).gt(lit(2)))
                .then(lit(0))
                .otherwise(lit(1))
                .sum()
                .alias(nombre)]);
        }

        mi_secondary =
            mi_secondary.with_columns([sum_horizontal([col("*")], true).unwrap().alias("total")]);

        let mi_weight = (1.0 - SECOND_TERM_FACTOR)
            * calculate_entropy(&mi_principal.collect().unwrap())
            + SECOND_TERM_FACTOR * calculate_entropy(&mi_secondary.collect().unwrap());

        return mi_weight; //If the hyperedge doesn't exist, return a default weight of 0.0
    }

    fn calculate_vark_weight(&self) -> f32 {
        // This factor can be adjusted based on how much you want to weight entropy for secundary characteristics
        const SECOND_TERM_FACTOR: f32 = 0.15;

        let vark_df = self.student_data.clone().lazy().select([col("^VARK.*$")]);

        let names = vark_df
            .clone()
            .collect_schema()
            .unwrap()
            .iter_names()
            .map(|s| s.to_string())
            .collect::<Vec<String>>();

        let mut vark_principal: LazyFrame = vark_df.clone();
        for name in &names {
            vark_principal = vark_principal.with_columns([when(col(name).neq(lit(1)))
                .then(lit(0))
                .otherwise(col(name))
                .sum()
                .alias(name)]);
        }

        vark_principal =
            vark_principal.with_columns([sum_horizontal([col("*")], true).unwrap().alias("total")]);

        let mut vark_secondary: LazyFrame = vark_df.clone();
        for name in &names {
            vark_secondary = vark_secondary.with_columns([when(col(name).gt(lit(1)))
                .then(lit(0))
                .otherwise(lit(1))
                .sum()
                .alias(name)]);
        }

        vark_secondary =
            vark_secondary.with_columns([sum_horizontal([col("*")], true).unwrap().alias("total")]);

        let vark_weight = (1.0 - SECOND_TERM_FACTOR)
            * calculate_entropy(&vark_principal.collect().unwrap())
            + SECOND_TERM_FACTOR * calculate_entropy(&vark_secondary.collect().unwrap());

        return vark_weight; // Return the calculated VARK weight
    }

    fn calculate_motivations_weight(&self) -> f32 {
        let motivations_df = self.student_data.clone().lazy().select([col("^.*Motiv$")]);

        let names = motivations_df
            .clone()
            .collect_schema()
            .unwrap()
            .iter_names()
            .map(|s| s.to_string())
            .collect::<Vec<String>>();

        let mut weight = 0.0;

        for name in &names {
            let cv = motivations_df
                .clone()
                .select([col(name).std(1) / mean(name)])
                .collect()
                .unwrap()
                .get(0)
                .unwrap()[0]
                .try_extract::<f32>()
                .unwrap();

            //Scale CV to give one point if data is widely heterogeneous,
            //or points relative for the homogeneous threeshold (0.3)
            let truncated_cv = if cv > 0.3 { 1.0 } else { 0.3 - cv };
            weight += truncated_cv;
        }

        return weight; // Return the calculated motivation weight
    }

    fn calculate_engagement_weight(&self) -> f32 {
        let engagement_df = self.student_data.clone().lazy().select([col("^.*Engage$")]);

        let names = engagement_df
            .clone()
            .collect_schema()
            .unwrap()
            .iter_names()
            .map(|s| s.to_string())
            .collect::<Vec<String>>();

        let mut weight = 0.0;

        for name in &names {
            let cv = engagement_df
                .clone()
                .select([col(name).std(1) / mean(name)])
                .collect()
                .unwrap()
                .get(0)
                .unwrap()[0]
                .try_extract::<f32>()
                .unwrap();

            //Scale CV to give one point if data is widely heterogeneous,
            //or points relative for the homogeneous threeshold (0.3)
            let truncated_cv = if cv > 0.3 { 1.0 } else { 0.3 - cv };
            weight += truncated_cv;
        }

        return weight; // Return the calculated engagement weight
    }

    fn print(&self) {
        println!("Student data:\n{:?}", self.student_data);
    }
}

fn calculate_entropy(data: &DataFrame) -> f32 {
    let array = data.to_ndarray::<Float32Type>(IndexOrder::C).unwrap();
    let total = array[[0, array.ncols() - 1]]; // Get the total from the last column

    let mut entropy = 0.0;

    for i in 0..array.ncols() - 1 {
        let p_i: f32 = array[[0, i]] / total; // Calculate the probability for the i-th characteristic

        if p_i > 0.0 {
            entropy -= p_i * p_i.log2();
        }
    }

    return entropy;
}
