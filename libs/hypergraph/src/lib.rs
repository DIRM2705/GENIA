use std::collections::{HashSet, HashMap};

#[allow(private_bounds)]
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

pub struct Hypergraph<T : Integer + Copy + Clone>
{
    matrix : Vec<T>,
    hyperedges : HashMap<String, HashSet<usize>>
}
    

impl<T : Integer + Copy + Clone> Hypergraph<T>
{
    pub fn new(num_students : usize) -> Self 
    {
        //Create an empty hypergraph
        Hypergraph 
        {
            matrix : vec![T::zero(); num_students],
            hyperedges : HashMap::new(),
        }
    }

    pub fn len(&self) -> usize
    {
        return self.hyperedges.len();
    }

    pub fn add_id_to_hyperedge(&mut self, id : usize, hyperedge : &String)
    {
        //Ensure the characteristic exists in the hypergraph
        if !self.hyperedges.contains_key(hyperedge)
        {
            self.hyperedges.insert(hyperedge.clone(), HashSet::new()); //Add a new hyperedge for the new characteristic
        }


        let hyperedge = self.hyperedges.get_mut(hyperedge).unwrap();
        //Update the matrix to indicate the student has this characteristic
        hyperedge.insert(id);
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