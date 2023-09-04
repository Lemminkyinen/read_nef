use byteorder::{ByteOrder, LittleEndian};

use crate::utils::bytes_to_num;

macro_rules! alloc_image_plain {
    ($width:expr, $height:expr, $dummy: expr) => {{
        if $width * $height > 500000000 || $width > 50000 || $height > 50000 {
            panic!(
                "rawloader: surely there's no such thing as a >500MP or >50000 px wide/tall image!"
            );
        }
        if $dummy {
            vec![0]
        } else {
            vec![0; $width * $height]
        }
    }};
}

macro_rules! alloc_image {
    ($width:expr, $height:expr, $dummy: expr) => {{
        let out = alloc_image_plain!($width, $height, $dummy);
        if $dummy {
            return out;
        }
        out
    }};
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum IfdType {
    NikonMakerNote,
    Exif,
    Other,
}
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct Ifd {
    pub ifd_type: IfdType,
    pub ifd_buffer_start_point: usize,
    pub entries: Vec<IfdEntry>,
}

#[derive(Debug, Copy, Clone, PartialEq, Eq)]
pub struct IfdEntry {
    pub ifd_buffer_start_point: usize,
    pub tag: IfdEntryTag,
    pub data_type: IfdEntryType,
    pub data_or_offset: [u8; 4],
    pub data_length: usize,
    pub offset: bool,
    pub raw_entry: [u8; 12],
}

#[derive(Debug, Copy, Clone, PartialEq, Eq)]
pub enum IfdEntryTag {
    NewSubfileType,
    ImageWidth,
    ImageLength,
    BitsPerSample,
    Compression,
    PhotometricInterpretation,
    Make,
    Model,
    StripOffsets,
    Orientation,
    SamplesPerPixel,
    RowsPerStrip,
    StripByteCounts,
    XResolution,
    YResolution,
    PlanarConfiguration,
    ResolutionUnit,
    Software,
    DateTime,
    Artist,
    SubIFDS,
    JpgFromRawStart,
    JpgFromRawLength,
    YCbCrPositioning,
    ReferenceBlackNWhite,
    XMLMetaData,
    CFARepeatPatternDim,
    CFAPattern,
    CopyRight,
    ExposureTime,
    FNumber,
    ExifIFDPointer,
    ExposureProgram,
    GPSInfo,
    ISOSpeedRatings,
    SensitivityType,
    RecommendedExposureIndex,
    DateTimeOriginal,
    DateTimeDigitized,
    NikonPictureControlVersion, // kysymysmerkki
    NikonPictureControlName,    // kysymysmerkki
    NikonPictureControlComment, // kysymysmerkki
    ExposureBias,
    MaxAperture,
    ExposureMeteringMode,
    LightSource,
    Flash,
    FocalLength,
    TIFFEPStandardID,
    SensingMethod,
    MakerNote,
    UserComment,
    SubSecTime,
    SubSecTimeOriginal,
    SubSecTimeDigitized,
    NikonAFInfo2, // kysymysmerkki
    FileSource,
    NikonCaptureVersion,
    NikonCaptureOffset,
    NikonScanIFD,
    NikonCaptureEditVersion,
    NikonCaptureEditCount,
    NikonCaptureEditApplied,
    NikonCaptureToneCurve,
    NikonCaptureSharpener,
    NikonCaptureColorMode,
    NikonCaptureColorHue,
    NikonCaptureSaturation,
    NikonCaptureNoiseReduction,
    // Add other tags as needed.
    Unknown(usize),
}

#[derive(Debug, Copy, Clone, PartialEq, Eq)]
pub enum IfdEntryType {
    UnsignedByte(u8),
    AsciiString(u8),
    UnsignedShort(u8),
    UnsignedLong(u8),
    UnsignedRational(u8),
    SignedByte(u8),
    Undefined(u8),
    SignedShort(u8),
    SignedLong(u8),
    SignedRational(u8),
    SingleFloat(u8),
    DoubleFloat(u8),
    Unknown(u8),
}

#[derive(Debug, Copy, Clone, PartialEq, Eq)]
pub enum TagParam {
    IfdEntry(IfdEntryTag),
    U8(u8),
}

impl From<usize> for IfdEntryTag {
    fn from(tag_value: usize) -> Self {
        match tag_value {
            0xFE => IfdEntryTag::NewSubfileType,
            0x100 => IfdEntryTag::ImageWidth,
            0x101 => IfdEntryTag::ImageLength,
            0x102 => IfdEntryTag::BitsPerSample,
            0x103 => IfdEntryTag::Compression,
            0x106 => IfdEntryTag::PhotometricInterpretation,
            0x10F => IfdEntryTag::Make,
            0x110 => IfdEntryTag::Model,
            0x111 => IfdEntryTag::StripOffsets,
            0x112 => IfdEntryTag::Orientation,
            0x115 => IfdEntryTag::SamplesPerPixel,
            0x116 => IfdEntryTag::RowsPerStrip,
            0x117 => IfdEntryTag::StripByteCounts,
            0x11A => IfdEntryTag::XResolution,
            0x11B => IfdEntryTag::YResolution,
            0x11C => IfdEntryTag::PlanarConfiguration,
            0x128 => IfdEntryTag::ResolutionUnit,
            0x131 => IfdEntryTag::Software,
            0x132 => IfdEntryTag::DateTime,
            0x13B => IfdEntryTag::Artist,
            0x14A => IfdEntryTag::SubIFDS,
            0x201 => IfdEntryTag::JpgFromRawStart,
            0x202 => IfdEntryTag::JpgFromRawLength,
            0x213 => IfdEntryTag::YCbCrPositioning,
            0x214 => IfdEntryTag::ReferenceBlackNWhite,
            0x2BC => IfdEntryTag::XMLMetaData,
            0x828D => IfdEntryTag::CFARepeatPatternDim,
            0x828E => IfdEntryTag::CFAPattern,
            0x8298 => IfdEntryTag::CopyRight,
            0x829A => IfdEntryTag::ExposureTime,
            0x829D => IfdEntryTag::FNumber,
            0x8769 => IfdEntryTag::ExifIFDPointer,
            0x8822 => IfdEntryTag::ExposureProgram,
            0x8825 => IfdEntryTag::GPSInfo,
            0x8827 => IfdEntryTag::ISOSpeedRatings,
            0x8830 => IfdEntryTag::SensitivityType,
            0x8832 => IfdEntryTag::RecommendedExposureIndex,
            0x9003 => IfdEntryTag::DateTimeOriginal,
            0x9004 => IfdEntryTag::DateTimeDigitized,
            0x9010 => IfdEntryTag::NikonPictureControlVersion,
            0x9011 => IfdEntryTag::NikonPictureControlName,
            0x9012 => IfdEntryTag::NikonPictureControlComment,
            0x9204 => IfdEntryTag::ExposureBias,
            0x9205 => IfdEntryTag::MaxAperture,
            0x9207 => IfdEntryTag::ExposureMeteringMode,
            0x9208 => IfdEntryTag::LightSource,
            0x9209 => IfdEntryTag::Flash,
            0x920A => IfdEntryTag::FocalLength,
            0x9216 => IfdEntryTag::TIFFEPStandardID,
            0x9217 => IfdEntryTag::SensingMethod,
            0x927C => IfdEntryTag::MakerNote,
            0x9286 => IfdEntryTag::UserComment,
            0x9290 => IfdEntryTag::SubSecTime,
            0x9291 => IfdEntryTag::SubSecTimeOriginal,
            0x9292 => IfdEntryTag::SubSecTimeDigitized,
            0xA217 => IfdEntryTag::NikonAFInfo2,
            0xA300 => IfdEntryTag::FileSource,
            0xA301 => IfdEntryTag::NikonCaptureVersion,
            0xA302 => IfdEntryTag::NikonCaptureOffset,
            0xA401 => IfdEntryTag::NikonScanIFD,
            0xA402 => IfdEntryTag::NikonCaptureEditVersion,
            0xA403 => IfdEntryTag::NikonCaptureEditCount,
            0xA405 => IfdEntryTag::NikonCaptureEditApplied,
            0xA406 => IfdEntryTag::NikonCaptureToneCurve,
            0xA407 => IfdEntryTag::NikonCaptureSharpener,
            0xA408 => IfdEntryTag::NikonCaptureColorMode,
            0xA409 => IfdEntryTag::NikonCaptureColorHue,
            0xA40A => IfdEntryTag::NikonCaptureSaturation,
            0xA40C => IfdEntryTag::NikonCaptureNoiseReduction,
            // Add other tag value mappings as needed.
            _ => IfdEntryTag::Unknown(tag_value),
        }
    }
}

impl IfdEntryTag {
    pub fn usize_value(&self) -> usize {
        match self {
            IfdEntryTag::NewSubfileType => 0xFE,
            IfdEntryTag::ImageWidth => 0x100,
            IfdEntryTag::ImageLength => 0x101,
            IfdEntryTag::BitsPerSample => 0x102,
            IfdEntryTag::Compression => 0x103,
            IfdEntryTag::PhotometricInterpretation => 0x106,
            IfdEntryTag::Make => 0x10F,
            IfdEntryTag::Model => 0x110,
            IfdEntryTag::StripOffsets => 0x111,
            IfdEntryTag::Orientation => 0x112,
            IfdEntryTag::SamplesPerPixel => 0x115,
            IfdEntryTag::RowsPerStrip => 0x116,
            IfdEntryTag::StripByteCounts => 0x117,
            IfdEntryTag::XResolution => 0x11A,
            IfdEntryTag::YResolution => 0x11B,
            IfdEntryTag::PlanarConfiguration => 0x11C,
            IfdEntryTag::ResolutionUnit => 0x128,
            IfdEntryTag::Software => 0x131,
            IfdEntryTag::DateTime => 0x132,
            IfdEntryTag::Artist => 0x13B,
            IfdEntryTag::SubIFDS => 0x14A,
            IfdEntryTag::JpgFromRawStart => 0x201,
            IfdEntryTag::JpgFromRawLength => 0x202,
            IfdEntryTag::YCbCrPositioning => 0x213,
            IfdEntryTag::ReferenceBlackNWhite => 0x214,
            IfdEntryTag::XMLMetaData => 0x2BC,
            IfdEntryTag::CFARepeatPatternDim => 0x828D,
            IfdEntryTag::CFAPattern => 0x828E,
            IfdEntryTag::CopyRight => 0x8298,
            IfdEntryTag::ExposureTime => 0x829A,
            IfdEntryTag::FNumber => 0x829D,
            IfdEntryTag::ExifIFDPointer => 0x8769,
            IfdEntryTag::ExposureProgram => 0x8822,
            IfdEntryTag::GPSInfo => 0x8825,
            IfdEntryTag::ISOSpeedRatings => 0x8827,
            IfdEntryTag::SensitivityType => 0x8830,
            IfdEntryTag::RecommendedExposureIndex => 0x8832,
            IfdEntryTag::DateTimeOriginal => 0x9003,
            IfdEntryTag::DateTimeDigitized => 0x9004,
            IfdEntryTag::NikonPictureControlVersion => 0x9010,
            IfdEntryTag::NikonPictureControlName => 0x9011,
            IfdEntryTag::NikonPictureControlComment => 0x9012,
            IfdEntryTag::ExposureBias => 0x9204,
            IfdEntryTag::MaxAperture => 0x9205,
            IfdEntryTag::ExposureMeteringMode => 0x9207,
            IfdEntryTag::LightSource => 0x9208,
            IfdEntryTag::Flash => 0x9209,
            IfdEntryTag::FocalLength => 0x920A,
            IfdEntryTag::TIFFEPStandardID => 0x9216,
            IfdEntryTag::SensingMethod => 0x9217,
            IfdEntryTag::MakerNote => 0x927C,
            IfdEntryTag::UserComment => 0x9286,
            IfdEntryTag::SubSecTime => 0x9290,
            IfdEntryTag::SubSecTimeOriginal => 0x9291,
            IfdEntryTag::SubSecTimeDigitized => 0x9292,
            IfdEntryTag::NikonAFInfo2 => 0xA217,
            IfdEntryTag::FileSource => 0xA300,
            IfdEntryTag::NikonCaptureVersion => 0xA301,
            IfdEntryTag::NikonCaptureOffset => 0xA302,
            IfdEntryTag::NikonScanIFD => 0xA401,
            IfdEntryTag::NikonCaptureEditVersion => 0xA402,
            IfdEntryTag::NikonCaptureEditCount => 0xA403,
            IfdEntryTag::NikonCaptureEditApplied => 0xA405,
            IfdEntryTag::NikonCaptureToneCurve => 0xA406,
            IfdEntryTag::NikonCaptureSharpener => 0xA407,
            IfdEntryTag::NikonCaptureColorMode => 0xA408,
            IfdEntryTag::NikonCaptureColorHue => 0xA409,
            IfdEntryTag::NikonCaptureSaturation => 0xA40A,
            IfdEntryTag::NikonCaptureNoiseReduction => 0xA40C,
            IfdEntryTag::Unknown(value) => *value as usize,
        }
    }
}

impl From<usize> for IfdEntryType {
    fn from(data_type: usize) -> Self {
        match data_type {
            1 => IfdEntryType::UnsignedByte(1),
            2 => IfdEntryType::AsciiString(1),
            3 => IfdEntryType::UnsignedShort(2),
            4 => IfdEntryType::UnsignedLong(4),
            5 => IfdEntryType::UnsignedRational(8),
            6 => IfdEntryType::SignedByte(1),
            7 => IfdEntryType::Undefined(1),
            8 => IfdEntryType::SignedShort(2),
            9 => IfdEntryType::SignedLong(4),
            10 => IfdEntryType::SignedRational(8),
            11 => IfdEntryType::SingleFloat(4),
            12 => IfdEntryType::DoubleFloat(8),
            _ => IfdEntryType::Unknown(data_type as u8),
        }
    }
}

impl IfdEntryType {
    pub fn bytes_per_component(&self) -> u8 {
        match self {
            IfdEntryType::UnsignedByte(bytes)
            | IfdEntryType::AsciiString(bytes)
            | IfdEntryType::SignedByte(bytes)
            | IfdEntryType::Undefined(bytes) => *bytes,
            IfdEntryType::UnsignedShort(bytes) | IfdEntryType::SignedShort(bytes) => *bytes,
            IfdEntryType::UnsignedLong(bytes)
            | IfdEntryType::SignedLong(bytes)
            | IfdEntryType::SingleFloat(bytes) => *bytes,
            IfdEntryType::UnsignedRational(bytes)
            | IfdEntryType::SignedRational(bytes)
            | IfdEntryType::DoubleFloat(bytes) => *bytes,
            IfdEntryType::Unknown(bytes) => *bytes,
        }
    }
}

impl Ifd {
    pub fn parse_ifd(buffer: &[u8], offset: usize) -> Vec<Self> {
        // https://www.ozhiker.com/electronics/pjmt/jpeg_info/nikon_mn.html

        let mut ifds = Vec::new();
        let mut internal_offset;
        let mut nikon_mapping = false;
        let mut ifd_offset = None;
        let ifd_buffer = &buffer[offset..];

        let nikon_patterns = vec![
            [0x4E, 0x69, 0x6B, 0x6F, 0x6E, 0x00, 0x02, 0x00, 0x00, 0x00],
            [0x4E, 0x69, 0x6B, 0x6F, 0x6E, 0x00, 0x02, 0x10, 0x00, 0x00],
            [0x4E, 0x69, 0x6B, 0x6F, 0x6E, 0x00, 0x02, 0x11, 0x00, 0x00],
        ];

        // println!("{:?}", &buffer[0..10]);
        if ifd_buffer[0..8] == [73, 73, 42, 0, 8, 0, 0, 0] {
            internal_offset = 8;
        } else if nikon_patterns
            .iter()
            .any(|pattern| pattern == &ifd_buffer[0..10])
        {
            internal_offset = 10 + 8; // 10 nikon header bytes + 8 tiff header bytes
            nikon_mapping = true;
            // println!("Nikon mapping");
        } else {
            internal_offset = 0; // ifd starts right away
        }

        let num_entries = bytes_to_num(&ifd_buffer[internal_offset..internal_offset + 2]);
        internal_offset += 2;

        let ifd_entries = Vec::with_capacity(num_entries);
        let mut ifd = Ifd {
            ifd_type: IfdType::Other,
            ifd_buffer_start_point: offset,
            entries: ifd_entries,
        };

        if nikon_mapping {
            ifd_offset = Some(offset + 10);
            ifd.ifd_buffer_start_point = ifd_offset.unwrap();
            ifd.ifd_type = IfdType::NikonMakerNote;
        }

        for _ in 0..num_entries {
            let ifd_entry;
            let entry_data = &ifd_buffer[internal_offset..internal_offset + 12]
                .try_into()
                .unwrap();
            if !nikon_mapping {
                ifd_entry = IfdEntry::parse_entry(entry_data, ifd_offset);
            } else {
                ifd_entry = IfdEntry::parse_entry(entry_data, ifd_offset);
            }

            ifd.entries.push(ifd_entry);
            internal_offset += 12;
        }
        ifds.push(ifd.clone());
        // println!("Successfully fetched first IFD");

        if !nikon_mapping {
            Self::try_fetch_ifds(
                &ifd,
                TagParam::IfdEntry(IfdEntryTag::SubIFDS),
                &buffer,
                &mut ifds,
            );
            Self::try_fetch_ifds(
                &ifd,
                TagParam::IfdEntry(IfdEntryTag::ExifIFDPointer),
                &buffer,
                &mut ifds,
            );
            Self::try_fetch_ifds(
                &ifd,
                TagParam::IfdEntry(IfdEntryTag::MakerNote),
                &buffer,
                &mut ifds,
            );

            // fetch ifds linked at the end
            let offset_to_next_ifd = &ifd_buffer[internal_offset..internal_offset + 4];
            if offset_to_next_ifd == [0, 0, 0, 0] {
                // println!("No linked ifd at the end of the current ifd");
            } else {
                let offset_of_next_ifd = bytes_to_num(&offset_to_next_ifd);
                Self::try_fetch_ifds(
                    &ifd,
                    TagParam::U8(offset_of_next_ifd as u8),
                    buffer,
                    &mut ifds,
                );
            }
        }

        ifds
    }

    fn try_fetch_ifds(ifd: &Ifd, tag: TagParam, buffer: &[u8], ifds: &mut Vec<Ifd>) {
        let mut ifd_offsets: Vec<usize> = Vec::new();
        match tag {
            TagParam::IfdEntry(ifd_tag) => {
                if let Some(offset_to_ifd) = ifd.get_entry(TagParam::IfdEntry(ifd_tag)) {
                    println!("Parsing ifd from tag: {:?}", ifd_tag);
                    if offset_to_ifd.offset && ifd_tag != IfdEntryTag::MakerNote {
                        println!("IFD {:?} offset true", ifd_tag);
                        let bytes_per_comp = offset_to_ifd.data_type.bytes_per_component();
                        let offset_data = offset_to_ifd.get_offset_data(&buffer);
                        ifd_offsets = offset_data
                            .chunks(bytes_per_comp as usize)
                            .map(|chunk| bytes_to_num(chunk))
                            .collect();
                    } else {
                        ifd_offsets.push(bytes_to_num(&offset_to_ifd.data_or_offset));
                    }
                }
            }
            TagParam::U8(u8_val) => {
                ifd_offsets.push(u8_val as usize);
            }
        }
        for ifd_offset in ifd_offsets {
            let mut sub_ifds = Self::parse_ifd(&buffer, ifd_offset);
            ifds.append(&mut sub_ifds);
            println!("Successfully fetched {:?} IFD", tag);
        }
    }

    pub fn get_entry(&self, ifd_name: TagParam) -> Option<&IfdEntry> {
        match ifd_name {
            TagParam::IfdEntry(ifd_entry_name) => self
                .entries
                .iter()
                .find(|entry| entry.tag == ifd_entry_name),
            TagParam::U8(value) => self
                .entries
                .iter()
                .find(|entry| entry.tag.usize_value() == value.into()),
        }
    }

    pub fn print_info(&self) {
        println!("-------------------------");
        for ifd_entry in &self.entries {
            ifd_entry.print_info();
        }
        println!("-------------------------");
    }

    pub fn get_image_data(&self, ifd_buffer: &[u8]) -> Option<Vec<u16>> {
        let img_strip_offsets = self.get_entry(TagParam::IfdEntry(IfdEntryTag::StripOffsets));
        let img_rows_per_strip = self.get_entry(TagParam::IfdEntry(IfdEntryTag::RowsPerStrip));
        let img_strip_byte_count = self.get_entry(TagParam::IfdEntry(IfdEntryTag::StripByteCounts));
        let img_length = self.get_entry(TagParam::IfdEntry(IfdEntryTag::ImageLength));
        let img_sample_per_pixel = self.get_entry(TagParam::IfdEntry(IfdEntryTag::SamplesPerPixel));
        let img_width = self.get_entry(TagParam::IfdEntry(IfdEntryTag::ImageWidth));
        let img_height = self.get_entry(TagParam::IfdEntry(IfdEntryTag::ImageLength));

        if img_strip_offsets.is_none()
            || img_sample_per_pixel.is_none()
            || img_rows_per_strip.is_none()
            || img_strip_byte_count.is_none()
            || img_length.is_none()
            || img_width.is_none()
            || img_height.is_none()
        {
            return None;
        }

        let img_strip_offsets = bytes_to_num(&img_strip_offsets.unwrap().data_or_offset);
        let img_rows_per_strip = bytes_to_num(&img_rows_per_strip.unwrap().data_or_offset);
        let img_strip_byte_count = bytes_to_num(&img_strip_byte_count.unwrap().data_or_offset);
        let img_length = bytes_to_num(&img_length.unwrap().data_or_offset);
        let img_sample_per_pixel = bytes_to_num(&img_sample_per_pixel.unwrap().data_or_offset);
        let img_width = bytes_to_num(&img_width.unwrap().data_or_offset);
        let img_height = bytes_to_num(&img_height.unwrap().data_or_offset);

        // let img_strip_offsets = img_strip_offsets.get_offset_data(&ifd_buffer);
        let strips_per_image =
            f64::floor(((img_length + img_rows_per_strip - 1) / img_rows_per_strip) as f64)
                as usize;

        let strip_start = img_strip_offsets;
        let strip_end = img_strip_offsets + img_strip_byte_count;
        let strip_data = &ifd_buffer[strip_start..strip_end as usize];

        let image = decode_14le_unpacked(strip_data, img_width, img_height, false);
        // todo!("Decode image data");

        Some(image)
    }
}

impl IfdEntry {
    pub fn parse_entry(data: &[u8; 12], ifd_buffer_start_point: Option<usize>) -> Self {
        let raw_entry = *data;
        let tag = IfdEntryTag::from(bytes_to_num(&data[0..2]));
        let data_type = IfdEntryType::from(bytes_to_num(&data[2..4]));
        let num_values = bytes_to_num(&data[4..8]);
        let data_length = num_values * data_type.bytes_per_component() as usize;
        let offset = data_length > 4;
        let data_or_offset = [data[8], data[9], data[10], data[11]];
        let ifd_buffer_start_point = ifd_buffer_start_point.unwrap_or(0);
        Self {
            ifd_buffer_start_point,
            tag,
            data_type,
            data_or_offset,
            data_length,
            offset,
            raw_entry,
        }
    }
    pub fn get_human_readable_value(&self) -> String {
        if !self.offset {
            // Value is stored directly in the data_or_offset field
            match self.data_type {
                // IfdType::UnsignedByte(_) => {
                //     format_bytes_per_component_u8(&self.data_or_offset, num_components)
                // }
                IfdEntryType::AsciiString(_) => {
                    String::from_utf8_lossy(&self.data_or_offset).to_string()
                }
                _ => String::from("Not implemented yet")
                // IfdType::UnsignedShort(_) => {
                //     format_bytes_per_component_u16(&self.data_or_offset, num_components)
                // }
                // IfdType::UnsignedLong(_) => {
                //     format_bytes_per_component_u32(&self.data_or_offset, num_components)
                // }
                // IfdType::UnsignedRational(_) => {
                //     format_rational(&self.data_or_offset, num_components)
                // }
                // IfdType::SignedByte(_) => {
                //     format_bytes_per_component_i8(&self.data_or_offset, num_components)
                // }
                // IfdType::Undefined(_) => {
                //     format_bytes_per_component_u8(&self.data_or_offset, num_components)
                // }
                // IfdType::SignedShort(_) => {
                //     format_bytes_per_component_i16(&self.data_or_offset, num_components)
                // }
                // IfdType::SignedLong(_) => {
                //     format_bytes_per_component_i32(&self.data_or_offset, num_components)
                // }
                // IfdType::SignedRational(_) => format_rational(&self.data_or_offset, num_components),
                // IfdType::SingleFloat(_) => format_float(&self.data_or_offset, num_components, 4),
                // IfdType::DoubleFloat(_) => format_float(&self.data_or_offset, num_components, 8),
            }
        } else {
            // Value is stored at an offset in the file
            String::from_utf8_lossy(&self.data_or_offset).to_string()
        }
    }

    pub fn get_offset_data<'a>(&self, buffer: &'a [u8]) -> &'a [u8] {
        if self.offset || self.tag == IfdEntryTag::StripOffsets {
            let mut offset = bytes_to_num(&self.data_or_offset) as usize;
            offset = offset + self.ifd_buffer_start_point;
            &buffer[offset..offset + self.data_length]
        } else {
            &[]
        }
    }

    pub fn print_info(&self) {
        print!(
            "Tag ID: {:#02X}, {:?}. ",
            self.tag.usize_value(),
            self.tag.usize_value()
        );
        print!("Tag name: {:?}. ", self.tag);
        print!(
            "Data format: {:?}, {} byte(s). ",
            self.data_type,
            self.data_type.bytes_per_component()
        );
        print!("Data length: {:?}. ", self.data_length);
        print!(
            "Data value or offset to data value: {:#02X}, {:?}, {:?}. ",
            bytes_to_num(&self.data_or_offset),
            bytes_to_num(&self.data_or_offset),
            self.data_or_offset
        );
        println!("Offset: {}.", self.offset);
    }

    // fn get_buffer_start_point(&self) -> &usize {
    //     &self.ifd_buffer_start_point
    // }
}

