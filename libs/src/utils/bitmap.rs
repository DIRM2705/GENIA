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

#[typetag::serde(tag = "bitmap")]
pub trait BitmapLen: Send + Sync {
    fn get_size_bits(&self) -> usize;
    fn set_bit(&mut self, index: usize) -> Result<(), String>; // Method to set a specific bit in the bitmap
    fn get_bit(&self, index: usize) -> Result<bool, String>; // Method to get the value of a specific bit in the bitmap
}

#[typetag::serde]
impl BitmapLen for u8 {
    fn get_size_bits(&self) -> usize {
        return 8;
    }

    fn set_bit(&mut self, index: usize) -> Result<(), String> {
        if index >= 8 {
            return Err("Index out of bounds for u8 bitmap".into());
        }

        *self |= 1 << index;
        Ok(())
    }

    fn get_bit(&self, index: usize) -> Result<bool, String> {
        if index >= 8 {
            return Err("Index out of bounds for u8 bitmap".into());
        }

        Ok((*self & (1 << index)) != 0)
    }
}

#[typetag::serde]
impl BitmapLen for u16 {
    fn get_size_bits(&self) -> usize {
        return 16;
    }

    fn set_bit(&mut self, index: usize) -> Result<(), String> {
        if index >= 16 {
            return Err("Index out of bounds for u16 bitmap".into());
        }

        *self |= 1 << index;
        Ok(())
    }

    fn get_bit(&self, index: usize) -> Result<bool, String> {
        if index >= 16 {
            return Err("Index out of bounds for u16 bitmap".into());
        }

        Ok((*self & (1 << index)) != 0)
    }
}

#[typetag::serde]
impl BitmapLen for u32 {
    fn get_size_bits(&self) -> usize {
        return 32;
    }

    fn set_bit(&mut self, index: usize) -> Result<(), String> {
        if index >= 32 {
            return Err("Index out of bounds for u32 bitmap".into());
        }

        *self |= 1 << index;
        Ok(())
    }

    fn get_bit(&self, index: usize) -> Result<bool, String> {
        if index >= 32 {
            return Err("Index out of bounds for u32 bitmap".into());
        }

        Ok((*self & (1 << index)) != 0)
    }
}

#[typetag::serde]
impl BitmapLen for u64 {
    fn get_size_bits(&self) -> usize {
        return 64;
    }

    fn set_bit(&mut self, index: usize) -> Result<(), String> {
        if index >= 64 {
            return Err("Index out of bounds for u64 bitmap".into());
        }

        *self |= 1 << index;
        Ok(())
    }

    fn get_bit(&self, index: usize) -> Result<bool, String> {
        if index >= 64 {
            return Err("Index out of bounds for u64 bitmap".into());
        }

        Ok((*self & (1 << index)) != 0)
    }
}

#[typetag::serde]
impl BitmapLen for u128 {
    fn get_size_bits(&self) -> usize {
        return 128;
    }

    fn set_bit(&mut self, index: usize) -> Result<(), String> {
        if index >= 128 {
            return Err("Index out of bounds for u128 bitmap".into());
        }
        *self |= 1 << index;
        Ok(())
    }

    fn get_bit(&self, index: usize) -> Result<bool, String> {
        if index >= 128 {
            return Err("Index out of bounds for u128 bitmap".into());
        }

        Ok((*self & (1 << index)) != 0)
    }
}

#[typetag::serde]
impl BitmapLen for Box<[u8]> {

    fn get_size_bits(&self) -> usize {
        self.len() * 8
    }

    fn set_bit(&mut self, index: usize) -> Result<(), String> {
        if index >= self.get_size_bits() {
            return Err("Index out of bounds for Box<[u8]> bitmap".into());
        }

        let byte_index = index / 8;
        let bit_index = index % 8;
        self[byte_index] |= 1 << bit_index;
        Ok(())
    }

    fn get_bit(&self, index: usize) -> Result<bool, String> {
        if index >= self.get_size_bits() {
            return Err("Index out of bounds for Box<[u8]> bitmap".into());
        }

        let byte_index = index / 8;
        let bit_index = index % 8;
        Ok((self[byte_index] & (1 << bit_index)) != 0)
    }
}

// Crea un bitmap de tamaño específico
pub fn make_bitmap_of_len(size_bits: usize) -> Box<dyn BitmapLen> {
    let size_bytes = (size_bits + 7) / 8; // Calcular el número de bytes mínimo
    match size_bytes {
        0..=1 => Box::new(0u8),    
        2 => Box::new(0u16),       
        3..=4 => Box::new(0u32),   
        5..=8 => Box::new(0u64),   
        9..=16 => Box::new(0u128),
        _ => Box::new(vec![0u8; size_bytes].into_boxed_slice())
    }
}

//Cambiar el tamaño del bitmap, copiando los bits del bitmap original al nuevo bitmap
pub fn resize_bitmap(bitmap: &Box<dyn BitmapLen>, new_size_bits: usize) -> Result<Box<dyn BitmapLen>, String> {
    if new_size_bits < bitmap.get_size_bits() {
        return Err("New size must be greater than or equal to the current size".into());
    }

    let mut target  = make_bitmap_of_len(new_size_bits);
    for i in 0..bitmap.get_size_bits() {
        if bitmap.get_bit(i)? {
            target.set_bit(i)?;
        }
    }
    return Ok(target);
}