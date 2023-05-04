use std::path::Path;
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
        }
        Err(e) => eprintln!("Error: {:?}", e),
    }
}
