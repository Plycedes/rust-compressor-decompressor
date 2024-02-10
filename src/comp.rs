extern crate flate2;

use flate2::write::GzEncoder;
use flate2::Compression;
use std::fs::File;
use std::io::copy;
use std::io::BufReader;
use std::time::Instant;

pub fn compress(){
    let path = "book.pdf";
    let output = "compressed".to_owned() + path;
    let mut input = BufReader::new(File::open(path).unwrap());
    let output = File::create(output).unwrap();
    let mut encoder = GzEncoder::new(output, Compression::default());
    let start = Instant::now();
    copy(&mut input, &mut encoder).unwrap();
    let output = encoder.finish().unwrap();
    println!(
        "Source len: {}",
        input.get_ref().metadata().unwrap().len()
    );
    println!(
        "Target len: {}", output.metadata().unwrap().len()
    );
    println!("Time Elasped {:?}", start.elapsed());
}