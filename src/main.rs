mod comp;
mod decom;
slint::include_modules!();

fn main() -> Result<(), slint::PlatformError> {
    let ui = AppWindow::new()?;

    let ui_handle = ui.as_weak();
    ui.on_compress(move |string| {
        let ui: AppWindow = ui_handle.unwrap();

        comp::compress(&string);
        ui.set_result("File compressed".into());
    });

    let ui_handle = ui.as_weak();
    ui.on_decompress(move |string| {
        let ui: AppWindow = ui_handle.unwrap();

        match decom::decomp(&string) {
            Ok(msg) => ui.set_result(msg.into()),
            Err(err) => ui.set_result(format!("Decompression failed: {}", err).into()),
        }
    });

    ui.run()
}