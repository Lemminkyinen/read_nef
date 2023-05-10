use crate::utils::bytes_to_num;

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct Ifd {
    pub entries: Vec<IfdEntry>,
}

#[derive(Debug, Copy, Clone, PartialEq, Eq)]
pub struct IfdEntry {
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
enum TagParam {
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
    pub fn u16_value(&self) -> u16 {
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
            IfdEntryTag::Unknown(value) => *value as u16,
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
        let mut ifds = Vec::new();
        let mut internal_offset;
        let mut nikon_mapping = false;
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
            entries: ifd_entries,
        };

        for _ in 0..num_entries {
            let ifd_entry;
            let entry_data = &ifd_buffer[internal_offset..internal_offset + 12]
                .try_into()
                .unwrap();
            if !nikon_mapping {
                ifd_entry = IfdEntry::parse_entry(entry_data);
            } else {
                // ifd_entry = IfdEntryNikon::parse_entry(entry_data);
                ifd_entry = IfdEntry::parse_entry(entry_data);
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
                if let Some(offset_to_ifd) = ifd.get_entry(ifd_tag) {
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

    fn get_entry(&self, ifd_name: IfdEntryTag) -> Option<&IfdEntry> {
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
        let tag = IfdEntryTag::from(bytes_to_num(&data[0..2]));
        let data_type = IfdEntryType::from(bytes_to_num(&data[2..4]));
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
        if self.offset {
            let offset = bytes_to_num(&self.data_or_offset) as usize;
            &buffer[offset..offset + self.data_length]
        } else {
            &[]
        }
    }
}
