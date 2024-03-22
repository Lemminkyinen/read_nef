use std::{io::Write, path::Path};
mod huffman;
mod huffmanv2;
mod ifd;
mod nef;
mod utils;

fn main() {
    let file_path = Path::new("DSC_6400.NEF");
    match nef::NefFile::open(file_path) {
        Ok(nef_file) => {
            // println!("Metadata: {:?}", nef_file.metadata);
            // println!("Image data: {:?}", nef_file.image_data);
            // for (x, ifd) in nef_file.ifds.iter().enumerate() {
            //     println!("Ifd {:?}", x);
            //     ifd.print_info();
            // }
            // println!("Ifd 2");
            // nef_file.ifds[2].print_info();
            // println!("Ifd 5");
            // nef_file.ifds[5].print_info();

            let asd = nef_file
                .parse_raw_image_data()
                .expect("Error parsing raw image data");
        }
        Err(e) => eprintln!("Error: {:?}", e),
    }

    // let testi_data = "huffman decode testi".as_bytes();
    // println!("Testi_data_pituus: {:?}", testi_data.len() * 8);
    // let encoded_data = huffman::encode(testi_data);
    // println!(
    //     "huffman table: {:?} \nencoded data: {:?}",
    //     encoded_data.0, encoded_data.1
    // );
    // println!(
    //     "encoded_length: {:?}",
    //     encoded_data.1.len() + encoded_data.0.len()
    // );
    // let decoded_data = huffman::decode(encoded_data.0.clone(), &encoded_data.1);
    // println!("decoded data: {:?}", decoded_data);
    // let string = String::from_utf8(decoded_data).unwrap();
    // println!("String: {}", string);
}
