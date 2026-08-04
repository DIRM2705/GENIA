use crate::utils::bitmap;

use bitmap::BitmapLen;
use serde::{Deserialize, Serialize};
use std::collections::HashMap;
use std::fs::File;
use std::io::{Read, Write};

/// A hyperedge representing a relationship between students in a hypergraph.
/// 
/// # Fields
/// 
/// * `bitmap`: A bitmap representing the students in the hyperedge. Each bit corresponds to a student, and a set bit indicates that the student is part of the hyperedge.
/// * `id`: The identifier for the hyperedge.

#[derive(Serialize, Deserialize)]
pub struct Hyperedge {
    bitmap: BitmapLen, 
    id: String,      
}

impl Hyperedge {
    /// Creates a new hyperedge
    /// 
    /// # Arguments
    /// 
    /// * `size_bits`: The number of bits in the bitmap
    /// * `id`: The identifier for the hyperedge
    pub fn new(size_bits: usize, id: String) -> Self {
        let bitmap = BitmapLen::new(size_bits);
        Hyperedge { bitmap, id }
    }

    pub fn get_id(&self) -> &String {
        return &self.id;
    }

    /// Adds a student to the hyperedge
    /// 
    /// # Arguments
    /// 
    /// * `student_id`: The index of the student to add
    pub fn add_student(&mut self, student_id: usize) -> Result<(), HypergraphError> {
        return Ok(self.bitmap.set_bit(student_id)?);
    }

    /// Finds the students in the hyperedge that are also present in the given mask
    /// 
    /// # Arguments
    /// 
    /// * `mask`: A bitmap representing the students to check against
    pub fn apply_mask(&self, mask: &BitmapLen) -> BitmapLen {
        return self.bitmap.clone() & mask.clone();
    }
}


/// A hypergraph representing relationships between students.
/// 
/// # Fields
/// 
/// * `student_count`: The number of students in the hypergraph.
/// * `hyperedges`: A map of hyperedge prefixes to their corresponding hyperedges.
#[derive(Serialize, Deserialize)]
pub struct Hypergraph {
    student_count: usize,
    hyperedges: HashMap<String, Vec<Hyperedge>>,
}

impl Hypergraph {
    /// Creates a new hypergraph with the given number of students.
    /// 
    /// # Arguments
    /// 
    /// * `student_count`: The number of students in the hypergraph.
    pub fn new(student_count: usize) -> Self {
        return Hypergraph {
            hyperedges: HashMap::new(),
            student_count,
        };
    }

    /// Returns the number of students in the hypergraph.
    /// 
    /// # Returns
    /// 
    /// * `usize`: The number of students in the hypergraph.
    pub fn get_student_count(&self) -> usize {
        return self.student_count;
    }

