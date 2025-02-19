extern crate flate2;
extern crate tar;

use flate2::write::GzEncoder;
use flate2::Compression;
use std::fs::{self, File};
use std::io::{self, copy, BufReader};
use std::path::Path;
use std::time::Instant;
use tar::Builder;

pub fn compress(s: &str) -> Result<String, String>{
    let path = Path::new(s);
    let output_file = format!("{}.tar.gz", s);

    let output = File:: create(&output_file).map_err(|_| "Failed to create output file")?;

    let mut encoder = GzEncoder::new(output, Compression::default());

    let start = Instant::now();

    if path.is_dir() {
        let mut tar_builder = Builder::new(&mut encoder);
        
        add_dir_to_archive(&mut tar_builder, path, Path::new("")).map_err(|_| "Failed to archive directory")?;
        tar_builder.finish().map_err(|_| "Failed to finalize archive")?;
    }
    else if path.is_file() {
        let mut input = BufReader::new(File::open(path).map_err(|_| "Failed to open file")?);
        copy(&mut input, &mut encoder).map_err(|_| "Failed to compress file")?;
    }
    else {
        return Err("Invalid input: Path is neither a file nor a directory".to_string());
    }

    let duration = start.elapsed();

    Ok(format!(
        "Successfully compressed to '{}' in {:.2?}",
        output_file, duration
    ))
}

fn add_dir_to_archive<W: io::Write>(tar_builder: &mut Builder<W>, dir: &Path, base_path: &Path) -> io::Result<()> {
    for entry in fs::read_dir(dir)? {
        let entry = entry?;
        let path = entry.path();
        let relative_path = base_path.join(path.file_name().unwrap());

        if path.is_dir() {
            add_dir_to_archive(tar_builder, &path, &relative_path)?;
        }
        else {
            let mut file = File::open(&path)?;
            tar_builder.append_file(relative_path, &mut file)?;
        }
    }
    Ok(())
}