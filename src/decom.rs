use std::fs;
use std::io;

pub fn decomp(s: &str) -> Result<String, String>{
    let path = s;
    let file = fs::File::open(&path).map_err(|_| "Failed to open file")?;
    let mut archive = zip::ZipArchive::new(file).map_err(|_| "Failed to read ZIP archive")?;

    for i in 0..archive.len() {
        let mut file = archive.by_index(i).map_err(|_| "Failed to access file in ZIP")?;

        let output = match file.enclosed_name() {
            Some(path) => path.to_owned(),
            None => continue,
        };
        if (*file.name()).ends_with('/'){
            println!("File {} extracted to \"{}\"", i, output.display());
            fs::create_dir_all(&output).map_err(|_| "Failed to create directory")?;
        } else {
            println!("File {} extracted to  \"{}\" ({} bytes)", i, output.display(), file.size());

            if let Some(p) = output.parent(){
                if !p.exists(){
                    fs::create_dir_all(&p).map_err(|_| "Failed to create parent directory")?;
                }
            }

            let mut outfile = fs::File::create(&output).map_err(|_| "Failed to create output file")?;
            io::copy(&mut file, &mut outfile).map_err(|_| "Failed to write output file")?;
        }        
    }
    
    Ok("File decompressed successfully".to_string())
}