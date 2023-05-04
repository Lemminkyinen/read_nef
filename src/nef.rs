use crate::ifd::Ifd;
use std::path::PathBuf;
use std::{fs::File, io::Read, path::Path};

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct NefFile {
    pub file_name: String,
    pub file_path: PathBuf,
    pub meta_data: ImageMetadata,
    pub image_data: ImageData,
    pub ifds: Vec<Ifd>,
    buffer: Vec<u8>,
}

#[derive(Debug, Copy, Clone, PartialEq, Eq)]
pub struct ImageMetadata {
    pub image_size: usize,
    // pub make: String,
    // pub model: String,
    // pub iso_speed_ratings: Option<u16>,
    // pub exposure_time: Option<f32>,
    // pub f_number: Option<f32>,
}

#[derive(Debug, Copy, Clone, PartialEq, Eq)]
pub struct ImageData {
    pub height: usize,
    // pub width: usize,
    // pub data_offset: u64,
    // pub tiff_bps: u16,
    // pub raw_image: Vec<u16>,
}

#[derive(Debug, Copy, Clone, PartialEq, Eq)]
pub struct ImageThumbnail {}

#[derive(Debug)]
pub enum Error {
    IoError(std::io::Error),
    ParseError(String),
    // Add other error variants as needed.
}

impl NefFile {
    pub fn open(file_path: &Path) -> Result<NefFile, Error> {
        let mut file = File::open(file_path).expect("Error loading file");
        let file_path = Self::get_absolute_path(file_path).expect("Error getting full path");
        let file_name = file_path
            .file_name()
            .unwrap()
            .to_str()
            .unwrap()
            .to_owned()
            .into();

        let mut buffer = Vec::new();
        file.read_to_end(&mut buffer).unwrap();

        let meta_data = ImageMetadata { image_size: 0 };

        let image_data = ImageData { height: 0 };

        let ifds: Vec<Ifd> = Vec::new();

        let mut nef_file = NefFile {
            file_name,
            file_path,
            meta_data,
            image_data,
            ifds,
            buffer,
        };

        let mut ifds = nef_file.parse_ifds().expect("Error parsing ifds");

        nef_file.ifds.append(&mut ifds);

        Ok(nef_file)
    }

    fn get_absolute_path(file_name: &Path) -> Option<PathBuf> {
        let current_dir = std::env::current_dir().ok()?;
        let file_path = current_dir.join(file_name);
        Some(file_path.canonicalize().ok()?)
    }

    fn parse_ifds(&self) -> Result<Vec<Ifd>, Error> {
        let parsed_ifds = Ifd::parse_ifd(self.buffer.as_slice());
        Ok(parsed_ifds)
    }
    // fn parse_metadata(&mut self) -> Result<(), Error> {
    //     // Extract metadata from the file.
    // }

    // fn parse_image_data(&mut self) -> Result<(), Error> {
    //     // Extract raw image data from the file.
    // }

    // fn parse_image_thumbnail(&mut self) -> Result<(), Error> {
    //     // Extract image thumbnail data from the file.
    // }
}
