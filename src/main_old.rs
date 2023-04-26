use byteorder::WriteBytesExt;
use byteorder::{LittleEndian, ReadBytesExt};
// use core::slice::SlicePattern;
// use image::codecs::tiff;
// use image::io::Reader;
// use image::ImageFormat;
// use math::round;
use image::io::Reader as ImageReader;
use std::collections::HashMap;
use std::convert::AsMut;
use std::fmt::LowerHex;
use std::fs::File;
use std::io::Read;
use std::io::{Cursor, Write};

fn main() {
    let mut f = File::open("DSC_6400.NEF").unwrap();
    // let mut f = File::open("file_example_TIFF_5MB.tiff").unwrap();

    let mut buffer: Vec<u8> = Vec::new();
    f.read_to_end(&mut buffer).unwrap();

    let buffer_length = buffer.len();

    // let reader = Reader::with_format(Cursor::new(buffer), ImageFormat::Tiff);
    // let img = reader.decode().unwrap();
    // let img2 = Reader::new(Cursor::new(buffer)).with_guessed_format().unwrap().decode().unwrap();
    // _ = img.save("testi.jpg").unwrap();

    println!("Length: {}", buffer_length);

    // let byte_order = &buffer[0..2];
    // print_list_dec_and_hex(byte_order);

    // let magic_value = &buffer[2..4];
    // print_list_dec_and_hex(magic_value);

    // let tiff_offset = &buffer[4..8];
    // print_list_dec_and_hex(tiff_offset);

    // https://www.media.mit.edu/pia/Research/deepview/exif.html

    let eka_ifd = read_ifd(&buffer, &8);

    // tag id 8769
    let tag8769_ifd = read_ifd(&buffer, &bytes_to_num(&eka_ifd.entries[24].data_or_offset));

    let offset_to_childs = bytes_to_num(
        &eka_ifd
            .get_entry_by_name(TagName::OffsetToChildIfds)
            .unwrap()
            .data_or_offset,
    );
    println!("{:?}", offset_to_childs);
    let asd = &buffer[offset_to_childs..offset_to_childs + 12];
    println!("{:?}", asd);
    let asd1 = &bytes_to_num(&asd[0..4]);
    let asd2 = &bytes_to_num(&asd[4..8]);
    let asd3 = &bytes_to_num(&asd[8..12]);

    let thumb1 = read_ifd(&buffer, asd1);
    let oikeekuva = read_ifd(&buffer, asd2);
    let thumb2 = read_ifd(&buffer, asd3);

    eka_ifd.print_info(true);
    tag8769_ifd.print_info(true);
    thumb1.print_info(true);
    oikeekuva.print_info(true);
    thumb2.print_info(true);

    let makernote = tag8769_ifd.get_entry_by_name(TagName::MakerNote);
    let makernote_offset = bytes_to_num(&makernote.unwrap().data_or_offset);
    let maker_note_header = &buffer[makernote_offset..makernote_offset + 10];
    // print_list_dec_and_hex(&makernote.unwrap().raw_data);
    print_list_dec_and_hex(&maker_note_header);
    // println!("{:?}", makernote.unwrap().raw_data);
    let nikon_hex = [0x4E, 0x69, 0x6B, 0x6F, 0x6E];

    let makernote_tiff_header = &buffer[makernote_offset + 10..makernote_offset + 10 + 8];
    print_list_dec_and_hex(makernote_tiff_header);

    let makernote_first_entry_num_of_entries =
        &buffer[makernote_offset + 10 + 8..makernote_offset + 10 + 8 + 2];
    print_list_dec_and_hex(makernote_first_entry_num_of_entries);

    let makernote_ifd = read_ifd(&buffer, &(&(makernote_offset + 10 + 8)));
    makernote_ifd.print_info(true);

    let entry_0x8c = makernote_ifd.get_entry_by_hex(0x8c).unwrap();
    let entry_0x96 = makernote_ifd.get_entry_by_hex(0x96).unwrap();

    print_list_dec_and_hex(&entry_0x8c.raw_data);
    print_list_dec_and_hex(&entry_0x96.raw_data);

    let offset_0x8c = bytes_to_num(&entry_0x8c.data_or_offset);
    let num_of_comp_0x8c = &entry_0x8c.num_of_components;
    let data_0x8c = &buffer[makernote_offset + 10 + offset_0x8c
        ..makernote_offset + 10 + offset_0x8c + num_of_comp_0x8c];
    println!("{:?}", data_0x8c);

    let offset_0x96 = bytes_to_num(&entry_0x96.data_or_offset);
    let num_of_comp_0x96 = &entry_0x96.num_of_components;
    let data_0x96 = &buffer[makernote_offset + 10 + offset_0x96
        ..makernote_offset + 10 + offset_0x96 + num_of_comp_0x96];
    println!("{:?}", data_0x96);
    // The quantization tables are at 0x8c and 0x96 tag from the MakerNote

    let image_length = bytes_to_num(
        &eka_ifd
            .get_entry_by_name(TagName::ImageLength)
            .unwrap()
            .data_or_offset,
    ) as f32;

    let rows_per_strip = bytes_to_num(
        &eka_ifd
            .get_entry_by_name(TagName::RowsPerStrip)
            .unwrap()
            .data_or_offset,
    ) as f32;

    let samples_per_pixel = bytes_to_num(
        &eka_ifd
            .get_entry_by_name(TagName::SamplesPerPixel)
            .unwrap()
            .data_or_offset,
    ) as f32;

    let strips_per_image = ((image_length + rows_per_strip - 1_f32) / rows_per_strip).floor();
    println!("Strips per image: {}", strips_per_image);

    let strip_byte_count = bytes_to_num(
        &eka_ifd
            .get_entry_by_name(TagName::StripByteCounts)
            .unwrap()
            .data_or_offset,
    );
    println!("Strip byte count: {:?}", strip_byte_count);

    let strip_offsets = bytes_to_num(
        &eka_ifd
            .get_entry_by_name(TagName::StripOffSets)
            .unwrap()
            .data_or_offset,
    );
    println!("Strip offsets: {:?}", strip_offsets);

    let n = samples_per_pixel * strips_per_image;
    println!("N: {}", n);

    // let thumbnail_data = &buffer[strip_offsets..strip_offsets + strip_byte_count];
    let mut file1 = File::create("thumbnail1.jpg").unwrap();
    file1
        .write_all(thumb1.get_thumbnail_data(&buffer).unwrap())
        .unwrap();
    let mut file2 = File::create("thumbnail2.jpg").unwrap();
    file2
        .write_all(thumb2.get_thumbnail_data(&buffer).unwrap())
        .unwrap();

    let image_data = oikeekuva.get_image_data(&buffer).unwrap();

    println!("{:?}", image_data.len());
}

