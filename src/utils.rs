use std::sync::Arc;

use byteorder::{LittleEndian, ReadBytesExt};

pub fn bytes_to_num(bytes: &[u8]) -> usize {
    let mut template: [u8; 4] = [0, 0, 0, 0];
    template[..bytes.len()].copy_from_slice(bytes);
    let mut cursor = std::io::Cursor::new(&template);
    let num = cursor.read_u32::<LittleEndian>().unwrap() as usize;
    num
}

pub fn read_leu8(buffer: &[u8], pointer: &mut usize, peek: bool) -> u8 {
    let int = buffer[*pointer];
    if !peek {
        *pointer += 1;
    }
    int
}

pub fn read_leu16(buffer: &[u8], pointer: &mut usize, peek: bool) -> u16 {
    let mut template: [u8; 2] = [0, 0];
    template.copy_from_slice(&buffer[*pointer..*pointer + 2]);
    let mut cursor = std::io::Cursor::new(&template);
    let int = cursor.read_u16::<LittleEndian>().unwrap();
    if !peek {
        *pointer += 2;
    }
    int
}

pub fn read_leu32(buffer: &[u8], pointer: &mut usize, peek: bool) -> u32 {
    let mut template: [u8; 4] = [0, 0, 0, 0];
    template.copy_from_slice(&buffer[*pointer..*pointer + 4]);
    let mut cursor = std::io::Cursor::new(&template);
    let int = cursor.read_u32::<LittleEndian>().unwrap();
    if !peek {
        *pointer += 4;
    }
    int
}

pub fn read_beu8(buffer: &[u8], pointer: &mut usize, peek: bool) -> u8 {
    let int = buffer[*pointer];
    if !peek {
        *pointer += 1;
    }
    int
}

pub fn read_beu16(buffer: &[u8], pointer: &mut usize, peek: bool) -> u16 {
    let mut template: [u8; 2] = [0, 0];
    template.copy_from_slice(&buffer[*pointer..*pointer + 2]);
    let mut cursor = std::io::Cursor::new(&template);
    let int = cursor.read_u16::<byteorder::BigEndian>().unwrap();
    if !peek {
        *pointer += 2;
    }
    int
}

pub fn read_beu32(buffer: &[u8], pointer: &mut usize, peek: bool) -> u32 {
    let mut template: [u8; 4] = [0, 0, 0, 0];
    template.copy_from_slice(&buffer[*pointer..*pointer + 4]);
    let mut cursor = std::io::Cursor::new(&template);
    let int = cursor.read_u32::<byteorder::BigEndian>().unwrap();
    if !peek {
        *pointer += 4;
    }
    int
}
