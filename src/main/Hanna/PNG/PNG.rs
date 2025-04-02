/*
1.Load the PNG using image crate.

2.Extract raw pixel data.

3.Split into chunks and compress them in parallel using rayon.

4.Use lz77 crate for actual compression.

5.Save the compressed output to a file. 
 */

 use image::io::Reader as ImageReader;
 use image::DynamicImage;
 use lz77::compressor::Lz77Compressor;
 use rayon::prelude::*;
 use std::fs::File;
 use std::io::Write;
 
 fn main() {

     let img = ImageReader::open("input.png").unwrap().decode().unwrap();
     let raw_data = extract_raw_data(&img); // Convert image to raw bytes

     let compressed_chunks: Vec<Vec<u8>> = raw_data
         .par_chunks(4096) // Split into 4KB chunks
         .map(|chunk| lz77_compress(chunk))
         .collect();
 
     let compressed_data: Vec<u8> = compressed_chunks.concat();

     let mut file = File::create("compressed.lz77").unwrap();
     file.write_all(&compressed_data).unwrap();
 
     println!("Compression complete! Saved as compressed.lz77");
 }
 

 fn extract_raw_data(img: &DynamicImage) -> Vec<u8> {
     img.to_bytes() // Converts image into raw bytes
 }
 

 fn lz77_compress(data: &[u8]) -> Vec<u8> {
     let mut compressor = Lz77Compressor::new();
     compressor.compress(data).unwrap()
 }
 