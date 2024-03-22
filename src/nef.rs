use crate::huffmanv2::{BitPump, HuffTable};
use crate::ifd::Ifd;
use crate::utils::{read_beu32, read_leu16};
use byteorder::{ByteOrder, LittleEndian};
use std::hash::Hash;
use std::hash::{DefaultHasher, Hasher};
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
        let parsed_ifds = Ifd::parse_ifd(self.buffer.as_slice(), 0);
        Ok(parsed_ifds)
    }
    // fn parse_metadata(&mut self) -> Result<(), Error> {
    //     // Extract metadata from the file.
    // }

    pub fn parse_raw_image_data(&self) -> Result<Vec<u16>, Error> {
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

        let stripoffsets = data_ifd
            .get_entry(crate::ifd::IfdEntryTag::StripOffsets)
            .unwrap()
            .get_data_or_offset();

        let stripbytecounts = data_ifd
            .get_entry(crate::ifd::IfdEntryTag::StripByteCounts)
            .unwrap()
            .get_data_or_offset();

        let src = &self.buffer[stripoffsets..stripoffsets + stripbytecounts];
        println!(
            "StripOffsets: {} StripByteCounts: {}",
            stripoffsets, stripbytecounts
        );

        // Initialize split value
        let split = 0;

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
            .get_data_or_offset() as u16;

        // Calculate the total pointer for the 0x96 tag data
        let mut pointer = pointer_0x96 + makernote_ifd.offset_location;

        println!("Pointer total 0x96: {}\n", pointer);

        // Get the version bytes
        let (ver0, ver1) = (self.buffer[pointer], self.buffer[pointer + 1]);
        println!("Version: {:#02X} {:#02X}", ver0, ver1);
        pointer += 2;

        // Determine the Huffman compression type based on the version bytes and BitsPerSample
        let mut huff_select = if ver0 == 0x46 { 2 } else { 0 };
        if tiff_bps == 14 {
            huff_select += 3;
        }

        // Create the Huffman table
        let mut huff_table = create_hufftable(huff_select).expect("Error creating huffman table");

        // Read the vertical predictor values
        let mut vpred: [[u16; 2]; 2] = [[0; 2]; 2];
        let vpred_bytes = &self.buffer[pointer..pointer + 8];
        for i in 0..2 {
            for j in 0..2 {
                vpred[i][j] = LittleEndian::read_u16(&vpred_bytes[(i * 2 + j) * 2..]);
            }
        }
        let mut pred_up1 = [vpred[0][0] as i32, vpred[0][1] as i32];
        let mut pred_up2 = [vpred[1][0] as i32, vpred[1][1] as i32];
        println!("Predictors: {:?} {:?}", pred_up1, pred_up2);

        pointer += 8;

        // Get curve
        let curve = nikon_read_curve(self.buffer.as_slice(), &mut pointer, tiff_bps, ver0, ver1);
        let mut hasher = DefaultHasher::new();
        curve.table.hash(&mut hasher);
        println!("Curve hash: {:?}", hasher.finish());

        println!("src len: {}", src.len());
        println!(
            "src ten first and last: {:?} {:?}",
            &src[0..10],
            &src[src.len() - 10..]
        );

        let mut pump = BitPumpMSB::new(src);
        let mut random = pump.peek_bits(24);

        // Reduce the max value by checking the curve array from the end
        // If the last two elements of the curve array are equal, decrease max by 1
        // This process continues until the last two elements are not equal or max is less than or equal to 2
        let mut out = vec![0; width * height];
        println!("out.len: {}", out.len());
        let bps: u32 = tiff_bps as u32;
        println!("random: {:?}", random);
        for row in 0..height {
            // println!("Row: {}", row);
            if split > 0 && row == split {
                // This should not happen
                // huff_table =
                //     create_hufftable(huff_select + 1).expect("Error creating huffman table");
            }
            pred_up1[row & 1] += huff_table
                .huff_decode(&mut pump)
                .expect("Error decoding huffman");
            // println!("pred_up1: {}", pred_up1[row & 1]);
            pred_up2[row & 1] += huff_table
                .huff_decode(&mut pump)
                .expect("Error decoding huffman");
            // println!("pred_up2: {}", pred_up2[row & 1]);
            let mut pred_left1 = pred_up1[row & 1];
            // println!("pred_left1: {}", pred_left1);
            let mut pred_left2 = pred_up2[row & 1];
            // println!("pred_left2: {}", pred_left2);
            for col in (0..width).step_by(2) {
                if col > 0 {
                    pred_left1 += huff_table
                        .huff_decode(&mut pump)
                        .expect("Error decoding huffman");
                    // println!("pred_left1: {}", pred_left1);
                    pred_left2 += huff_table
                        .huff_decode(&mut pump)
                        .expect("Error decoding huffman");
                    // println!("pred_left2: {}", pred_left2);
                }
                out[row * width + col + 0] = curve.dither(clampbits(pred_left1, bps), &mut random);
                out[row * width + col + 1] = curve.dither(clampbits(pred_left2, bps), &mut random);
            }
        }

        Ok(out)
    }

    // fn parse_image_thumbnail(&mut self) -> Result<(), Error> {
    //     // Extract image thumbnail data from the file.
    // }
}