pub fn decode_threaded<F>(width: usize, height: usize, dummy: bool, closure: &F) -> Vec<u16>
where
    F: Fn(&mut [u16], usize) + Sync,
{
    let mut out: Vec<u16> = alloc_image!(width, height, dummy);
    out.rchunks_mut(width).enumerate().for_each(|(row, line)| {
        closure(line, row);
    });
    out
}

pub fn decode_14le_unpacked(buf: &[u8], width: usize, height: usize, dummy: bool) -> Vec<u16> {
    decode_threaded(
        width,
        height,
        dummy,
        &(|out: &mut [u16], row| {
            let inb = &buf[(row * width * 2)..];

            for (i, bytes) in (0..width).zip(inb.chunks_exact(2)) {
                out[i] = LEu16(bytes, 0) & 0x3fff;
            }
        }),
    )
}

#[allow(non_snake_case)]
#[inline]
pub fn LEu16(buf: &[u8], pos: usize) -> u16 {
    LittleEndian::read_u16(&buf[pos..pos + 2])
}

fn decode_compressed(
    &self,
    src: &[u8],
    width: usize,
    height: usize,
    bps: usize,
    dummy: bool,
) -> Result<Vec<u16>, String> {
    // let metaifd = fetch_ifd!(self.tiff, Tag::NefMeta1);
    // let meta = if let Some(meta) = metaifd.find_entry(Tag::NefMeta2) {
    //     meta
    // } else {
    //     fetch_tag!(metaifd, Tag::NefMeta1)
    // };
    let offset_data_0x8c = [
        73, 48, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0,
        0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0,
        0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0,
        0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0,
        0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0,
        0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0,
        0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0,
        0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0,
        0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0,
        0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0,
        0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0,
        0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0,
        0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0,
        0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0,
        0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0,
        0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0,
        0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0,
        0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0,
        0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0,
        0, 0, 0, 0, 0, 0, 0, 0,
    ];
    let offset_data_0x96 = [
        70, 48, 0, 8, 0, 8, 0, 8, 0, 8, 34, 0, 183, 42, 170, 157, 224, 114, 94, 145, 24, 245, 28,
        153, 101, 130, 240, 175, 191, 32, 210, 210, 47, 198, 193, 2, 167, 134, 197, 90, 33, 88,
        216, 166, 202, 54,
    ];

    do_decode(
        src,
        offset_data_0x96,
        metaifd.get_endian(),
        width,
        height,
        bps,
        dummy,
    )
}

