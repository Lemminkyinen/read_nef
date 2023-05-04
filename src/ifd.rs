use crate::utils::bytes_to_num;

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct Ifd {
    pub entries: Vec<IfdEntry>,
}

#[derive(Debug, Copy, Clone, PartialEq, Eq)]
pub struct IfdEntry {
    pub tag: IfdTag,
    pub data_type: IfdType,
    pub data_or_offset: [u8; 4],
    pub data_length: usize,
    pub offset: bool,
    pub raw_entry: [u8; 12],
}

#[derive(Debug, Copy, Clone, PartialEq, Eq)]
pub enum IfdTag {
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
pub enum IfdType {
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
}

impl From<usize> for IfdTag {
    fn from(tag_value: usize) -> Self {
        match tag_value {
            0xFE => IfdTag::NewSubfileType,
            0x100 => IfdTag::ImageWidth,
            0x101 => IfdTag::ImageLength,
            0x102 => IfdTag::BitsPerSample,
            0x103 => IfdTag::Compression,
            0x106 => IfdTag::PhotometricInterpretation,
            0x10F => IfdTag::Make,
            0x110 => IfdTag::Model,
            0x111 => IfdTag::StripOffsets,
            0x112 => IfdTag::Orientation,
            0x115 => IfdTag::SamplesPerPixel,
            0x116 => IfdTag::RowsPerStrip,
            0x117 => IfdTag::StripByteCounts,
            0x11A => IfdTag::XResolution,
            0x11B => IfdTag::YResolution,
            0x11C => IfdTag::PlanarConfiguration,
            0x128 => IfdTag::ResolutionUnit,
            0x131 => IfdTag::Software,
            0x132 => IfdTag::DateTime,
            0x13B => IfdTag::Artist,
            0x14A => IfdTag::SubIFDS,
            0x201 => IfdTag::JpgFromRawStart,
            0x202 => IfdTag::JpgFromRawLength,
            0x213 => IfdTag::YCbCrPositioning,
            0x214 => IfdTag::ReferenceBlackNWhite,
            0x2BC => IfdTag::XMLMetaData,
            0x828D => IfdTag::CFARepeatPatternDim,
            0x828E => IfdTag::CFAPattern,
            0x8298 => IfdTag::CopyRight,
            0x829A => IfdTag::ExposureTime,
            0x829D => IfdTag::FNumber,
            0x8769 => IfdTag::ExifIFDPointer,
            0x8822 => IfdTag::ExposureProgram,
            0x8825 => IfdTag::GPSInfo,
            0x8827 => IfdTag::ISOSpeedRatings,
            0x8830 => IfdTag::SensitivityType,
            0x8832 => IfdTag::RecommendedExposureIndex,
            0x9003 => IfdTag::DateTimeOriginal,
            0x9004 => IfdTag::DateTimeDigitized,
            0x9010 => IfdTag::NikonPictureControlVersion,
            0x9011 => IfdTag::NikonPictureControlName,
            0x9012 => IfdTag::NikonPictureControlComment,
            0x9204 => IfdTag::ExposureBias,
            0x9205 => IfdTag::MaxAperture,
            0x9207 => IfdTag::ExposureMeteringMode,
            0x9208 => IfdTag::LightSource,
            0x9209 => IfdTag::Flash,
            0x920A => IfdTag::FocalLength,
            0x9216 => IfdTag::TIFFEPStandardID,
            0x9217 => IfdTag::SensingMethod,
            0x927C => IfdTag::MakerNote,
            0x9286 => IfdTag::UserComment,
            0x9290 => IfdTag::SubSecTime,
            0x9291 => IfdTag::SubSecTimeOriginal,
            0x9292 => IfdTag::SubSecTimeDigitized,
            0xA217 => IfdTag::NikonAFInfo2,
            0xA300 => IfdTag::FileSource,
            0xA301 => IfdTag::NikonCaptureVersion,
            0xA302 => IfdTag::NikonCaptureOffset,
            0xA401 => IfdTag::NikonScanIFD,
            0xA402 => IfdTag::NikonCaptureEditVersion,
            0xA403 => IfdTag::NikonCaptureEditCount,
            0xA405 => IfdTag::NikonCaptureEditApplied,
            0xA406 => IfdTag::NikonCaptureToneCurve,
            0xA407 => IfdTag::NikonCaptureSharpener,
            0xA408 => IfdTag::NikonCaptureColorMode,
            0xA409 => IfdTag::NikonCaptureColorHue,
            0xA40A => IfdTag::NikonCaptureSaturation,
            0xA40C => IfdTag::NikonCaptureNoiseReduction,
            // Add other tag value mappings as needed.
            _ => IfdTag::Unknown(tag_value),
        }
    }
}

impl IfdTag {
    pub fn u16_value(&self) -> u16 {
        match self {
            IfdTag::NewSubfileType => 0xFE,
            IfdTag::ImageWidth => 0x100,
            IfdTag::ImageLength => 0x101,
            IfdTag::BitsPerSample => 0x102,
            IfdTag::Compression => 0x103,
            IfdTag::PhotometricInterpretation => 0x106,
            IfdTag::Make => 0x10F,
            IfdTag::Model => 0x110,
            IfdTag::StripOffsets => 0x111,
            IfdTag::Orientation => 0x112,
            IfdTag::SamplesPerPixel => 0x115,
            IfdTag::RowsPerStrip => 0x116,
            IfdTag::StripByteCounts => 0x117,
            IfdTag::XResolution => 0x11A,
            IfdTag::YResolution => 0x11B,
            IfdTag::PlanarConfiguration => 0x11C,
            IfdTag::ResolutionUnit => 0x128,
            IfdTag::Software => 0x131,
            IfdTag::DateTime => 0x132,
            IfdTag::Artist => 0x13B,
            IfdTag::SubIFDS => 0x14A,
            IfdTag::JpgFromRawStart => 0x201,
            IfdTag::JpgFromRawLength => 0x202,
            IfdTag::YCbCrPositioning => 0x213,
            IfdTag::ReferenceBlackNWhite => 0x214,
            IfdTag::XMLMetaData => 0x2BC,
            IfdTag::CFARepeatPatternDim => 0x828D,
            IfdTag::CFAPattern => 0x828E,
            IfdTag::CopyRight => 0x8298,
            IfdTag::ExposureTime => 0x829A,
            IfdTag::FNumber => 0x829D,
            IfdTag::ExifIFDPointer => 0x8769,
            IfdTag::ExposureProgram => 0x8822,
            IfdTag::GPSInfo => 0x8825,
            IfdTag::ISOSpeedRatings => 0x8827,
            IfdTag::SensitivityType => 0x8830,
            IfdTag::RecommendedExposureIndex => 0x8832,
            IfdTag::DateTimeOriginal => 0x9003,
            IfdTag::DateTimeDigitized => 0x9004,
            IfdTag::NikonPictureControlVersion => 0x9010,
            IfdTag::NikonPictureControlName => 0x9011,
            IfdTag::NikonPictureControlComment => 0x9012,
            IfdTag::ExposureBias => 0x9204,
            IfdTag::MaxAperture => 0x9205,
            IfdTag::ExposureMeteringMode => 0x9207,
            IfdTag::LightSource => 0x9208,
            IfdTag::Flash => 0x9209,
            IfdTag::FocalLength => 0x920A,
            IfdTag::TIFFEPStandardID => 0x9216,
            IfdTag::SensingMethod => 0x9217,
            IfdTag::MakerNote => 0x927C,
            IfdTag::UserComment => 0x9286,
            IfdTag::SubSecTime => 0x9290,
            IfdTag::SubSecTimeOriginal => 0x9291,
            IfdTag::SubSecTimeDigitized => 0x9292,
            IfdTag::NikonAFInfo2 => 0xA217,
            IfdTag::FileSource => 0xA300,
            IfdTag::NikonCaptureVersion => 0xA301,
            IfdTag::NikonCaptureOffset => 0xA302,
            IfdTag::NikonScanIFD => 0xA401,
            IfdTag::NikonCaptureEditVersion => 0xA402,
            IfdTag::NikonCaptureEditCount => 0xA403,
            IfdTag::NikonCaptureEditApplied => 0xA405,
            IfdTag::NikonCaptureToneCurve => 0xA406,
            IfdTag::NikonCaptureSharpener => 0xA407,
            IfdTag::NikonCaptureColorMode => 0xA408,
            IfdTag::NikonCaptureColorHue => 0xA409,
            IfdTag::NikonCaptureSaturation => 0xA40A,
            IfdTag::NikonCaptureNoiseReduction => 0xA40C,
            IfdTag::Unknown(value) => *value as u16,
        }
    }
}

impl From<usize> for IfdType {
    fn from(data_type: usize) -> Self {
        match data_type {
            1 => IfdType::UnsignedByte(1),
            2 => IfdType::AsciiString(1),
            3 => IfdType::UnsignedShort(2),
            4 => IfdType::UnsignedLong(4),
            5 => IfdType::UnsignedRational(8),
            6 => IfdType::SignedByte(1),
            7 => IfdType::Undefined(1),
            8 => IfdType::SignedShort(2),
            9 => IfdType::SignedLong(4),
            10 => IfdType::SignedRational(8),
            11 => IfdType::SingleFloat(4),
            12 => IfdType::DoubleFloat(8),
            _ => panic!("Invalid data type: {}", data_type),
        }
    }
}

impl IfdType {
    pub fn bytes_per_component(&self) -> u8 {
        match self {
            IfdType::UnsignedByte(bytes)
            | IfdType::AsciiString(bytes)
            | IfdType::SignedByte(bytes)
            | IfdType::Undefined(bytes) => *bytes,
            IfdType::UnsignedShort(bytes) | IfdType::SignedShort(bytes) => *bytes,
            IfdType::UnsignedLong(bytes)
            | IfdType::SignedLong(bytes)
            | IfdType::SingleFloat(bytes) => *bytes,
            IfdType::UnsignedRational(bytes)
            | IfdType::SignedRational(bytes)
            | IfdType::DoubleFloat(bytes) => *bytes,
        }
    }
}

impl Ifd {
    pub fn parse_ifd(buffer: &[u8]) -> Vec<Self> {
        let mut ifds = Vec::new();
        let mut internal_offset;
        let mut nikon_mapping = false;

        // println!("{:?}", &buffer[0..8]);
        if buffer[0..8] == [73, 73, 42, 0, 8, 0, 0, 0] {
            internal_offset = 8;
        } else if buffer[0..10] == [0x4E, 0x69, 0x6B, 0x6F, 0x6E, 0x00, 0x02, 0x00, 0x00, 0x00]
            || buffer[0..10] == [0x4E, 0x69, 0x6B, 0x6F, 0x6E, 0x00, 0x02, 0x10, 0x00, 0x00]
        {
            internal_offset = 10 + 8; // 10 nikon header bytes + 8 tiff header bytes
            nikon_mapping = true;
        } else {
            internal_offset = 0; // ifd starts right away
        }

        let num_entries = bytes_to_num(&buffer[internal_offset..internal_offset + 2]);
        internal_offset += 2;

        let ifd_entries = Vec::with_capacity(num_entries as usize);
        let mut ifd = Ifd {
            entries: ifd_entries,
        };

        for _ in 0..num_entries {
            let entry_data = &buffer[internal_offset..internal_offset + 12]
                .try_into()
                .unwrap();
            let ifd_entry = IfdEntry::parse_entry(entry_data);
            ifd.entries.push(ifd_entry);
            internal_offset += 12;
        }
        ifds.push(ifd.clone());
        // println!("Successfully fetched first IFD");

        // fetch other linked ifds from tag 0x014A (ChildIfdsOffset)
        if let Some(offset_to_childs) = ifd.get_entry(IfdTag::SubIFDS) {
            let bytes_per_comp = offset_to_childs.data_type.bytes_per_component();
            let offset_data = offset_to_childs.get_offset_data(&buffer);
            let sub_ifd_offsets = offset_data.chunks(bytes_per_comp as usize);
            for sub_ifd_offset in sub_ifd_offsets {
                let sub_ifd_offset_u8 = bytes_to_num(sub_ifd_offset);
                let mut sub_ifds = Self::parse_ifd(&buffer[sub_ifd_offset_u8..]);
                ifds.append(&mut sub_ifds);
                println!("Successfully fetched child IFD")
            }
        }

        // and 0x8769 (ExifOffset)
        if let Some(offset_to_exif) = ifd.get_entry(IfdTag::ExifIFDPointer) {
            let exif_ifd_offset_u8 = bytes_to_num(&offset_to_exif.data_or_offset);
            let mut exif_ifd = Self::parse_ifd(&buffer[exif_ifd_offset_u8..]);
            ifds.append(&mut exif_ifd);
            println!("Successfully fetched exif IFD")
        }

        // fetch ifds linked at the end
        let offset_to_next_ifd = &buffer[internal_offset..internal_offset + 4];
        if offset_to_next_ifd == [0, 0, 0, 0] {
            // println!("No linked ifd at the end of the current ifd");
        } else {
            println!("Parsing linked ifd from offest: {:?}", offset_to_next_ifd);
            let offset_of_next_ifd = bytes_to_num(&offset_to_next_ifd);
            let mut linked_ifds = Self::parse_ifd(&buffer[offset_of_next_ifd..]);
            ifds.append(&mut linked_ifds);
        }

        ifds
    }

