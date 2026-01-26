use std::collections::{HashSet, HashMap, BTreeMap};
use ordered_f64::OrderedF64;

pub struct Hypergraph 
{
    nodes : BTreeMap<usize, Student>,
    hyperedges : HashMap<String, HashSet<usize>>,
}

#[derive(Hash, Eq, PartialEq, Clone)]
pub struct Student
{
    pub id : usize,
    pub cronotype: u8, //Chronotype bit map
    pub ndd: u8, //Bit map for neurodevelopmental disorders 
    pub mi_order: [u8; 8], //Order of multiple intelligences
    pub vark_scores: [OrderedF64; 4], //VARK learning style scores
    pub be : OrderedF64, //Behavioral engagement percentage
    pub ee: OrderedF64, //Emotional engagement percentage
    pub ce : OrderedF64, //Cognitive engagement percentage
    pub autonomous_motivation : OrderedF64, //Autonomous motivation percentage
    pub competitive_motivation : OrderedF64, //Competitive motivation percentage
    pub relationship_motivation : OrderedF64, //Relationship motivation percentage
    pub gpa : OrderedF64,
}


impl Hypergraph 
{
    pub fn new() -> Self 
    {
        //Create an empty hypergraph
        Hypergraph 
        {
            nodes : BTreeMap::new(),
            hyperedges : HashMap::new(),
        }
    }

    pub fn add_hyperedge(&mut self, hyperedge_id : String) 
    {

        self.hyperedges.insert(hyperedge_id, HashSet::new());
    }

    pub fn add_node_to_hyperedge(&mut self, hyperedge_id: &str, student : &Student) 
    {
        if self.nodes.get(&student.id).is_none() 
        {
            self.nodes.insert(student.id, student.clone());
        } 

        self.hyperedges.get_mut(hyperedge_id).unwrap().insert(student.id);
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