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

        decom::decomp(&string);
        ui.set_result("File compressed".into());
    });

    ui.run()
}