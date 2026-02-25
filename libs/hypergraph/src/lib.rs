use std::collections::{HashSet, HashMap};

pub struct Hypergraph 
{
    nodes : HashMap<usize, Student>,
    hyperedges : HashMap<String, HashSet<usize>>,
}

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
    Autism,
    Dislexia,
    Disgrafia,
    Discalculia,
    MIKin(u8),
    MIExis(u8),
    MIInter(u8),
    MIIntra(u8),
    MILog(u8),
    MIMus(u8),
    MINat(u8),
    MIVer(u8),
    MIVis(u8),
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

    fn get_hyperedge_name(&mut self, characteristic : &CharacteristicType) -> String
    {
        return match characteristic 
        {
            CharacteristicType::Chronotype(value) => format!("Chronotype_{}", value),
            CharacteristicType::Disgrafia => format!("Disgrafia"),
            CharacteristicType::Discalculia =>format!("Discalculia"),
            CharacteristicType::ADHD => format!("ADHD"),
            CharacteristicType::Autism => format!("Autism"),
            CharacteristicType::MIKin(value) => format!("MIKin_{}", value),
            CharacteristicType::MIExis(value) => format!("MIExis_{}", value),
            CharacteristicType::MIInter(value) => format!("MIInter_{}", value),
            CharacteristicType::MIIntra(value) => format!("MIIntra_{}", value),
            CharacteristicType::MILog(value) => format!("MILog_{}", value),
            CharacteristicType::MIMus(value) => format!("MIMus_{}", value),
            CharacteristicType::MINat(value) => format!("MINat_{}", value),
            CharacteristicType::MIVer(value) => format!("MIVer_{}", value),
            CharacteristicType::MIVis(value) => format!("MIVis_{}", value),
            CharacteristicType::VarkVisual(value) => format!("VISUAL_{}", value),
            CharacteristicType::VarkAural(value) => format!("AURAL_{}", value),     
            CharacteristicType::VarkRW(value) => format!("RW_{}", value),
            CharacteristicType::VarkKinesthetic(value) => format!("KINESTHETIC_{}", value),
            CharacteristicType::BE(value) => format!("BE_{}", value),
            CharacteristicType::EE(value) => format!("EE_{}", value),
            CharacteristicType::CE(value) => format!("CE_{}", value),
            CharacteristicType::AM(value) => format!("AM_{}", value),
            CharacteristicType::CM(value) => format!("CM_{}", value),
            CharacteristicType::RM(value) => format!("RM_{}", value),
            _ => String::from("")
        }
    }

    pub fn add_student_to_characteristic(&mut self, characteristic : &CharacteristicType, student_id : usize)
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