fn read_ifd(buffer: &Vec<u8>, start_of_ifd: &usize) -> Ifd {
    let start_of_ifd = start_of_ifd.clone();
    let mut first_two: &[u8] = &[buffer[start_of_ifd], buffer[start_of_ifd + 1], 0, 0];
    let num_of_ifds = first_two.read_u32::<LittleEndian>().unwrap() as usize;
    const DIR_ENTRY_LEN: usize = 12;
    let start_of_entries: usize = start_of_ifd as usize + 2;
    let end_of_entries: usize = start_of_entries + DIR_ENTRY_LEN * num_of_ifds as usize;
    let end_of_ifd = end_of_entries + 4;
    let ifd = &buffer[start_of_entries..end_of_entries];
    let chunk_iterator = ifd.chunks(DIR_ENTRY_LEN as usize);

    let ifds: Vec<IfdEntry> = chunk_iterator
        .enumerate()
        .map(|(i, chunk)| {
            let index_in_ifd = i;
            let tag = Tag::map_tag(bytes_to_num(&chunk[0..2]) as usize);
            let data_format_num = bytes_to_num(&chunk[2..4]) as usize;
            let data_format = DataFormat::map_data_format(data_format_num);
            let num_of_components = bytes_to_num(&chunk[4..8]);
            let data_or_offset = [chunk[8], chunk[9], chunk[10], chunk[11]];
            let offset = match data_format.n_of_bytes as usize * num_of_components as usize {
                result if result > 4 => true,
                result if result <= 4 => false,
                _ => panic!("match error"),
            };
            let data_int = bytes_to_num(&data_or_offset);
            let raw_data: [u8; 12] = chunk[0..12].try_into().unwrap();
            IfdEntry {
                index_in_ifd,
                raw_data,
                tag,
                data_format,
                num_of_components,
                data_or_offset,
                data_int,
                offset,
            }
        })
        .collect();

    let offset_to_next_ifd = bytes_to_num(&buffer[end_of_ifd - 4..end_of_ifd]);
    let start = start_of_ifd;
    let end = end_of_ifd;

    return Ifd {
        // data: buffer.clone(),
        start,
        end,
        num_of_entries: num_of_ifds,
        entries: ifds,
        offset_to_next_ifd,
    };
}

