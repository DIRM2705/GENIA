use serde::{Deserialize, Serialize};
use std::cmp::min;
use std::ops;

#[derive(Serialize, Deserialize, Clone, Debug)]
pub struct BitmapLen {
    bitmap: Box<[u8]>,
}

impl BitmapLen {
    pub fn new(size_bits: usize) -> Self {
        // Calculate the number of bytes needed to store the bits, rounding up
        let size_bytes = (size_bits + 7) / 8; 
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

    pub fn get_chunk_mut(&mut self, index: usize) -> Result<&mut u8, BitMapError> {
        if index >= self.bitmap.len() {
            return Err(BitMapError::IndexOutOfChunksError(index, self.bitmap.len()));
        }

        return Ok(&mut self.bitmap[index]);
    }

    pub fn set_bit(&mut self, index: usize) -> Result<(), BitMapError> {
        if index >= self.get_size_bits() {
            return Err(BitMapError::IndexOutOfBitsError(index, self.get_size_bits()));
        }

        let byte_index = index / 8;
        let bit_index = index % 8;
        self.bitmap[byte_index] |= 1 << bit_index;
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
            else 
            {
                unreachable!()
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
            else
            {
                unreachable!()
            }
        }

        return result;
    }
}

#[derive(Debug)]
pub enum BitMapError
{
    IndexOutOfBitsError(usize, usize),
    IndexOutOfChunksError(usize, usize)
}


impl std::fmt::Display for BitMapError
{
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        return match self
        {
            BitMapError::IndexOutOfBitsError(idx, len) => write!(f, "Indice {} fuera de los límites para bitmap de longitud {}", idx, len),
            BitMapError::IndexOutOfChunksError(idx, chunk_count) => write!(f, "Indice {} fuera de los limites, hay {} chunks", idx, chunk_count)
        }
    }
}