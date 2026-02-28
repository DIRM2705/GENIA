use std::collections::{HashSet, HashMap};
use polars_core::prelude::*;
use polars_lazy::prelude::*;

trait Integer
{
    fn zero() -> Self;
}

impl Integer for u32{
    fn zero() -> Self {
        return 0;
    }
}
impl Integer for u64{
    fn zero() -> Self {
        return 0;
    }
}
impl Integer for u128{
    fn zero() -> Self {
        return 0;
    }
}

pub trait WeightedHypergraph
{
    fn calculate_hyperedge_weight(&self, hyperedge : &String) -> f64;
}

#[allow(private_bounds)]
pub struct Hypergraph<T : Integer + Copy + Clone>
{
    matrix : Vec<T>,
    hyperedges : HashMap<String, (HashSet<usize>, f64)>,
} 

#[allow(private_bounds)]
impl<T : Integer + Copy + Clone> Hypergraph<T>
{
    pub fn new(num_nodes : usize) -> Self 
    {
        //Create an empty hypergraph
        Hypergraph 
        {
            matrix : vec![T::zero(); num_nodes],
            hyperedges : HashMap::new(),
        }
    }

    pub fn len(&self) -> usize
    {
        return self.hyperedges.len();
    }

    pub fn add_students_to_hyperedge(&mut self, students : DataFrame, hyperedge : &String)
    {
        //Ensure the characteristic exists in the hypergraph
        if !self.hyperedges.contains_key(hyperedge)
        {
            self.hyperedges.insert(hyperedge.clone(), (HashSet::new(), self.calculate_hyperedge_weight(&students))); //Add a new hyperedge for the new characteristic with weight 0.0
        }


        let hyperedge = self.hyperedges.get_mut(hyperedge).unwrap();
        //Update the matrix to indicate the student has this characteristic
        for id in students["Id"].u32().unwrap().into_iter() {
            hyperedge.0.insert(id.unwrap_or(0) as usize);
        }
    }

    fn calculate_hyperedge_weight(&self, data : &DataFrame) -> f64
    {
        let MI_weight = self.calculate_MI_weight(&data);
        let VARK_weight = self.calculate_VARK_weight(&data);
        let motivations_weight = self.calculate_motivations_weight(&data);
        let weight = MI_weight + VARK_weight + motivations_weight; //Here you can combine the MI weight with other factors to calculate the final weight of the hyperedge

        println!("Calculated weight for hyperedge: {}", weight);
        return weight; //If the hyperedge doesn't exist, return a default weight of 0.0
    }

    fn calculate_MI_weight(&self, data : &DataFrame) -> f64
    {
        // This factor can be adjusted based on how much you want to weight entropy for secundary intelligences
        const second_term_factor : f64 = 0.15; 

        let MI_df = data.clone().lazy().select(
            [col("^MI.*$")]
        );

        let nombres = MI_df.clone().collect_schema().unwrap().iter_names().map(|s| s.to_string()).collect::<Vec<String>>();

        let mut MI_principal : LazyFrame = MI_df.clone();
        for nombre in &nombres {
            MI_principal = MI_principal.with_columns(
            [when(col(nombre).neq(lit(1)))
            .then(lit(0))
            .otherwise(col(nombre))
            .sum()
            .alias(nombre)
            ]);
        }

        MI_principal = MI_principal.with_columns(
            [sum_horizontal([col("*")], true).unwrap().alias("total")]
        );

        let mut MI_secondary : LazyFrame = MI_df.clone();
        for nombre in &nombres {
            MI_secondary = MI_secondary.with_columns(
            [when(col(nombre).gt(lit(2)))
            .then(lit(0))
            .otherwise(lit(1))
            .sum()
            .alias(nombre)
            ]);
        }

        MI_secondary = MI_secondary.with_columns(
            [sum_horizontal([col("*")], true).unwrap().alias("total")]
        );

        let mi_weight = (1.0 - second_term_factor) * self.calculate_entropy(&MI_principal.collect().unwrap()) + second_term_factor * self.calculate_entropy(&MI_secondary.collect().unwrap());

        return mi_weight; //If the hyperedge doesn't exist, return a default weight of 0.0
    }

    fn calculate_VARK_weight(&self, data : &DataFrame) -> f64
    {
        // This factor can be adjusted based on how much you want to weight entropy for secundary characteristics
        const second_term_factor : f64 = 0.15;

        let VARK_df = data.clone().lazy().select(
            [col("^VARK.*$")]
        );

        let names = VARK_df.clone().collect_schema().unwrap().iter_names().map(|s| s.to_string()).collect::<Vec<String>>();

        let mut VARK_principal : LazyFrame = VARK_df.clone();
        for name in &names {
            VARK_principal = VARK_principal.with_columns(
            [when(col(name).neq(lit(1)))
            .then(lit(0))
            .otherwise(col(name))
            .sum()
            .alias(name)
            ]);
        }

        VARK_principal = VARK_principal.with_columns(
                [sum_horizontal([col("*")], true).unwrap().alias("total")]
            );

        let mut VARK_secondary : LazyFrame = VARK_df.clone();
        for name in &names {
            VARK_secondary = VARK_secondary.with_columns(
            [when(col(name).gt(lit(1)))
            .then(lit(0))
            .otherwise(lit(1))
            .sum()
            .alias(name)
            ]);
        }

        VARK_secondary = VARK_secondary.with_columns(
                [sum_horizontal([col("*")], true).unwrap().alias("total")]
            );

        let vark_weight = (1.0 - second_term_factor) * self.calculate_entropy(&VARK_principal.collect().unwrap()) + second_term_factor * self.calculate_entropy(&VARK_secondary.collect().unwrap());

        return vark_weight; // Return the calculated VARK weight
    }

    fn calculate_entropy(&self, data : &DataFrame) -> f64
    {
        let array = data.to_ndarray::<Float64Type>(IndexOrder::C).unwrap();
        let total = array[[0, array.ncols() - 1]]; // Get the total from the last column

        let mut entropy = 0.0;
        
        for i in 0..array.ncols() - 1 {
            let p_i : f64 = array[[0, i]] / total; // Calculate the probability for the i-th characteristic

            if p_i > 0.0 {
                entropy -= p_i * p_i.log2();
            }
        }

        println!("Calculated entropy: {}", entropy);

       return entropy;
    }

    fn calculate_motivations_weight(&self, data : &DataFrame) -> f64
    {
        let motivations_df = data.clone().lazy().select(
            [col("^.*Motiv$")]
        );

        let names = motivations_df.clone().collect_schema().unwrap().iter_names().map(|s| s.to_string()).collect::<Vec<String>>();

        for name in &names {
            let mean = motivations_df.clone().select([col(name).std(1)/mean(name)]).collect().unwrap();
            println!("Mean for {}: {:?}", name, mean);
        }

        return 0.0; // Return the calculated motivation weight
    }

    pub fn print(&self) 
    {
        //Print the hypergraph in a readable format
        for (characteristic, students) in &self.hyperedges 
        {
            println!("Characteristic: {}, Students: {:?}", characteristic, students);
        }
    }
}