#[derive(Debug, Copy, Clone, Eq, PartialEq)]
pub struct IfdEntry {
    index_in_ifd: usize,
    raw_data: [u8; 12],
    tag: Tag,
    data_format: DataFormat,
    num_of_components: usize,
    data_or_offset: [u8; 4],
    data_int: usize,
    offset: bool,
}
#[derive(Debug, Clone)]
pub struct Ifd {
    // data: Vec<u8>,
    start: usize,
    end: usize,
    num_of_entries: usize,
    entries: Vec<IfdEntry>,
    offset_to_next_ifd: usize,
}

impl Ifd {
    // tag mappaus
    // thumbnail kaivaminen
    // raakakuvan kaivaminen
    //
    fn print_info(&self, print_all: bool) {
        fn print_ifd_entry(entry: &IfdEntry) {
            print!("{:?} ", entry.index_in_ifd);
            print!("Tag ID: {:#02X}, {:?}. ", entry.tag.id, entry.tag.id);
            print!("Tag name: {:?}. ", entry.tag.name);
            print!(
                "Data format: {:?}, {} byte(s). ",
                entry.data_format.name, entry.data_format.n_of_bytes
            );
            print!("Number of components: {:?}. ", entry.num_of_components);
            print!(
                "Data value or offset to data value: {:#02X}, {:?}, {:?}. ",
                bytes_to_num(&entry.data_or_offset),
                bytes_to_num(&entry.data_or_offset),
                entry.data_or_offset
            );
            println!("Offset: {}.", entry.offset);
        }
        println!("-------------------------");
        println!("Start of IFD: {:?}", &self.start);
        println!("End of IFD: {:?}", &self.end);
        for ifd_entry in &self.entries {
            if print_all || ifd_entry.tag.name != TagName::Undefined {
                print_ifd_entry(ifd_entry);
            }
        }
        if self.offset_to_next_ifd == 0 {
            println!("No linked IFD! Value {:?}", &self.offset_to_next_ifd)
        } else {
            println!("Offset to next IFD: {:?}", &self.offset_to_next_ifd)
        }
        println!("-------------------------");
    }

    fn get_entry_by_name(&self, tag_name: TagName) -> Option<&IfdEntry> {
        self.entries.iter().find(|entry| entry.tag.name == tag_name)
    }

    fn get_entry_by_hex(&self, tag_id: usize) -> Option<&IfdEntry> {
        self.entries.iter().find(|entry| entry.tag.id == tag_id)
    }

    fn get_thumbnail_data<'a>(&'a self, buffer: &'a Vec<u8>) -> Option<&[u8]> {
        let jpeg_offset_entry = self
            .entries
            .iter()
            .find(|entry| entry.tag.name == TagName::JpegIFOffset);
        let jpeg_bytecount_entry = self
            .entries
            .iter()
            .find(|entry| entry.tag.name == TagName::JpegIFByteCount);
        let _jpeg_compression_entry = self
            .entries
            .iter()
            .find(|entry| entry.tag.name == TagName::Compression);

