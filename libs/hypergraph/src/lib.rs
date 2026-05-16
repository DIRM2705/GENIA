mod bitmap;

use std::collections::HashMap;
use bitmap::{BitmapLen, make_bitmap_of_len, resize_bitmap};
use serde::{Serialize, Deserialize};
use std::fs::File;
use std::io::{Result, Read, Write};
use bincode::{serialize, deserialize};

#[derive(Serialize, Deserialize)]
pub struct Hyperedge {
    #[serde(with = "BitmapLen")]
    bitmap: Box<dyn BitmapLen>, // Bitmap representing the students in the hyperedge
}

impl Hyperedge {
    // Crea una nueva hiperarista con un bitmap del tamaño adecuado para el número de estudiantes
    pub fn new(size_bits: usize) -> Self {
        let bitmap = make_bitmap_of_len(size_bits);
        Hyperedge {
            bitmap
        }
    }

    // Cambia el tamaño del bitmap de la hiperarista, ajustándolo al nuevo número de estudiantes
    pub fn resize(&mut self, new_size_bits: usize) -> Result<(), String> {
        self.bitmap = resize_bitmap(&mut self.bitmap, new_size_bits)?;
        Ok(())
    }

    // Agrega un estudiante a la hiperarista, estableciendo el bit correspondiente en el bitmap
    pub fn add_student(&mut self, student_id: usize) -> Result<(), String> {
        return self.bitmap.set_bit(student_id);
    }
}

#[derive(Serialize, Deserialize)]
pub struct Hypergraph {
    hyperedges: HashMap<String, Hyperedge>, // Map from hyperedge name to Hyperedge
    student_count: usize,                   // Number of students in the hypergraph
}

impl Hypergraph {
    // Crea un nuevo hipergrafo con un número específico de estudiantes
    pub fn new(student_count: usize) -> Self {
        return Hypergraph {
            hyperedges: HashMap::new(),
            student_count,
        };
    }

    // Crea una nueva hiperarista con los valores de los estudiantes seteado
    pub fn add_hyperedge(&mut self, name: String, students: Vec<usize>) -> Result<(), String> {
        let mut hyperedge = Hyperedge::new(self.student_count);
        for student in students {
            hyperedge.add_student(student)?;
        }
        self.hyperedges.insert(name, hyperedge);
        Ok(())
    }

    // Agrega un nuevo estudiante al hipergrafo, ajustando los bitmaps de las hiperaristas existentes
    pub fn add_student(&mut self) -> Result<(), String> {
        self.student_count += 1;
        for hyperedge in self.hyperedges.values_mut() {
            hyperedge.resize(self.student_count)?;
        }
        Ok(())
    }

    // Obtiene una referencia mutable a una hiperarista por su nombre, permitiendo modificarla
    pub fn get_hyperedge_mut(&mut self, name: &str) -> Option<&mut Hyperedge> {
        self.hyperedges.get_mut(name)
    }

    // Obtiene una referencia a una hiperarista por su nombre, permitiendo leerla
    pub fn save_to_file(&self, filename: &str) -> Result<()> {
        let encoded: Vec<u8> = serialize(self).unwrap();
        let mut file = File::create(filename)?;
        file.write_all(&encoded)?;
        Ok(())
    }

    // Carga un hipergrafo desde un archivo, deserializando su contenido
    pub fn load_from_file(filename: &str) -> Result<Self> {
        let mut file = File::open(filename)?;
        let mut buffer = Vec::new();
        file.read_to_end(&mut buffer)?;
        let hypergraph: Hypergraph = deserialize(&buffer).unwrap();
        Ok(hypergraph)
    }
}