const NIKON_TREE: [[[u8; 16]; 3]; 6] = [
    [
        // 12-bit lossy
        [0, 0, 1, 5, 1, 1, 1, 1, 1, 1, 2, 0, 0, 0, 0, 0],
        [5, 4, 3, 6, 2, 7, 1, 0, 8, 9, 11, 10, 12, 0, 0, 0],
        [0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0],
    ],
    [
        // 12-bit lossy after split
        [0, 0, 1, 5, 1, 1, 1, 1, 1, 1, 2, 0, 0, 0, 0, 0],
        [6, 5, 5, 5, 5, 5, 4, 3, 2, 1, 0, 11, 12, 12, 0, 0],
        [3, 5, 3, 2, 1, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0],
    ],
    [
        // 12-bit lossless
        [0, 0, 1, 4, 2, 3, 1, 2, 0, 0, 0, 0, 0, 0, 0, 0],
        [5, 4, 6, 3, 7, 2, 8, 1, 9, 0, 10, 11, 12, 0, 0, 0],
        [0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0],
    ],
    [
        // 14-bit lossy
        [0, 0, 1, 4, 3, 1, 1, 1, 1, 1, 2, 0, 0, 0, 0, 0],
        [5, 6, 4, 7, 8, 3, 9, 2, 1, 0, 10, 11, 12, 13, 14, 0],
        [0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0],
    ],
    [
        // 14-bit lossy after split
        [0, 0, 1, 5, 1, 1, 1, 1, 1, 1, 1, 2, 0, 0, 0, 0],
        [8, 7, 7, 7, 7, 7, 6, 5, 4, 3, 2, 1, 0, 13, 14, 0],
        [0, 5, 4, 3, 2, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0],
    ],
    [
        // 14-bit lossless
        [0, 0, 1, 4, 2, 2, 3, 1, 2, 0, 0, 0, 0, 0, 0, 0],
        [7, 6, 8, 5, 9, 4, 10, 3, 11, 12, 2, 0, 1, 13, 14, 0],
        [0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0],
    ],
];

fn create_hufftable(num: usize) -> Result<HuffTable, String> {
    let mut htable = HuffTable::empty();

    for i in 0..15 {
        htable.bits[i] = NIKON_TREE[num][0][i] as u32;
        htable.huffval[i] = NIKON_TREE[num][1][i] as u32;
        htable.shiftval[i] = NIKON_TREE[num][2][i] as u32;
    }

    htable.initialize()?;
    println!("Hufftable: {:?}", htable);
    Ok(htable)
}

fn nikon_read_curve(
    buffer: &[u8],
    pointer: &mut usize,
    tiff_bps: u16,
    ver0: u8,
    ver1: u8,
) -> LookupTable {
    let mut points = [0 as u16; 1 << 16];
    for i in 0..points.len() {
        points[i] = i as u16;
    }
    let mut max = 1 << tiff_bps;
    let csize = read_leu16(buffer, pointer, false) as usize;
    // let mut split = 0 as usize;
    let step = if csize > 1 { max / (csize - 1) } else { 0 };
    if ver0 == 0x44 && ver1 == 0x20 && step > 0 {
        // should not happen
        // for i in 0..csize {
        //     points[i * step] = read_u16(buffer.clone(), pointer);
        // }
        // for i in 0..max {
        //     points[i] = ((points[i - i % step] as usize * (step - i % step)
        //         + points[i - i % step + step] as usize * (i % step))
        //         / step) as u16;
        // }
        // split = endian.ru16(meta, 562) as usize;
        println!("Should not happen")
    } else if ver0 != 0x46 && csize <= 0x4001 {
        for i in 0..csize {
            points[i] = read_leu16(buffer.clone(), pointer, false);
        }
        max = csize;
    }
    LookupTable::new(&points[0..max])
}

pub fn clampbits(val: i32, bits: u32) -> u16 {
    let max = (1 << bits) - 1;
    if val < 0 {
        0
    } else if val > max {
        max as u16
    } else {
        val as u16
    }
}
#[derive(Debug, Clone)]
pub struct LookupTable {
    pub table: Vec<(u16, u16, u16)>,
}

impl LookupTable {
    pub fn new(table: &[u16]) -> LookupTable {
        let mut tbl = vec![(0, 0, 0); table.len()];
        for i in 0..table.len() {
            let center = table[i];
            let lower = if i > 0 { table[i - 1] } else { center };
            let upper = if i < (table.len() - 1) {
                table[i + 1]
            } else {
                center
            };
            let base = if center == 0 {
                0
            } else {
                center - ((upper - lower + 2) / 4)
            };
            let delta = upper - lower;
            tbl[i] = (center, base, delta);
        }
        LookupTable { table: tbl }
    }

    //  pub fn lookup(&self, value: u16) -> u16 {
    //    let (val, _, _) = self.table[value as usize];
    //    val
    //  }

    pub fn dither(&self, value: u16, rand: &mut u32) -> u16 {
        let (_, sbase, sdelta) = self.table[value as usize];
        let base = sbase as u32;
        let delta = sdelta as u32;
        let pixel = base + ((delta * (*rand & 2047) + 1024) >> 12);
        *rand = 15700 * (*rand & 65535) + (*rand >> 16);
        pixel as u16
    }
}

pub struct BitPumpMSB<'a> {
    buffer: &'a [u8],
    pos: usize,
    bits: u64,
    nbits: u32,
}

impl<'a> BitPumpMSB<'a> {
    pub fn new(src: &'a [u8]) -> BitPumpMSB {
        BitPumpMSB {
            buffer: src,
            pos: 0,
            bits: 0,
            nbits: 0,
        }
    }
}

impl<'a> BitPump for BitPumpMSB<'a> {
    #[inline(always)]
    fn peek_bits(&mut self, num: u32) -> u32 {
        if num > self.nbits {
            let inbits: u64 = read_beu32(self.buffer.into(), &mut self.pos, true) as u64;
            self.bits = (self.bits << 32) | inbits;
            self.pos += 4;
            self.nbits += 32;
        }
        (self.bits >> (self.nbits - num)) as u32
    }

    fn consume_bits(&mut self, num: u32) {
        self.nbits -= num;
        self.bits &= (1 << self.nbits) - 1;
    }
}
