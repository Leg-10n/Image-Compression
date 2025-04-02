use image::io::Reader as ImageReader;
use image::DynamicImage;
use rayon::prelude::*;
use std::fs::{File, create_dir_all, rename};
use std::io::Write;

fn main() {
    let img = ImageReader::open("src/image/heart.png").unwrap().decode().unwrap();

    let raw_data = extract_raw_data(&img);

    let compressed_chunks: Vec<Vec<u8>> = raw_data
        .par_chunks(4096) // Split into 4KB chunks.
        .map(|chunk| lz77_compress(chunk)) 
        .collect();

    let compressed_data: Vec<u8> = compressed_chunks.concat();

    // Ensure the directory exists
    create_dir_all("compressed").unwrap();

    // Save the compressed file
    let mut file = File::create("src/compressed.lz77").unwrap();
    file.write_all(&compressed_data).unwrap();

    // Move the file to the new directory
    rename("src/compressed.lz77", "compressed/compressed.lz77").unwrap();

    println!("Compression complete! Saved as compressed/compressed.lz77");
}

fn extract_raw_data(img: &DynamicImage) -> Vec<u8> {
    img.to_bytes()
}

fn lz77_compress(data: &[u8]) -> Vec<u8> {
    // Placeholder for LZ77 compression algorithm
    data.to_vec()
}
