use image::io::Reader as ImageReader;
use image::DynamicImage;
use rayon::prelude::*;
use std::fs::{File, create_dir_all, read_dir};
use std::io::BufWriter;
use oxipng::{optimize, InFile, Options, OutFile};

pub fn main() {
    let image_dir = "src/image";
    let output_dir = "src/compressed";
    create_dir_all(output_dir).expect("Failed to create output directory");

    let images = read_dir(image_dir).expect("Failed to read image directory")
        .filter_map(Result::ok)
        .filter(|entry| entry.path().extension().map(|ext| ext == "png").unwrap_or(false))
        .collect::<Vec<_>>();

    images.par_iter().for_each(|entry| {
        let img = match ImageReader::open(entry.path()) {
            Ok(reader) => match reader.decode() {
                Ok(img) => img,
                Err(e) => {
                    eprintln!("Failed to decode image {}: {}", entry.path().display(), e);
                    return;
                }
            },
            Err(e) => {
                eprintln!("Failed to open image {}: {}", entry.path().display(), e);
                return;
            }
        };

        let compressed_path = format!("{}/{}", output_dir, entry.file_name().to_string_lossy());
        if let Err(e) = save_png_compressed(&compressed_path, &img, 9) {
            eprintln!("Failed to save PNG {}: {}", compressed_path, e);
        } else {
            println!("Compression complete for: {}", entry.path().display());

            let infile = InFile::Path(compressed_path.clone().into()); 
            let options = Options::default(); 
            let outfile = OutFile::Path(None);

            if let Err(e) = optimize(&infile, &outfile, &options) {
                eprintln!("Failed to optimize PNG {}: {}", compressed_path, e);
            }
        }
    });

    println!("All images compressed!");
}

fn save_png_compressed(path: &str, img: &DynamicImage, quality: u8) -> std::io::Result<()> {
    let file = File::create(path)?;
    let buf_writer = BufWriter::new(file);
    img.write_to(&mut buf_writer.into_inner()?, image::ImageOutputFormat::Png).map_err(|e| std::io::Error::new(std::io::ErrorKind::Other, e))?;
    Ok(())
}
