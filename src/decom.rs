use std::fs;
use std::io;

pub fn decomp(s: &str){
    let path = s;
    let file = fs::File::open(&path).unwrap();
    let mut archive = zip::ZipArchive::new(file).unwrap();

    for i in 0..archive.len() {
        let mut file = archive.by_index(i).unwrap();

        let output = match file.enclosed_name() {
            Some(path) => path.to_owned(),
            None => continue,
        };
        if (*file.name()).ends_with('/'){
            println!("File {} extracted to \"{}\"", i, output.display());
            fs::create_dir_all(&output).unwrap();
        } else {
            println!("File {} extracted to  \"{}\" ({} bytes)", i, output.display(), file.size());

            if let Some(p) = output.parent(){
                if !p.exists(){
                    fs::create_dir_all(&p).unwrap();
                }
            }

            let mut outfile = fs::File::create(&output).unwrap();
            io::copy(&mut file, &mut outfile).unwrap();
        }

        #[cfg(unix)]
        {
            use std::os::unix::fs::PermissionExt;

            if let Some(mode) = file.unix_mode(){
                fs::set_permissions(&output, fs::Permissions::from_mode(mode)).unwrap();
            }
        }
    }
    
}