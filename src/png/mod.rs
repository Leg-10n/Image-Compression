use image::io::Reader as ImageReader;
use image::DynamicImage;
use rayon::prelude::*;
use std::fs::{File, create_dir_all};
use std::io::{Write, BufWriter};

const WINDOW_SIZE: usize = 4096; 

pub fn main() {
    let img = ImageReader::open("src/image/heart.png").unwrap().decode().unwrap();

    let raw_data = extract_raw_data(&img);

    let compressed_chunks: Vec<Vec<u8>> = raw_data
        .par_chunks(16 * 1024) // 16KB chunks
        .map(|chunk| lz77_compress(chunk))
        .collect();

    let compressed_data: Vec<u8> = compressed_chunks.concat();

    create_dir_all("src/compressed").unwrap();
    let file = File::create("src/compressed/compressed.lz77").unwrap();
    
    let mut buf_writer = BufWriter::new(file);
    buf_writer.write_all(&compressed_data).unwrap();

    println!("Compression complete! Saved as src/compressed/compressed.lz77");
}

fn extract_raw_data(img: &DynamicImage) -> Vec<u8> {
    img.to_bytes()
}

fn lz77_compress(data: &[u8]) -> Vec<u8> {
    let mut compressed = Vec::new();
    let mut i = 0;

    while i < data.len() {
        let mut match_length = 0;
        let mut match_distance = 0;

        let start = if i >= WINDOW_SIZE { i - WINDOW_SIZE } else { 0 };
        let window = &data[start..i];

        let best_match = (0..window.len())
            .into_par_iter()
            .rev() 
            .map(|j| {
                let mut length = 0;
                while length < 258 
                    && i + length < data.len()
                    && j + length < window.len()
                    && window[j + length] == data[i + length]
                {
                    length += 1;
                }
                (length, window.len() - j) 
            })
            .max_by_key(|&(length, _)| length);

        if let Some((length, distance)) = best_match {
            if length >= 3 {
                compressed.push(0); // Match flag
                compressed.push((distance >> 8) as u8);
                compressed.push((distance & 0xFF) as u8);
                compressed.push(length as u8);
                i += length; 
            } else {
                compressed.push(1); // Literal flag
                compressed.push(data[i]);
                i += 1; 
            }
        } else {
            compressed.push(1); 
            compressed.push(data[i]);
            i += 1;
        }
    }
    compressed
}
