

use rayon::prelude::*;
use std::fs;
use std::fs::File;
use std::io::{Read, Write};
use std::path::Path;
use oxipng::{optimize_from_memory, Options};
use std::time::Instant;

pub fn main() {
    let input_dir = "src/image";
    let output_dir = "src/compressed";

    fs::create_dir_all(output_dir).expect("Failed to create output directory");

    let files: Vec<_> = fs::read_dir(input_dir)
        .expect("Failed to read input directory")
        .filter_map(|entry| entry.ok())
        .filter(|entry| entry.path().extension().map_or(false, |ext| ext == "png"))
        .collect();

    let timer = Instant::now();

    files
    .par_iter()
    .for_each(|file| {
        let input_path = file.path();
        let output_path = Path::new(output_dir).join(file.file_name());

        println!("Compressing: {:?} -> {:?}", input_path, output_path);

        if let Ok(mut input_data) = File::open(&input_path).map(|mut file| {
            let mut data = Vec::new();
            file.read_to_end(&mut data).map(|_| data).unwrap_or_else(|e| {
                eprintln!("Failed to read {:?}: {:?}", input_path, e);
                Vec::new()
            })
        }) {
            let options = Options::from_preset(1); 
            match optimize_from_memory(&input_data, &options) {
                Ok(output_data) => {
                    if let Err(e) = File::create(&output_path)
                        .and_then(|mut file| file.write_all(&output_data))
                    {
                        eprintln!("Failed to write {:?}: {:?}", output_path, e);
                    }
                }
                Err(e) => eprintln!("Compression failed for {:?}: {:?}", input_path, e),
            }
        } else {
            eprintln!("Failed to open {:?}", input_path);
        }
    });
    let duration = timer.elapsed();
    println!("Time taken to compressed {:.2?} ", duration);
}
