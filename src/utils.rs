use byteorder::{LittleEndian, ReadBytesExt};

pub fn bytes_to_num(bytes: &[u8]) -> usize {
    let mut template: [u8; 4] = [0, 0, 0, 0];
    template[..bytes.len()].copy_from_slice(bytes);
    let mut cursor = std::io::Cursor::new(&template);
    let num = cursor.read_u32::<LittleEndian>().unwrap() as usize;
    num
}