    /// Adds a student to a hyperedge in the hypergraph.
    ///
    /// # Arguments
    /// 
    /// * `hyperedge_name`: The name of the hyperedge to add the student to
    /// * `student_id`: The index of the student to add
    /// 
    /// # Returns
    /// 
    /// * `Result<(), HypergraphError>`: A result indicating success or failure.
    /// 
    /// # Errors
    /// 
    /// * `HypergraphError::InvalidHyperedgeError`: If the hyperedge name does not contain an underscore.
    /// * `HypergraphError::StudentOutOfBoundsError`: If the student index is out of bounds.
    /// * `HypergraphError::UnknownSubHypergraphError`: If the hyperedge prefix does not exist in the hypergraph.
    pub fn add_student_to_hyperedge(
        &mut self,
        hyperedge_name: &str,
        student_id: usize,
    ) -> Result<(), HypergraphError> {
        if student_id >= self.student_count {
            return Err(HypergraphError::StudentOutOfBoundsError(
                student_id,
                self.student_count,
            ));
        }
        let prefix = hyperedge_name.split("_").next().ok_or_else(|| HypergraphError::InvalidHyperedgeError)?;

        // If the prefix does not exist in the hyperedges map, create a new hyperedge and add the student
        if !self.hyperedges.contains_key(prefix) {
            let hyperedge = self.add_hyperedge(hyperedge_name.to_string())?;
            return hyperedge.add_student(student_id);
        }

        // Look for the hyperedge with the given name in the hyperedges map and add the student
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
    /// 
    /// # Arguments
    /// 
    /// * `name`: The name of the hyperedge to add
    /// 
    /// # Returns
    /// 
    /// * `Result<&mut Hyperedge, HypergraphError>`: A result containing a mutable reference to the newly added hyperedge or an error.
    /// 
    /// # Errors
    /// 
    /// * `HypergraphError::InvalidHyperedgeError`: If the hyperedge name does not contain an underscore.
    /// * `HypergraphError::UnknownSubHypergraphError`: If the hyperedge prefix does not exist in the hypergraph.
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

    /// Retrieves a subhypergraph by its prefix.
    /// 
    /// # Arguments
    /// 
    /// * `prefix`: The prefix of the subhypergraph to retrieve
    /// 
    /// # Returns
    /// 
    /// * `Result<&Vec<Hyperedge>, HypergraphError>`: A result containing a reference to the vector of hyperedges corresponding to the prefix or an error.
    /// 
    /// # Errors
    /// 
    /// * `HypergraphError::UnknownSubHypergraphError`: If the hyperedge prefix does not exist in the hypergraph.
    pub fn get_subhypergraph_by_prefix(&self, prefix: &str) -> Result<&Vec<Hyperedge>, HypergraphError> {
        if !self.hyperedges.contains_key(prefix) {
            return Err(HypergraphError::UnknownSubHypergraphError(prefix.to_string()));
        }

        Ok(self.hyperedges.get(prefix).unwrap())
    }

    /// Saves the hypergraph to a file in a serialized format.
    ///
    /// # Arguments
    /// 
    /// * `filename`: The name of the file to save the hypergraph to
    /// 
    /// # Returns
    /// 
    /// * `Result<(), HypergraphError>`: A result indicating success or failure.
    /// 
    /// # Errors
    /// 
    /// * `HypergraphError::HypergraphSerializationError`: If there is an error during serialization.
    /// * `HypergraphError::HypergraphIOError`: If there is an error during file I/O operations.
    pub fn save_to_file(&self, filename: &str) -> Result<(), HypergraphError> {
        let encoded = postcard::to_allocvec(self)?;
        let mut file = File::create(filename)?;
        file.write_all(&encoded)?;
        Ok(())
    }

    /// Loads a hypergraph from a file in a serialized format.
    /// 
    /// # Arguments
    /// 
    /// * `filename`: The name of the file to load the hypergraph from
    /// 
    /// # Returns
    /// 
    /// * `Result<Self, HypergraphError>`: A result containing the loaded hypergraph or an error.
    /// 
    /// # Errors
    /// 
    /// * `HypergraphError::HypergraphSerializationError`: If there is an error during deserialization.
    /// * `HypergraphError::HypergraphIOError`: If there is an error during file I/O operations.
    pub fn load_from_file(filename: &str) -> Result<Self, HypergraphError> {
        let mut file = File::open(filename)?;
        let mut buffer = Vec::new();
        file.read_to_end(&mut buffer)?;
        let hypergraph: Hypergraph = postcard::from_bytes(&buffer)?;
        Ok(hypergraph)
    }
}


/// An enumeration representing possible errors that can occur in the hypergraph operations.
/// 
/// # Errors
/// * `InvalidHyperedgeError`: Indicates that the hyperedge name does not contain an underscore.
/// * `UnknownSubHypergraphError`: Indicates that the hyperedge prefix does not exist in the hypergraph.
/// * `StudentOutOfBoundsError`: Indicates that the student index is out of bounds.
/// * `HypergraphSerializationError`: Indicates an error during serialization or deserialization of the hypergraph.
/// * `HypergraphIOError`: Indicates an error during file I/O operations.
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