    pub fn get_entry(&self, ifd_name: IfdTag) -> Option<&IfdEntry> {
        self.entries.iter().find(|entry| entry.tag == ifd_name)
    }

    pub fn print_info(&self) {
        fn print_ifd_entry(entry: &IfdEntry) {
            print!(
                "Tag ID: {:#02X}, {:?}. ",
                entry.tag.u16_value(),
                entry.tag.u16_value()
            );
            print!("Tag name: {:?}. ", entry.tag);
            print!(
                "Data format: {:?}, {} byte(s). ",
                entry.data_type,
                entry.data_type.bytes_per_component()
            );
            print!("Data length: {:?}. ", entry.data_length);
            print!(
                "Data value or offset to data value: {:#02X}, {:?}, {:?}. ",
                bytes_to_num(&entry.data_or_offset),
                bytes_to_num(&entry.data_or_offset),
                entry.data_or_offset
            );
            println!("Offset: {}.", entry.offset);
        }
        println!("-------------------------");
        // println!("Start of IFD: {:?}", &self.start);
        // println!("End of IFD: {:?}", &self.end);
        for ifd_entry in &self.entries {
            print_ifd_entry(ifd_entry);
        }
        // if self.offset_to_next_ifd == 0 {
        //     println!("No linked IFD! Value {:?}", &self.offset_to_next_ifd)
        // } else {
        //     println!("Offset to next IFD: {:?}", &self.offset_to_next_ifd)
        // }
        println!("-------------------------");
    }
}

impl IfdEntry {
    pub fn parse_entry(data: &[u8; 12]) -> Self {
        let raw_entry = *data;
        let tag = IfdTag::from(bytes_to_num(&data[0..2]));
        let data_type = IfdType::from(bytes_to_num(&data[2..4]));
        let num_values = bytes_to_num(&data[4..8]);
        let data_length = num_values * data_type.bytes_per_component() as usize;
        let offset = data_length > 4;
        let data_or_offset = [data[8], data[9], data[10], data[11]];
        Self {
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
                IfdType::AsciiString(_) => {
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
        if self.offset {
            let offset = bytes_to_num(&self.data_or_offset) as usize;
            &buffer[offset..offset + self.data_length]
        } else {
            &[]
        }
    }
}
