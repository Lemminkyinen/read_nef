use std::{
    fs::File,
    io::{BufWriter, Write},
    path::Path,
};
mod huffman;
mod huffmanv2;
mod ifd;
mod nef;
mod utils;
use image::{ExtendedColorType::Rgb8, ImageBuffer};
use imagepipe::Pipeline;

fn main() {
    let file_path = Path::new("DSC_3935.NEF");
    let mut asdcasd =
        Pipeline::new_from_file(file_path).expect("Cannot create pipeline from filepath");
    let decoded = asdcasd.output_8bit(None).expect("Cannot decode image");

    let uf = File::create("output2.jpg").expect("Cannot create outfile");
    let mut f = BufWriter::new(uf);
    let mut jpg_encoder = image::codecs::jpeg::JpegEncoder::new_with_quality(&mut f, 100);
    jpg_encoder
        .encode(
            &decoded.data,
            decoded.width as u32,
            decoded.height as u32,
            Rgb8,
        )
        .expect("asdcasd");

    match nef::NefFile::open(file_path) {
        Ok(nef_file) => {
            /*
            let out = nef_file
                .parse_raw_image_data()
                .expect("Error parsing raw image data");

            let width = nef_file.image_data.width as u32;
            let height = nef_file.image_data.height as u32;
            */

            /*
            let raw_rgb_data = demosaic(&out, width, height);

            let img: ImageBuffer<Rgb<u8>, Vec<u8>> =
                ImageBuffer::from_raw(width, height, raw_rgb_data)
                    .expect("Failed to create imager bugger");

            // Save the image as a JPEG file
            let output_file = File::create("output.jpg").expect("Failed to create output file");
            let mut writer = BufWriter::new(output_file);

            image::codecs::jpeg::JpegEncoder::new(&mut writer)
                .encode_image(&img)
                .expect("Failed to encode image as JPEG");

            */

            /*
            let mut f = BufWriter::new(File::create("output.ppm").unwrap());
            let preamble = format!("P6 {} {} {}\n", width, height, 65535).into_bytes();
            f.write_all(&preamble).unwrap();

            for pix in out {
                // Do an extremely crude "demosaic" by setting R=G=B
                let pixhigh = (pix >> 8).saturating_add(50) as u8;
                let pixlow = (pix & 0xff).saturating_add(50) as u8;
                f.write_all(&[pixhigh, pixlow, pixhigh, pixlow, pixhigh, pixlow])
                    .unwrap();
            }
            */
        }
        Err(e) => eprintln!("Error: {:?}", e),
    }
}

fn demosaic(raw_data: &[u16], width: u32, height: u32) -> Vec<u8> {
    let mut rgb_data = vec![0u8; (width * height * 3) as usize];

    for y in 0..height {
        for x in 0..width {
            let i = (y * width + x) as usize;
            let pixel_value = raw_data[i];

            // Determine the position in the Bayer filter pattern
            let is_red = (y % 2 == 0) && (x % 2 == 0);
            let is_green1 = (y % 2 == 0) && (x % 2 != 0);
            let is_green2 = (y % 2 != 0) && (x % 2 == 0);
            let is_blue = (y % 2 != 0) && (x % 2 != 0);

            let (r, g, b) = if is_red {
                let g = if x > 0 && x < width - 1 && y < height - 1 {
                    (raw_data[i + 1] as u32 + raw_data[i + width as usize] as u32) / 2
                } else {
                    pixel_value as u32
                };
                let b = if x < width - 1 && y < height - 1 {
                    raw_data[i + width as usize + 1] as u32
                } else {
                    pixel_value as u32
                };
                (pixel_value as u32, g, b)
            } else if is_green1 || is_green2 {
                let r = if x > 0 {
                    raw_data[i - 1] as u32
                } else {
                    pixel_value as u32
                };
                let b = if y < height - 1 {
                    raw_data[i + width as usize] as u32
                } else {
                    pixel_value as u32
                };
                (r, pixel_value as u32, b)
            } else if is_blue {
                let r = if x > 0 && y > 0 {
                    raw_data[i - width as usize - 1] as u32
                } else {
                    pixel_value as u32
                };
                let g = if x > 0 && y > 0 {
                    (raw_data[i - 1] as u32 + raw_data[i - width as usize] as u32) / 2
                } else {
                    pixel_value as u32
                };
                (r, g, pixel_value as u32)
            } else {
                (pixel_value as u32, pixel_value as u32, pixel_value as u32)
            };

            let pixel_index = (y * width + x) as usize * 3;
            rgb_data[pixel_index] = r.min(255) as u8;
            rgb_data[pixel_index + 1] = g.min(255) as u8;
            rgb_data[pixel_index + 2] = b.min(255) as u8;
        }
    }

    rgb_data
}
