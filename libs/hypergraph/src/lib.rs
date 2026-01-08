use std::collections::HashSet;

pub struct Hypergraph 
{
    hyperedges : Vec<Hyperedge>,
}

pub struct Hyperedge
{
    name_id : String,
    nodes : HashSet<Student>,
}

pub struct Student
{
    pub id : usize,
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
    pub fn new() -> Self 
    {
        //Create an empty hypergraph
        Hypergraph 
        {
            hyperedges : Vec::new(),
        }
    }

    pub fn add_hyperedge(&mut self, hyperedge: Hyperedge) 
    {
        self.hyperedges.push(hyperedge);
    }
}

impl Hyperedge
{
    //Create an empty hyperedge with a given name_id
    pub fn new_empty(name_id: String) -> Self 
    {
        Hyperedge 
        {
            name_id,
            nodes : HashSet::new(),
        }
    }

    //Create a hyperedge with a given name_id and set of nodes
    pub fn new(name_id: String, nodes: HashSet<Student>) -> Self 
    {
        Hyperedge 
        {
            name_id,
            nodes,
        }
    }
}