use std::fs;
use std::io;

pub fn decomp(s: &str) -> Result<String, String>{
    let path = s;
    let file = fs::File::open(&path).map_err(|e| format!("Failed to open file: {}", e))?;
    let mut archive = zip::ZipArchive::new(file).map_err(|e| format!("Failed to read ZIP archive: {}", e))?;

    for i in 0..archive.len() {
        let mut file = archive.by_index(i).map_err(|e| format!("Failed to access file in ZIP: {}", e))?;

        let output = match file.enclosed_name() {
            Some(path) => path.to_owned(),
            None => continue,
        };
        if (*file.name()).ends_with('/'){
            println!("File {} extracted to \"{}\"", i, output.display());
            fs::create_dir_all(&output).map_err(|e| format!("Failed to create directory '{}': {}", output.display(), e))?;
        } else {
            println!("File {} extracted to  \"{}\" ({} bytes)", i, output.display(), file.size());

            if let Some(p) = output.parent(){
                if !p.exists(){
                    fs::create_dir_all(&p).map_err(|e| format!("Failed to create parent directory '{}': {}", p.display(), e))?;
                }
            }

            let mut outfile = fs::File::create(&output).map_err(|e| format!("Failed to create file '{}': {}", output.display(), e))?;
            io::copy(&mut file, &mut outfile).map_err(|e| format!("Failed to create file '{}': {}", output.display(), e))?;
        }        
    }
    
    Ok("File decompressed successfully".to_string())
}