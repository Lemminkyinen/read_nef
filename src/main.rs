use std::path::Path;

use ifd::TagParam;
mod huffman;
mod ifd;
mod nef;
mod utils;

fn main() {
    let file_path = Path::new("DSC_6400.NEF");
    match nef::NefFile::open(file_path) {
        Ok(nef_file) => {
            // println!("Metadata: {:?}", nef_file.metadata);
            // println!("Image data: {:?}", nef_file.image_data);
            for (x, ifd) in nef_file.ifds.iter().enumerate() {
                println!("Ifd {:?}", x);
                ifd.print_info();
            }

            match nef_file.get_n_ifd(5) {
                Some(makernote_ifd) => {
                    let entry_0x8c = makernote_ifd.get_entry(TagParam::U8(0x8c)).unwrap();
                    let entry_0x96 = makernote_ifd.get_entry(TagParam::U8(0x96)).unwrap();
                    entry_0x8c.print_info();
                    entry_0x96.print_info();
                    // tee 
                    let offset_data_0x8c = entry_0x8c.get_offset_data(nef_file.get_buffer());
                    let offset_data_0x96 = entry_0x96.get_offset_data(nef_file.get_buffer());
                    // println!("{:?}", )
                }
                None => {
                    println!("Couldn't get ifd")
                }
            }
        }
        Err(e) => eprintln!("Error: {:?}", e),
    }

    // The quantization tables are at 0x8c and 0x96 tag from the MakerNote

    let testi_data = "Jerellllllllllllllllllä on pittttttttttttttttttttttttttkäääääääääääääääää nulllllllllllllllllllllllllllllllllli".as_bytes();
    // println!("Testi_data_pituus: {:?}", testi_data.len() * 8);
    let encoded_data = huffman::encode(testi_data);
    // println!(
    //     "huffman table: {:?} \nencoded data: {:?}",
    //     encoded_data.0, encoded_data.1
    // );
    // println!(
    //     "encoded_length: {:?}",
    //     encoded_data.1.len() + encoded_data.0.len()
    // );
    let decoded_data = huffman::decode(encoded_data.0.clone(), &encoded_data.1);
    // println!("decoded data: {:?}", decoded_data);
    let string = String::from_utf8(decoded_data).unwrap();
    // println!("String: {}", string);
}
