use rfd::FileDialog;

mod comp;
mod decom;
slint::include_modules!();

fn main() -> Result<(), slint::PlatformError> {
    let ui = AppWindow::new()?;

    let ui_handle = ui.as_weak();
    ui.on_compress(move |string| {
        let ui: AppWindow = ui_handle.unwrap();

        if let Err(e) = comp::compress(&string) {
            ui.set_result(format!("Compression failed: {}", e).into());
        } else {
            ui.set_result("File compressed successfully".into());
        }
    });

    let ui_handle = ui.as_weak();
    ui.on_decompress(move |string| {
        let ui: AppWindow = ui_handle.unwrap();

        match decom::decomp(&string) {
            Ok(msg) => ui.set_result(msg.into()),
            Err(err) => ui.set_result(format!("Decompression failed: {}", err).into()),
        }
    });

    let ui_handle = ui.as_weak();
    ui.on_browse_file(move || {       
        let dialog = FileDialog::new();
        
        if let Some(path) = dialog.add_filter("ZIP Files & Folders", &["zip"]).pick_file() {
            if let Some(ui) = ui_handle.upgrade() {
                ui.set_path(path.display().to_string().into());
            }
        }
    });

    let ui_handle = ui.as_weak();
    ui.on_browse_folder(move || {
        let dialog = FileDialog::new();

        if let Some(path) = dialog.pick_folder() {
            if let Some(ui) = ui_handle.upgrade() {
                ui.set_path(path.display().to_string().into());
            }            
        }
    });

    ui.run()
}

