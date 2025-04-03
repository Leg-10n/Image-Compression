use std::fs::File;
use std::io::BufWriter;
use image::{io::Reader as ImageReader, DynamicImage, GenericImageView, ImageBuffer, ImageFormat, ImageOutputFormat, RgbImage};
use rayon::prelude::*;

pub fn compress_jpg(input_path: &str, output_path: &str, quality: u8, chunks: usize) -> Result<(), Box<dyn std::error::Error>> {
    let img = ImageReader::open(input_path)?.decode()?;
    let (width, height) = img.dimensions();
    let chunk_height = height / chunks as u32;

    let img_rgb = img.to_rgb8();
    let rows: Vec<_> = (0..chunks).collect();

    let chunked_images: Vec<ImageBuffer<image::Rgb<u8>, Vec<u8>>> = rows.par_iter().map(|&i| {
        let y_start = i as u32 * chunk_height;
        let y_end = if i == chunks - 1 { height } else { (i + 1) as u32 * chunk_height };

        let sub_img = img_rgb.view(0, y_start, width, y_end - y_start).to_image();
        sub_img
    }).collect();

    // Reconstruct the final image
    let mut final_img = RgbImage::new(width, height);
    for (i, chunk) in chunked_images.into_iter().enumerate() {
        let y_start = i as u32 * chunk_height;
        for y in 0..chunk.height() {
            for x in 0..chunk.width() {
                final_img.put_pixel(x, y_start + y, *chunk.get_pixel(x, y));
            }
        }
    }

    let file = File::create(output_path)?;
    let mut writer = BufWriter::new(file);
    final_img.write_to(&mut writer, ImageOutputFormat::Jpeg(quality))?;
    println!("Compressed {} to {}", input_path, output_path);

    Ok(())
}
