use std::collections::HashSet;

pub struct Hypergraph 
{
    hyperedges : Vec<Hyperedge>,
}

pub struct Hyperedge
{
    name_id : String,
    nodes : HashSet<Node>,
}

pub struct Node
{
    id : usize
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
    pub fn new(name_id: String, nodes: HashSet<Node>) -> Self 
    {
        Hyperedge 
        {
            name_id,
            nodes,
        }
    }
}