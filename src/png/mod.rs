// In your png.rs (or any module you want to place this code in)

use image::io::Reader as ImageReader;
use image::{DynamicImage, ImageOutputFormat};
use rayon::prelude::*;
use std::fs::{File, create_dir_all};
use std::io::BufWriter;
use std::path::Path;
use oxipng::{optimize, InFile, Options, OutFile};
use std::error::Error;

/// Saves the PNG image to the given path using lossless encoding.
pub fn save_png_compressed(path: &str, img: &DynamicImage, _quality: u8) -> std::io::Result<()> {
    let file = File::create(path)?;
    let mut buf_writer = BufWriter::new(file);
    // For PNG, quality is not used because it's lossless.
    img.write_to(&mut buf_writer, ImageOutputFormat::Png)
        .map_err(|e| std::io::Error::new(std::io::ErrorKind::Other, e))
}

/// Compress a PNG image found at `input_path` by saving a compressed version
/// to `output_dir` and then running an optimization with oxipng.
pub fn compress_png(input_path: &str, output_dir: &str, quality: u8) -> Result<(), Box<dyn Error>> {
    // Ensure the output directory exists.
    create_dir_all(output_dir)?;

    let input_path_obj = Path::new(input_path);
    let file_name = input_path_obj.file_name()
        .ok_or("Invalid input file name")?
        .to_string_lossy();
    let compressed_path = format!("{}/{}", output_dir, file_name);

    // Open and decode the PNG image.
    let img = ImageReader::open(input_path)?
        .decode()
        .map_err(|e| std::io::Error::new(std::io::ErrorKind::Other, e))?;

    // Save the compressed image.
    save_png_compressed(&compressed_path, &img, quality)?;

    println!("PNG compression complete for: {}", input_path);

    // Optimize the newly saved PNG image using oxipng.
    let infile = InFile::Path(compressed_path.clone().into());
    let options = Options::default();
    let outfile = OutFile::Path(None);

    optimize(&infile, &outfile, &options)
        .map_err(|e| std::io::Error::new(std::io::ErrorKind::Other, e))?;

    println!("PNG optimization complete for: {}", input_path);

    Ok(())
}