        if jpeg_offset_entry.is_some()
            && jpeg_bytecount_entry.is_some()
            && _jpeg_compression_entry.is_some()
        {
            let jpeg_offset = jpeg_offset_entry?.data_int;
            let jpeg_byte_count = jpeg_bytecount_entry?.data_int;
            let start_byte = bytes_to_num(&buffer.as_slice()[jpeg_offset..jpeg_offset + 2]);
            match start_byte {
                0xD8FF => Some(&buffer.as_slice()[jpeg_offset..jpeg_offset + jpeg_byte_count]),
                _ => None,
            }
        } else {
            println!("thumbnail data error");
            None
        }
    }

    fn get_image_data<'a>(&'a self, buffer: &'a Vec<u8>) -> Option<&[u8]> {
        let offset = bytes_to_num(
            &self
                .get_entry_by_name(TagName::StripOffSets)
                .unwrap()
                .data_or_offset,
        );
        let strip_byte_counts = bytes_to_num(
            &self
                .get_entry_by_name(TagName::StripByteCounts)
                .unwrap()
                .data_or_offset,
        );
        let image_data = buffer.get(offset as usize..(offset + strip_byte_counts) as usize)?;

        Some(image_data)
    }
}

#[derive(Debug, Copy, Clone, PartialEq, Eq)]
enum DataFormatName {
    UnsignedByte = 1,
    AsciiStrings,
    UnsignedShort,
    UnsignedLong,
    UnsignedRational,
    SignedByte,
    Undefined,
    SignedShort,
    SignedLong,
    SignedRational,
    SingleFloat,
    DoubleFloat,
}
#[derive(Debug, Copy, Clone, PartialEq, Eq)]
struct DataFormat {
    value: usize,
    name: DataFormatName,
    n_of_bytes: usize,
}

impl DataFormat {
    fn map_data_format(num: usize) -> DataFormat {
        let mut data_format = DataFormat {
            value: num,
            name: DataFormatName::Undefined,
            n_of_bytes: 0,
        };
        match num {
            1 => {
                data_format.name = DataFormatName::UnsignedByte;
                data_format.n_of_bytes = 1;
                data_format
            }
            2 => {
                data_format.name = DataFormatName::AsciiStrings;
                data_format.n_of_bytes = 1;
                data_format
            }
            3 => {
                data_format.name = DataFormatName::UnsignedShort;
                data_format.n_of_bytes = 2;
                data_format
            }
            4 => {
                data_format.name = DataFormatName::UnsignedLong;
                data_format.n_of_bytes = 4;
                data_format
            }
            5 => {
                data_format.name = DataFormatName::UnsignedRational;
                data_format.n_of_bytes = 8;
                data_format
            }
            6 => {
                data_format.name = DataFormatName::SignedByte;
                data_format.n_of_bytes = 1;
                data_format
            }
            7 => {
                // println!("Undefined data format!");
                data_format.n_of_bytes = 1;
                data_format
            }
            8 => {
                data_format.name = DataFormatName::SignedShort;
                data_format.n_of_bytes = 2;
                data_format
            }
            9 => {
                data_format.name = DataFormatName::SignedLong;
                data_format.n_of_bytes = 4;
                data_format
            }
            10 => {
                data_format.name = DataFormatName::SignedRational;
                data_format.n_of_bytes = 8;
                data_format
            }
            11 => {
                data_format.name = DataFormatName::SingleFloat;
                data_format.n_of_bytes = 4;
                data_format
            }
            12 => {
                data_format.name = DataFormatName::DoubleFloat;
                data_format.n_of_bytes = 8;
                data_format
            }
            _ => {
                println!("");
                panic!("No match in data format! {}", num);
                // data_format.name = DataFormatName::Undefined;
                // data_format
            }
        }
    }
}

