use numpy::Ix1;
use numpy::ndarray::Array;

pub struct Hypergraph 
{
    matrix : Array<u128, Ix1>,
    hyperedges : Vec<String>,
}


impl Hypergraph 
{
    pub fn new(num_students : usize) -> Self 
    {
        //Create an empty hypergraph
        Hypergraph 
        {
            matrix : Array::zeros(num_students),
            hyperedges : Vec::new()
        }
    }

    fn get_hyperedge_index(&mut self, id : &String) -> usize
    {
        //Get the index of the hyperedge corresponding to the characteristic, adding it if it doesn't exist
        if let Some(index) = self.hyperedges.iter().position(|x| x == id) 
        {
            return index;
        }
        else 
        {
            self.hyperedges.push(id.clone());
            return self.hyperedges.len() - 1;
        }
    }

    fn get_hyperedge_name(&mut self, hyperedge_idx : usize) -> String
    {
        return self.hyperedges[hyperedge_idx].clone();
    }

    pub fn add_student_to_characteristic(&mut self, characteristic : &String, student_id : usize)
    {
        //Ensure the characteristic exists in the hypergraph
        let hyperedge_idx = self.get_hyperedge_index(characteristic) ;
        //Update the matrix to indicate the student has this characteristic
        self.matrix[[student_id]] |= 1 << (hyperedge_idx + 1);
    }

    pub fn print(&self) 
    {
        //Print the hypergraph in a readable format
        for (i, hyperedge) in self.hyperedges.iter().enumerate() 
        {
            println!("Hyperedge {}: {}", i, hyperedge);
            for (j, student) in self.matrix.iter().enumerate() 
            {
                if student & (1 << (i + 1)) != 0 
                {
                    println!("  Student {}", j);
                }
            }
        }
    }
}