pub(crate) fn do_decode(
    src: &[u8],
    meta: &[u8],
    endian: Endian,
    width: usize,
    height: usize,
    bps: usize,
    dummy: bool,
) -> Result<Vec<u16>, String> {
    let mut out = alloc_image_ok!(width, height, dummy);
    let mut stream = ByteStream::new(meta, endian);
    let v0 = stream.get_u8();
    let v1 = stream.get_u8();
    //println!("Nef version v0:{}, v1:{}", v0, v1);

    let mut huff_select = 0;
    if v0 == 73 || v1 == 88 {
        stream.consume_bytes(2110);
    }
    if v0 == 70 {
        huff_select = 2;
    }
    if bps == 14 {
        huff_select += 3;
    }

    // Create the huffman table used to decode
    let mut htable = Self::create_hufftable(huff_select)?;

    // Setup the predictors
    let mut pred_up1: [i32; 2] = [stream.get_u16() as i32, stream.get_u16() as i32];
    let mut pred_up2: [i32; 2] = [stream.get_u16() as i32, stream.get_u16() as i32];

    // Get the linearization curve
    let mut points = [0 as u16; 1 << 16];
    for i in 0..points.len() {
        points[i] = i as u16;
    }
    let mut max = 1 << bps;
    let csize = stream.get_u16() as usize;
    let mut split = 0 as usize;
    let step = if csize > 1 { max / (csize - 1) } else { 0 };
    if v0 == 68 && v1 == 32 && step > 0 {
        for i in 0..csize {
            points[i * step] = stream.get_u16();
        }
        for i in 0..max {
            points[i] = ((points[i - i % step] as usize * (step - i % step)
                + points[i - i % step + step] as usize * (i % step))
                / step) as u16;
        }
        split = endian.ru16(meta, 562) as usize;
    } else if v0 != 70 && csize <= 0x4001 {
        for i in 0..csize {
            points[i] = stream.get_u16();
        }
        max = csize;
    }
    let curve = LookupTable::new(&points[0..max]);

    let mut pump = BitPumpMSB::new(src);
    let mut random = pump.peek_bits(24);

    let bps: u32 = bps as u32;
    for row in 0..height {
        if split > 0 && row == split {
            htable = Self::create_hufftable(huff_select + 1)?;
        }
        pred_up1[row & 1] += htable.huff_decode(&mut pump)?;
        pred_up2[row & 1] += htable.huff_decode(&mut pump)?;
        let mut pred_left1 = pred_up1[row & 1];
        let mut pred_left2 = pred_up2[row & 1];
        for col in (0..width).step_by(2) {
            if col > 0 {
                pred_left1 += htable.huff_decode(&mut pump)?;
                pred_left2 += htable.huff_decode(&mut pump)?;
            }
            out[row * width + col + 0] = curve.dither(clampbits(pred_left1, bps), &mut random);
            out[row * width + col + 1] = curve.dither(clampbits(pred_left2, bps), &mut random);
        }
    }

    Ok(out)
}