#[derive(Debug, Copy, Clone, Eq, PartialEq)]
enum TagName {
    Undefined,
    NewSubFileType = 0x00FE,
    ImageWidth = 0x0100,
    ImageLength = 0x0101,
    BitsPerSample = 0x0102,
    Compression = 0x0103,
    PhotoMetricInterpretation = 0x106,
    StripOffSets = 0x0111,
    SamplesPerPixel = 0x0115,
    RowsPerStrip = 0x0116,
    StripByteCounts = 0x0117,
    PlanarConfiguration = 0x011C,
    ScannerManufacturer = 0x010F,
    ScannerModel = 0x0110,
    XResolution = 0x011A,
    YResolution = 0x011B,
    Orientation = 0x0112,
    ResolutionUnit = 0x0128,
    SoftwareName = 0x0131,
    DateTime = 0x0132,
    Artist = 0x013B,
    OffsetToChildIfds = 0x014A,
    ReferenceBlackNWhite = 0x0214,
    JpegIFOffset = 0x0201,
    JpegIFByteCount = 0x0202,
    YCbCrPositioning = 0x0213,
    MakerNote = 0x927C,
}

#[derive(Debug, Copy, Clone, Eq, PartialEq)]
struct Tag {
    id: usize,
    name: TagName,
    format: DataFormatName,
    num_of_comp: usize,
}

impl Tag {
    fn map_tag(num: usize) -> Tag {
        let mut tag = Tag {
            id: num,
            name: TagName::Undefined,
            format: DataFormatName::Undefined,
            num_of_comp: 1,
        };
        match num {
            0x0213 => {
                tag.name = TagName::YCbCrPositioning;
                tag.format = DataFormatName::UnsignedShort;
            }
            0x0201 => {
                tag.name = TagName::JpegIFOffset;
                tag.format = DataFormatName::UnsignedLong;
            }
            0x0202 => {
                tag.name = TagName::JpegIFByteCount;
                tag.format = DataFormatName::UnsignedLong;
            }
            0x0214 => {
                tag.name = TagName::ReferenceBlackNWhite;
                tag.format = DataFormatName::UnsignedRational;
                tag.num_of_comp = 6;
            }
            0x014A => {
                tag.name = TagName::OffsetToChildIfds;
                tag.format = DataFormatName::UnsignedLong;
            }
            0x013B => {
                tag.name = TagName::Artist;
                tag.format = DataFormatName::AsciiStrings
            }
            0x0132 => {
                tag.name = TagName::DateTime;
                tag.format = DataFormatName::AsciiStrings;
                tag.num_of_comp = 20;
            }
            0x0131 => {
                tag.name = TagName::SoftwareName;
                tag.format = DataFormatName::AsciiStrings;
            }
            0x0128 => {
                tag.name = TagName::ResolutionUnit;
                tag.format = DataFormatName::UnsignedShort;
            }
            0x011B => {
                tag.name = TagName::YResolution;
                tag.format = DataFormatName::UnsignedRational;
            }
            0x0112 => {
                tag.name = TagName::Orientation;
                tag.format = DataFormatName::UnsignedShort;
            }
            0x011A => {
                tag.name = TagName::XResolution;
                tag.format = DataFormatName::UnsignedRational;
            }
            0x0110 => {
                tag.name = TagName::ScannerModel;
                tag.format = DataFormatName::AsciiStrings;
            }
            0x010F => {
                tag.name = TagName::ScannerManufacturer;
                tag.format = DataFormatName::AsciiStrings;
            }
            0x00FE => {
                tag.name = TagName::NewSubFileType;
                tag.format = DataFormatName::UnsignedLong;
            }
            0x0100 => {
                tag.name = TagName::ImageWidth;
                tag.format = DataFormatName::UnsignedLong;
            }
            0x0101 => {
                tag.name = TagName::ImageLength;
                tag.format = DataFormatName::UnsignedLong;
            }
            0x0102 => {
                tag.name = TagName::BitsPerSample;
                tag.format = DataFormatName::UnsignedShort;
                tag.num_of_comp = 3;
            }
            0x0103 => {
                tag.name = TagName::Compression;
                tag.format = DataFormatName::UnsignedShort
            }
            0x0106 => {
                tag.name = TagName::PhotoMetricInterpretation;
                tag.format = DataFormatName::UnsignedShort;
            }
            0x0111 => {
                tag.name = TagName::StripOffSets;
                tag.format = DataFormatName::UnsignedShort
            }
            0x0115 => {
                tag.name = TagName::SamplesPerPixel;
                tag.format = DataFormatName::UnsignedShort;
            }
            0x0116 => {
                tag.name = TagName::RowsPerStrip;
                tag.format = DataFormatName::UnsignedLong;
            }
            0x0117 => {
                tag.name = TagName::StripByteCounts;
                tag.format = DataFormatName::UnsignedLong;
            }
            0x011C => {
                tag.name = TagName::PlanarConfiguration;
                tag.format = DataFormatName::UnsignedShort;
            }
            0x927C => {
                tag.name = TagName::MakerNote;
                tag.format = DataFormatName::Undefined;
            }
            _ => tag.name = TagName::Undefined,
        }
        tag
    }
}
// 0x0100 ImageWidth
// 0x0101 ImageLength
// 0x0111 StripOffset, kuva koostuu useista stripeistä
// 0x0116 RowsPerStrip.
// 0x0117 StripByteCounts
// StripsPerImage = floor ((ImageLength + RowsPerStrip - 1) / RowsPerStrip).

