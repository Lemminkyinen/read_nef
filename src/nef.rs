use byteorder::{ByteOrder, LittleEndian};
use image::buffer;

use crate::ifd::Ifd;
use std::path::PathBuf;
use std::sync::Arc;
use std::{fs::File, io::Read, path::Path};

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct NefFile {
    pub file_name: String,
    pub file_path: PathBuf,
    pub meta_data: ImageMetadata,
    pub image_data: ImageData,
    pub ifds: Vec<Ifd>,
    buffer: Arc<[u8]>,
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
        file.read_to_end(&mut buffer).expect("Error reading file");

        let meta_data = ImageMetadata { image_size: 0 };

        let image_data = ImageData { height: 0 };

        let ifds: Vec<Ifd> = Vec::new();

        let mut nef_file = NefFile {
            file_name,
            file_path,
            meta_data,
            image_data,
            ifds,
            buffer: Arc::from(buffer),
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
        let parsed_ifds = Ifd::parse_ifd(self.buffer.clone(), 0);
        Ok(parsed_ifds)
    }
    // fn parse_metadata(&mut self) -> Result<(), Error> {
    //     // Extract metadata from the file.
    // }

    fn nikon_read_curve(buffer: Arc<[u8]>, meta_offset: usize, tiff_bps: usize) -> Vec<usize> {
        let ver0: u8;
        let ver1: u8;
        let mut vpred: [[u16; 2]; 2] = [[0; 2]; 2];
        let csize: usize;
        let mut step: usize;
        let mut max: usize;

        let mut pointer = meta_offset as usize;
        (ver0, ver1) = (buffer[pointer], buffer[pointer + 1]);
        if ver0 == 0x49 || ver1 == 0x58 {
            pointer += 2112;
        }

        // read_shorts
        let vpred_bytes = &buffer[pointer..pointer + 8];
        for i in 0..2 {
            for j in 0..2 {
                vpred[i][j] = LittleEndian::read_u16(&vpred_bytes[(i * 2 + j) * 2..]);
            }
        }
        pointer += 8;

        // Calculate the step size for the curve if the curve size is greater than 1
        step = (1 << tiff_bps) & 0x7fff as usize;
        max = step;

        // Calculate the step size for the curve if the curve size is greater than 1
        csize = LittleEndian::read_u16(&buffer[pointer..pointer + 2]) as usize;
        pointer += 2;
        if csize > 1 {
            step = max / (csize as usize - 1);
        }

        let mut curve = vec![0; max];
        if ver0 == 0x44 && (ver1 == 0x20 || (ver1 == 0x40 && step > 3)) && step > 0 {
            if ver1 == 0x40 {
                step /= 4;
                max /= 4;
            }
            for i in 0..csize as usize {
                curve[i * step] = LittleEndian::read_u16(&buffer[pointer..pointer + 2]) as usize;
            }
            for i in 0..max {
                curve[i] = (curve[i - i % step] * (step - i % step)
                    + curve[i - i % step + step] * (i % step))
                    / step;
            }
        } else if ver0 != 0x46 && csize <= 0x4001 {
            let max = csize as usize;
            for i in 0..max {
                curve[i] = LittleEndian::read_u16(&buffer[pointer..pointer + 2]) as usize;
                pointer += 2;
            }
        }
        curve
    }

    fn make_decoder(source: &[u8]) -> Vec<u16> {
        let mut max = 0;
        let mut h = 0;
        println!("Source: {:?}", source);
        let count = &source[0..17];
        for m in (0..=16).rev() {
            if count[m] != 0 {
                max = m;
                break;
            }
        }

        let mut huff = vec![0; 1 + (1 << max)];
        huff[0] = max as u16;

        let mut source_index = 16;
        for len in 1..=max {
            for _ in 0..count[len] {
                for _ in 0..(1 << (max - len)) {
                    if h <= (1 << max) {
                        huff[h] = ((len as u16) << 8) | source[source_index] as u16;
                        h += 1;
                    }
                }
                source_index += 1;
            }
        }
        huff
    }

    pub fn parse_raw_image_data(&self) -> Result<Vec<Vec<usize>>, Error> {
        static NIKON_TREE: [[u8; 32]; 6] = [
            [
                0, 1, 5, 1, 1, 1, 1, 1, 1, 2, 0, 0, 0, 0, 0, 0, // 12-bit lossy
                5, 4, 3, 6, 2, 7, 1, 0, 8, 9, 11, 10, 12, 0, 0, 0,
            ],
            [
                0, 1, 5, 1, 1, 1, 1, 1, 1, 2, 0, 0, 0, 0, 0, 0, // 12-bit lossy after split
                0x39, 0x5a, 0x38, 0x27, 0x16, 5, 4, 3, 2, 1, 0, 11, 12, 12, 0, 0,
            ],
            [
                0, 1, 4, 2, 3, 1, 2, 0, 0, 0, 0, 0, 0, 0, 0, 0, // 12-bit lossless
                5, 4, 6, 3, 7, 2, 8, 1, 9, 0, 10, 11, 12, 0, 0, 0,
            ],
            [
                0, 1, 4, 3, 1, 1, 1, 1, 1, 2, 0, 0, 0, 0, 0, 0, // 14-bit lossy
                5, 6, 4, 7, 8, 3, 9, 2, 1, 0, 10, 11, 12, 13, 14, 0,
            ],
            [
                0, 1, 5, 1, 1, 1, 1, 1, 1, 1, 2, 0, 0, 0, 0, 0, // 14-bit lossy after split
                8, 0x5c, 0x4b, 0x3a, 0x29, 7, 6, 5, 4, 3, 2, 1, 0, 13, 14, 0,
            ],
            [
                0, 1, 4, 2, 2, 3, 1, 2, 0, 0, 0, 0, 0, 0, 0, 0, // 14-bit lossless
                7, 6, 8, 5, 9, 4, 10, 3, 11, 12, 2, 0, 1, 13, 14, 0,
            ],
        ];

        // Clone the third IFD (Image File Directory) which contains the raw image data
        let data_ifd = self.ifds[2].clone();

        // init width and height
        let width = data_ifd
            .get_entry(crate::ifd::IfdEntryTag::ImageWidth)
            .unwrap()
            .get_data_or_offset();
        let height = data_ifd
            .get_entry(crate::ifd::IfdEntryTag::ImageLength)
            .unwrap()
            .get_data_or_offset();

        let mut hpred = vec![0; width];

        // Initialize vertical predictor array
        let mut vpred: [[u16; 2]; 2] = [[0; 2]; 2];

        // Initialize split value
        let mut split = 0;

        // Clone the last IFD which contains the MakerNote
        let makernote_ifd = self.ifds.last().unwrap().clone();
        println!("MakeNote IFD offset: {}", makernote_ifd.offset_location);

        // Get the entry for the 0x96 tag from the MakerNote IFD
        let entry_0x96 = makernote_ifd.get_entry_by_byte(0x96).unwrap().clone();

        // Get the offset for the 0x96 tag data
        let pointer_0x96 = entry_0x96.get_data_or_offset();
        println!("Pointer 0x96: {}", pointer_0x96);

        // Get the BitsPerSample value from the data IFD
        let tiff_bps = data_ifd
            .get_entry(crate::ifd::IfdEntryTag::BitsPerSample)
            .unwrap()
            .get_data_or_offset();

        // Calculate the total pointer for the 0x96 tag data
        let mut pointer = pointer_0x96 + makernote_ifd.offset_location;
        println!("Pointer total 0x96: {}\n", pointer);

        // Save the meta offset
        let meta_offset = pointer;

        // Get the version bytes
        let (ver0, ver1) = (self.buffer[pointer], self.buffer[pointer + 1]);
        println!("Version: {:#02X} {:#02X}", ver0, ver1);

        // Move the pointer past the version bytes
        pointer += 2;

        // Determine the Huffman compression type based on the version bytes and BitsPerSample
        let mut huff_tree = if ver0 == 0x46 { 2 } else { 0 };
        if tiff_bps == 14 {
            huff_tree += 3;
        }

        // Read the vertical predictor values
        let vpred_bytes = &self.buffer[pointer..pointer + 8];
        for i in 0..2 {
            for j in 0..2 {
                vpred[i][j] = LittleEndian::read_u16(&vpred_bytes[(i * 2 + j) * 2..]);
            }
        }
        pointer += 8;

        // Calculate the maximum value that can be represented with the given BitsPerSample
        let mut max = (1 << tiff_bps) & 0x7fff;

        // Read the curve size
        let csize = LittleEndian::read_u16(&self.buffer[pointer..pointer + 2]);
        pointer += 2;

        // If the version bytes match certain values, adjust the max value and read the split value
        if ver0 == 0x44 && (ver1 == 0x20 || ver1 == 0x40) {
            if ver1 == 0x40 {
                max /= 4;
            }
            pointer = meta_offset + 562;
            split = LittleEndian::read_u16(&self.buffer[pointer..pointer + 2]) as usize;
            pointer += 2;
        }

        // get curve
        let curve = Self::nikon_read_curve(self.buffer.clone(), meta_offset, tiff_bps);

        // Reduce the max value by checking the curve array from the end
        // If the last two elements of the curve array are equal, decrease max by 1
        // This process continues until the last two elements are not equal or max is less than or equal to 2
        while max > 2 && curve[max - 2] == curve[max - 1] {
            max -= 1;
        }

        let mut huff_decoder = Self::make_decoder(NIKON_TREE[huff_tree as usize].as_ref());
        let mut raw_data = vec![vec![0; width]; height];
        let mut bit_state = BitState::new();
        let mut row = 0;
        let mut min = 0;
        let encoded_data = data_ifd
            .get_encoded_data(self.buffer.clone())
            .expect("Error getting encoded data");
        while row < height {
            if split != 0 && row == split {
                huff_decoder = Self::make_decoder(NIKON_TREE[(huff_tree + 1) as usize].as_ref());
                min = 16;
                max += min << 1;
            }
            for col in 0..width {
                let i = getbithuff(&mut bit_state, 0, Some(&huff_decoder), encoded_data.clone())
                    .unwrap();
                let len = (i & 15) as usize;
                let shl = (i >> 4) as usize;
                let mut diff = ((getbithuff(
                    &mut bit_state,
                    (len - shl) as i32,
                    None,
                    encoded_data.clone(),
                )
                .unwrap()
                    << 1)
                    + 1)
                    << shl
                    >> 1;
                if len > 0 && (diff & (1 << (len - 1))) == 0 {
                    diff -= (1 << len) - !shl;
                }
                if col < 2 {
                    vpred[row & 1][col] += diff as u16;
                    hpred[col] = vpred[row & 1][col];
                } else {
                    hpred[col & 1] += diff as u16;
                }
                if (hpred[col & 1] as usize + min) >= max {
                    return Err(Error::ParseError(
                        "Error parsing raw image data".to_string(),
                    ));
                }
                raw_data[row][col] = curve[lim(hpred[col & 1] as i16, 0, 0x3fff) as usize];
            }
            row += 1;
        }
        Ok(raw_data)
    }

    // fn parse_image_thumbnail(&mut self) -> Result<(), Error> {
    //     // Extract image thumbnail data from the file.
    // }
}

