/*
   Módulo para crear los bitmaps de tamaño variable requeridos por las hiperaristas
   Se implementan para tamaños variables, principalmente:
   - 8 bits
   - 16 bits
   - 32 bits
   - 64 bits
   - 128 bits

   Se utiliza un arreglo de bytes por si se necesitaran bitmaps de tamaño superior
*/

use serde::{Deserialize, Serialize};
use std::cmp::min;
use std::ops;

#[derive(Serialize, Deserialize, Clone, Debug)]
pub struct BitmapLen {
    bitmap: Box<[u8]>,
}

impl BitmapLen {
    pub fn new(size_bits: usize) -> Self {
        let size_bytes = (size_bits + 7) / 8; // Calcular el número de bytes mínimo
        BitmapLen {
            bitmap: vec![0u8; size_bytes].into_boxed_slice(),
        }
    }

    pub fn count_ones(&self) -> u32 {
       return self.bitmap.iter().map(|byte| byte.count_ones()).sum();
    }

    pub fn get_size_bits(&self) -> usize {
        self.bitmap.len() * 8
    }

    pub fn get_chunk_mut(&mut self, index: usize) -> Result<&mut u8, String> {
        if index >= self.get_size_bits() {
            return Err("Index out of bounds for Box<[u8]> bitmap".into());
        }

        return Ok(&mut self.bitmap[index]);
    }

    pub fn set_bit(&mut self, index: usize) -> Result<(), String> {
        if index >= self.get_size_bits() {
            return Err("Index out of bounds for Box<[u8]> bitmap".into());
        }

        let byte_index = index / 8;
        let bit_index = index % 8;
        self.bitmap[byte_index] |= 1 << bit_index;
        Ok(())
    }

    pub fn clear_bit(&mut self, index: usize) -> Result<(), String> {
        if index >= self.get_size_bits() {
            return Err("Index out of bounds for Box<[u8]> bitmap".into());
        }

        let byte_index = index / 8;
        let bit_index = index % 8;
        self.bitmap[byte_index] &= !(1 << bit_index);
        Ok(())
    }

    pub fn get_bit(&self, index: usize) -> Result<bool, String> {
        if index >= self.get_size_bits() {
            return Err("Index out of bounds for Box<[u8]> bitmap".into());
        }

        let byte_index = index / 8;
        let bit_index = index % 8;
        return Ok((self.bitmap[byte_index] & (1 << bit_index)) != 0);
    }

    pub fn set_bits(&mut self, indices: &[usize]) -> Result<(), String> {
        for &index in indices {
            if index >= self.get_size_bits() {
                return Err("Index out of bounds for Box<[u8]> bitmap".into());
            }

            let byte_index = index / 8;
            let bit_index = index % 8;
            self.bitmap[byte_index] |= 1 << bit_index;
        }
        Ok(())
    }
}

impl ops::BitAnd for BitmapLen {
    type Output = Self;

    fn bitand(self, other: Self) -> Self::Output {
        let bytes = min(self.bitmap.len(), other.bitmap.len());
        let mut result = BitmapLen::new(self.get_size_bits());

        for i in 0..self.bitmap.len() {
            if let Ok(chunk) = result.get_chunk_mut(i)
                && i < bytes
            {
                *chunk = self.bitmap[i] & other.bitmap[i];
            }
        }

        return result;
    }
}

impl ops::BitOr for BitmapLen {
    type Output = Self;

    fn bitor(self, other: Self) -> Self::Output {
        let bytes = min(self.bitmap.len(), other.bitmap.len());
        let mut result = BitmapLen::new(self.get_size_bits());

        for i in 0..self.bitmap.len() {
            if let Ok(chunk) = result.get_chunk_mut(i)
                && i < bytes
            {
                *chunk = self.bitmap[i] | other.bitmap[i];
            }
        }

        return result;
    }
}