// struct ExifIfd {
//     tags: Vec<Tag>
// }

// impl ExifIfd {
//     fn map_ifd_exif(buffer: &Vec<u8>, first_ifd: &Ifd) {
//         // tag id 0x8769 ->
//         let exif_offset = first_ifd.ifds.iter().find(|entry| entry.tag_id == 0x8769).unwrap();
//         let exif_sub_ifd = read_ifd(buffer, &exif_offset.data_or_offset);

//         for entry in exif_sub_ifd.ifds {
//             let mut tag = Tag {
//                 id: entry.tag_id,
//                 name: "".to_string(),
//                 data: [0]
//             };
//         }
//     }
//     fn map_tag_name(tag_id: usize) -> String {
//         match tag_id {
//             0xA217 => String::from("SensingMethod"),
//             _ => String::from("Unknown tag name")

//         }
//     }
// }

// fn bytes_to_num(bytes: &[u8]) -> usize {
//     let mut template: [u8; 8] = [0, 0, 0, 0, 0, 0, 0, 0];
//     for (i, byte) in bytes.iter().enumerate() {
//         template[i] = *byte;
//     }
//     let mut template: &[u8] = &template;
//     let num = template.read_u32::<LittleEndian>().unwrap() as usize;
//     return num;
// }

fn bytes_to_num(bytes: &[u8]) -> usize {
    let mut template: [u8; 4] = [0, 0, 0, 0];
    template[..bytes.len()].copy_from_slice(bytes);
    let mut cursor = std::io::Cursor::new(&template);
    let num = cursor.read_u32::<LittleEndian>().unwrap() as usize;
    num
}

// fn print_ifd_entry(ifd: &IfdEntry) {
//     print!("Tag ID: {:#02X}, {:?}. ", ifd.tag_id, ifd.tag_id);
//     print!(
//         "Data format: {:?}, {} byte(s). ",
//         ifd.data_format.name, ifd.data_format.n_of_bytes
//     );
//     print!("Number of components: {:?}. ", ifd.num_of_components);
//     print!(
//         "Data value or offset to data value: {:#02X}, {:?}. ",
//         ifd.data_or_offset, ifd.data_or_offset
//     );
//     println!("Offset: {}.", ifd.offset);
// }

fn print_list_dec_and_hex(list: &[u8]) {
    let list_len = list.len();
    println!("{:?}", list);
    print!("[");
    for (i, elem) in list.iter().enumerate() {
        print!("{:#02X}", elem);
        if i + 1 < list_len {
            print!(", ");
        }
    }
    print!("]");
    println!("");
}

fn print_dec_and_hex(item: &u8) {
    println!("{}", item);
    println!("{:#02X}", item);
}

fn clone_into_array<A, T>(slice: &[T]) -> A
where
    A: Default + AsMut<[T]>,
    T: Clone,
{
    let mut a = A::default();
    <A as AsMut<[T]>>::as_mut(&mut a).clone_from_slice(slice);
    a
}