fn lim(value: i16, lower: i16, upper: i16) -> i16 {
    std::cmp::min(std::cmp::max(value, lower), upper)
}
pub struct BitState {
    bitbuf: u32,
    vbits: i32,
    reset: i32,
    pos: usize,
}

impl BitState {
    pub fn new() -> Self {
        BitState {
            bitbuf: 0,
            vbits: 0,
            reset: 0,
            pos: 0,
        }
    }
}

pub fn getbithuff(
    state: &mut BitState,
    nbits: i32,
    huff: Option<&[u16]>,
    buffer: Arc<[u8]>,
) -> Result<usize, &'static str> {
    let mut c: u32;

    if nbits > 25 {
        return Ok(0);
    }
    if nbits < 0 {
        state.bitbuf = 0;
        state.vbits = 0;
        state.reset = 0;
        return Ok(0);
    }
    if nbits == 0 || state.vbits < 0 {
        return Ok(0);
    }
    while state.reset == 0 && state.vbits < nbits {
        if state.pos < buffer.len() {
            c = buffer[state.pos] as u32;
            state.pos += 1;
        } else {
            break;
        }
    }
    c = if state.vbits == 0 {
        0
    } else {
        state.bitbuf << (32 - state.vbits) >> (32 - nbits)
    };
    if let Some(huff) = huff {
        state.vbits -= (huff[c as usize] >> 8) as i32;
        c = huff[c as usize] as u32;
    } else {
        state.vbits -= nbits;
    }
    if state.vbits < 0 {
        return Err("Error in getbithuff");
    }
    Ok(c as usize)
}
