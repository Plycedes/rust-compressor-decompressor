extern crate zip;

use std::fs::{self, File};
use std::io::{self, Write, Seek, Read};
use std::path::Path;
use zip::write::FileOptions;
use zip::ZipWriter;
use std::time::Instant;

pub fn compress(s: &str) -> Result<String, String>{
    let path = Path::new(s);
    let output_file = format!("{}.zip", s);

    let zip_file = File::create(&output_file).map_err(|_| "Failed to create ZIP file")?;
    let mut zip = ZipWriter::new(zip_file);

    let start = Instant::now();

    if path.is_dir() {        
        zip_directory(&mut zip, path, path, FileOptions::default()).map_err(|_| "Failed to archive directory")?;
    }
    else if path.is_file() {
        zip_file_entry(&mut zip, path, Path::new(path.file_name().unwrap())).map_err(|_| "Failed to compress file")?;
    }
    else {
        return Err("Invalid input: Path is neither a file nor a directory".to_string());
    }

    zip.finish().map_err(|_| "Failed to finalize ZIP archive")?;

    let duration = start.elapsed();

    Ok(format!(
        "Successfully compressed to '{}' in {:.2?}",
        output_file, duration
    ))
}

fn zip_directory<W: Write + Seek>(zip: &mut ZipWriter<W>, dir: &Path, base_path: &Path, options: FileOptions) -> io::Result<()> {
    for entry in fs::read_dir(dir)? {
        let entry = entry?;
        let path = entry.path();
        let relative_path = path.strip_prefix(base_path).unwrap();

        if path.is_dir() {
            zip.add_directory(relative_path.to_str().unwrap(), options)?;
            zip_directory(zip, &path, base_path, options)?;
        }
        else {
            zip_file_entry(zip, &path, relative_path)?;
        }
    }
    Ok(())
}

fn zip_file_entry<W: Write + Seek>(zip: &mut ZipWriter<W>, file_path: &Path, zip_path: &Path) -> io::Result<()> {
    let mut file = File::open(file_path)?;
    let mut buffer = Vec::new();
    file.read_to_end(&mut buffer)?;

    zip.start_file(zip_path.to_str().unwrap(), FileOptions::default())?;
    zip.write_all(&buffer)?;
    Ok(())
}