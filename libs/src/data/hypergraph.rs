use crate::utils::bitmap;

use bitmap::BitmapLen;
use serde::{Deserialize, Serialize};
use std::collections::HashMap;
use std::fs::File;
use std::io::{Read, Write};

#[derive(Serialize, Deserialize)]
pub struct Hyperedge {
    bitmap: BitmapLen, // Bitmap representing the students in the hyperedge
    id: String,        // Identifier for the hyperedge
}

impl Hyperedge {
    // Creates a new hyperedge with a max student count (size_bits) and an identifier
    pub fn new(size_bits: usize, id: String) -> Self {
        let bitmap = BitmapLen::new(size_bits);
        Hyperedge { bitmap, id }
    }

    pub fn get_id(&self) -> &String {
        return &self.id;
    }

    // Adds a student to the hyperedge by setting the corresponding bit in the bitmap
    pub fn add_student(&mut self, student_id: usize) -> Result<(), HypergraphError> {
        return Ok(self.bitmap.set_bit(student_id)?);
    }

    pub fn apply_mask(&self, mask: &BitmapLen) -> BitmapLen {
        return self.bitmap.clone() & mask.clone();
    }
}

#[derive(Serialize, Deserialize)]
pub struct Hypergraph {
    student_count: usize, // Number of students in the hypergraph
    hyperedges: HashMap<String, Vec<Hyperedge>>, // Map of hyperedge prefixes to their indices in the hyperedges vector
}

impl Hypergraph {
    pub fn new(student_count: usize) -> Self {
        return Hypergraph {
            hyperedges: HashMap::new(),
            student_count,
        };
    }

    pub fn get_student_count(&self) -> usize {
        return self.student_count;
    }

    pub fn add_student_to_hyperedge(
        &mut self,
        hyperedge_name: &str,
        student_id: usize,
    ) -> Result<(), HypergraphError> {
        let prefix = hyperedge_name.split("_").next().ok_or_else(|| HypergraphError::InvalidHyperedgeError)?;

        // If the prefix does not exist in the hyperedges map, create a new hyperedge and add the student
        if !self.hyperedges.contains_key(prefix) {
            let hyperedge = self.add_hyperedge(hyperedge_name.to_string())?;
            return hyperedge.add_student(student_id);
        }

        // LLook for the hyperedge with the given name in the hyperedges map and add the student
        for hyperedge in self.hyperedges.get_mut(prefix).unwrap() {
            if hyperedge.get_id() == hyperedge_name {
                return hyperedge.add_student(student_id);
            }
        }

        // If the hyperedge is not found, create a new one and add the student
        let hyperedge = self.add_hyperedge(hyperedge_name.to_string())?;
        hyperedge.add_student(student_id)?;
        return Ok(());
    }

    /// Adds a new hyperedge to the hypergraph.
    fn add_hyperedge(&mut self, name: String) -> Result<&mut Hyperedge, HypergraphError> {
        let hyperedge = Hyperedge::new(self.student_count, name.clone());

        if !name.contains("_")
        {
            return Err(HypergraphError::InvalidHyperedgeError);
        }

        // Get the prefix of the hyperedge name (the part before the first underscore)
        if let Some(prefix) = name.split("_").next() {
            self.hyperedges
                .entry(prefix.to_string())
                .or_insert(Vec::new()) // If the prefix does not exist, add it
                .push(hyperedge);

            return Ok(self.hyperedges.get_mut(prefix).unwrap().last_mut().unwrap());
        } 

        return Err(HypergraphError::InvalidHyperedgeError);
    }

    pub fn get_subhypergraph_by_prefix(&self, prefix: &str) -> Result<&Vec<Hyperedge>, HypergraphError> {
        if !self.hyperedges.contains_key(prefix) {
            return Err(HypergraphError::UnknownSubHypergraphError(prefix.to_string()));
        }

        Ok(self.hyperedges.get(prefix).unwrap())
    }

    pub fn save_to_file(&self, filename: &str) -> Result<(), HypergraphError> {
        let encoded = postcard::to_allocvec(self)?;
        let mut file = File::create(filename)?;
        file.write_all(&encoded)?;
        Ok(())
    }

    pub fn load_from_file(filename: &str) -> Result<Self, HypergraphError> {
        let mut file = File::open(filename)?;
        let mut buffer = Vec::new();
        file.read_to_end(&mut buffer)?;
        let hypergraph: Hypergraph = postcard::from_bytes(&buffer)?;
        Ok(hypergraph)
    }
}

#[derive(Debug)]
pub enum HypergraphError
{
    InvalidHyperedgeError,
    UnknownSubHypergraphError(String),
    StudentOutOfBoundsError (usize, usize),
    HypergraphSerializationError(postcard::Error),
    HypergraphIOError(std::io::Error)
}

impl std::fmt::Display for HypergraphError
{
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        return match self {
            HypergraphError::InvalidHyperedgeError => write!(f, "El nombre de la hiperarista debe contener un prefijo seguido de un guion bajo"),
            HypergraphError::UnknownSubHypergraphError(prefix) => write!(f, "No se ha encontrado el subhipergrafo con el prefijo {}", prefix),
            HypergraphError::HypergraphIOError(io_err) => io_err.fmt(f),
            HypergraphError::HypergraphSerializationError(ser_err) => ser_err.fmt(f),
            HypergraphError::StudentOutOfBoundsError(student_idx, max_students) => write!(f, "Intento de cambiar el estudiante {} cuando solo hay {}", student_idx, max_students) 
        }
    }
}

impl From<postcard::Error> for HypergraphError
{
    fn from(value: postcard::Error) -> HypergraphError {
        return HypergraphError::HypergraphSerializationError(value);
    }
}

impl From <std::io::Error> for HypergraphError
{
    fn from(value : std::io::Error) -> HypergraphError
    {
        return HypergraphError::HypergraphIOError(value);
    }
}

impl From <crate::utils::bitmap::BitMapError> for HypergraphError
{
    fn from(value : crate::utils::bitmap::BitMapError) -> HypergraphError
    {
        match value
        {
            crate::utils::bitmap::BitMapError::IndexOutOfBitsError(idx, size) =>
            {
                return HypergraphError::StudentOutOfBoundsError(idx, size);
            },
            _ => unreachable!("Ha ocurrido un error inesperado")
        }
    }
}