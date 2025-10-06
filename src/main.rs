mod huffman;
mod huffmanv2;
mod ifd;
mod nef;
mod utils;

use crate::nef::NefFile;
use image::{ImageBuffer, Luma};
use std::{fs::File, io::BufWriter, path::Path};

fn main() {
    let file_path = Path::new("test_data/DSC_3935.NEF");

    if false {
        // Imagepipe
        if let Err(e) = image_pipe_covert(file_path) {
            eprintln!("Failed to convert image with imagepipe: {e}");
        }
    }

    match nef::NefFile::open(file_path) {
        Ok(nef_file) => {
            if let Err(e) = convert(nef_file) {
                eprintln!("Failed to convert image: {e}");
            }
        }
        Err(e) => eprintln!("Error: {:?}", e),
    }
}

fn image_pipe_covert(file_path: &Path) -> Result<(), anyhow::Error> {
    use image::ExtendedColorType::Rgb8;
    use imagepipe::Pipeline;

    let mut pipeline = Pipeline::new_from_file(file_path).map_err(anyhow::Error::msg)?;
    let decoded = pipeline.output_8bit(None).map_err(anyhow::Error::msg)?;
    let mut jpg_file = file_path.to_path_buf();
    jpg_file.set_extension("jpg");

    let outfile = File::create(jpg_file)?;
    let mut writer = BufWriter::new(outfile);
    let mut jpg_encoder = image::codecs::jpeg::JpegEncoder::new_with_quality(&mut writer, 100);
    jpg_encoder
        .encode(
            &decoded.data,
            decoded.width as u32,
            decoded.height as u32,
            Rgb8,
        )
        .map_err(anyhow::Error::msg)
}

/// Currently, only grayscale supported, but the raw image data is available.
fn convert(nef_file: NefFile) -> Result<(), anyhow::Error> {
    let out = nef_file.parse_raw_image_data()?;

    let width = nef_file.image_data.width as u32;
    let height = nef_file.image_data.height as u32;

    // Option A: Save grayscale JPEG directly from raw u16 values
    // Convert 16-bit raw values to 8-bit using simple normalization
    let gray8 = normalize_to_u8(&out);
    let gray_img: ImageBuffer<Luma<u8>, Vec<u8>> = ImageBuffer::from_raw(width, height, gray8)
        .ok_or(anyhow::Error::msg("Failed to create ImageBuffer"))?;

    let gray_output = File::create("output_gray.jpg").expect("Failed to create output file");
    let mut gray_writer = BufWriter::new(gray_output);
    image::codecs::jpeg::JpegEncoder::new(&mut gray_writer).encode_image(&gray_img)?;

    // let mut f = BufWriter::new(File::create("output.ppm").unwrap());
    // let preamble = format!("P6 {} {} {}\n", width, height, 65535).into_bytes();
    // f.write_all(&preamble).unwrap();
    // for pix in out {
    //     // Do an extremely crude "demosaic" by setting R=G=B
    //     let pixhigh = (pix >> 8).saturating_add(50) as u8;
    //     let pixlow = (pix & 0xff).saturating_add(50) as u8;
    //     f.write_all(&[pixhigh, pixlow, pixhigh, pixlow, pixhigh, pixlow])
    //         .unwrap();
    // }
    Ok(())
}

fn normalize_to_u8(raw: &[u16]) -> Vec<u8> {
    // Auto-normalize u16 values to 0..255 for a reasonable grayscale output
    if raw.is_empty() {
        return Vec::new();
    }
    let mut min_v = u16::MAX;
    let mut max_v = u16::MIN;
    for &v in raw {
        if v < min_v {
            min_v = v;
        }
        if v > max_v {
            max_v = v;
        }
    }
    let range = (max_v as i32 - min_v as i32).max(1) as f32;
    let min_f = min_v as f32;
    let mut out = Vec::with_capacity(raw.len());
    for &v in raw {
        let n = ((v as f32 - min_f) / range * 255.0).clamp(0.0, 255.0) as u8;
        out.push(n);
    }
    out
}
