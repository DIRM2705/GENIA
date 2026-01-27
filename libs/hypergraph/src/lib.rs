use std::collections::{HashSet, HashMap};

pub struct Hypergraph 
{
    nodes : HashMap<usize, Student>,
    hyperedges : HashMap<String, HashSet<usize>>,
}

/*pub struct Student
{
    pub cronotype: u8, //Chronotype class
    pub ndd: u8, //Bit map for neurodevelopmental disorders 
    pub mi_classes: [u8; 3], //Top three MI
    pub vark_classes: [f64; 4], //VARK learning style classes
    pub be_class : u8, //Behavioral engagement class
    pub ee_class: u8, //Emotional engagement class
    pub ce_class : u8, //Cognitive engagement class
    pub autonomous_motivation_class : u8, //Autonomous motivation class
    pub competitive_motivation_class : u8, //Competitive motivation class
    pub relationship_motivation_class : u8, //Relationship motivation class
}*/

#[derive(Clone)]
pub struct Student
{
    characteristics : Vec<CharacteristicType>
}

impl Student 
{
    pub fn new() -> Self 
    {
        Student 
        {
            characteristics : Vec::new()
        }
    }

    pub fn add_characteristic (&mut self, characteristic : CharacteristicType) 
    {
        self.characteristics.push(characteristic);
    }
}

#[derive(Clone)]
pub enum CharacteristicType
{
    Chronotype(u8),
    ADHD,
    AUTSIM,
    DISLEXIA,
    DISGRAFIA,
    DISCALCULIA,
    MI1(String),
    MI2(String),
    MI3(String),    
    VarkVisual(u8),
    VarkAural(u8),
    VarkRW(u8),
    VarkKinesthetic(u8),
    BE(u8),
    EE(u8),
    CE(u8),
    AM(u8),
    CM(u8),
    RM(u8)
}


impl Hypergraph 
{
    pub fn new() -> Self 
    {
        //Create an empty hypergraph
        Hypergraph 
        {
            nodes : HashMap::new(),
            hyperedges : HashMap::new(),
        }
    }

    fn get_hyperedge_name(&mut self, characteristic : CharacteristicType) -> String
    {
        return match characteristic 
        {
            CharacteristicType::Chronotype(value) => format!("Chronotype_{}", value),
            CharacteristicType::DISGRAFIA => format!("DISGRAFIA"),
            CharacteristicType::DISCALCULIA =>format!("DISCALCULIA"),
            CharacteristicType::ADHD => format!("ADHD"),
            CharacteristicType::AUTSIM => format!("AUTSIM"),
            CharacteristicType::MI1(value) => format!("{}1", value),
            CharacteristicType::MI2(value) => format!("{}2", value),
            CharacteristicType::MI3(value) => format!("{}3", value),
            CharacteristicType::VarkVisual(value) => format!("VISUAL{}", value),
            CharacteristicType::VarkAural(value) => format!("AURAL{}", value),     
            CharacteristicType::VarkRW(value) => format!("RW{}", value),
            CharacteristicType::VarkKinesthetic(value) => format!("KINESTHETIC{}", value),
            CharacteristicType::BE(value) => format!("BE{}", value),
            CharacteristicType::EE(value) => format!("EE{}", value),
            CharacteristicType::CE(value) => format!("CE{}", value),
            CharacteristicType::AM(value) => format!("AM{}", value),
            CharacteristicType::CM(value) => format!("CM{}", value),
            CharacteristicType::RM(value) => format!("RM{}", value),
            _ => String::from("")
        }
    }

    pub fn add_student_to_characteristic(&mut self, characteristic : CharacteristicType, student_id : usize)
    {
        if !self.nodes.contains_key(&student_id)
        {
            self.nodes.insert(student_id, Student::new());
        }
        let student = self.nodes.get_mut(&student_id).unwrap();
        student.add_characteristic(characteristic.clone()); //Update the student's characteristics

        let hyperedge_id = self.get_hyperedge_name(characteristic);
        if !self.hyperedges.contains_key(&hyperedge_id)
        {
            self.hyperedges.insert(hyperedge_id.clone(), HashSet::new());
        }
        let nodes = self.hyperedges.get_mut(&hyperedge_id).unwrap();
        nodes.insert(student_id); //Update the hyperedge's nodes
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