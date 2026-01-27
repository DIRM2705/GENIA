use std::collections::{HashSet, HashMap};
use polars::prelude::*;

pub struct Hypergraph 
{
    nodes : DataFrame,
    hyperedges : HashMap<String, HashSet<usize>>,
}

pub struct Student
{
    pub id : usize,
    pub cronotype: u8, //Chronotype bit map
    pub ndd: u8, //Bit map for neurodevelopmental disorders 
    pub mi_order: [u8; 8], //Order of multiple intelligences
    pub vark_scores: [f64; 4], //VARK learning style scores
    pub be : f64, //Behavioral engagement percentage
    pub ee: f64, //Emotional engagement percentage
    pub ce : f64, //Cognitive engagement percentage
    pub autonomous_motivation : f64, //Autonomous motivation percentage
    pub competitive_motivation : f64, //Competitive motivation percentage
    pub relationship_motivation : f64, //Relationship motivation percentage
    pub gpa : f64,
}


impl Hypergraph 
{
    pub fn new(nodes : DataFrame) -> Self 
    {
        //Create an empty hypergraph
        Hypergraph 
        {
            nodes,
            hyperedges : HashMap::new(),
        }
    }

    pub fn add_hyperedge(&mut self, hyperedge_id : String) 
    {

        self.hyperedges.insert(hyperedge_id, HashSet::new());
    }

    pub fn add_student_to_hyperedge(&mut self, hyperedge_id: &str, student_id : usize) -> Result<(), String>
    {
        if self.nodes.get(student_id).is_none() 
        {
            return Err(format!("Student with id {} does not exist in the hypergraph", student_id).into());
        } 

        self.hyperedges.get_mut(hyperedge_id).unwrap().insert(student_id);
        return Ok(());
    }

    pub fn print(&self) 
    {
        for (hyperedge_id, node_idxs) in self.hyperedges.iter() 
        {
            println!("Hyperedge: {}", hyperedge_id);
            for node in node_idxs.iter()
            {
                println!(" - Student ID: {}", node);
            }
        }
    }
}