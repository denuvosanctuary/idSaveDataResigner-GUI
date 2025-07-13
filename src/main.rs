#![windows_subsystem = "windows"]
use eframe::egui;
mod logic;
mod app;
use app::SaveDataApp;

fn main() -> Result<(), eframe::Error> {
    let options = eframe::NativeOptions {
        viewport: egui::ViewportBuilder::default()
            .with_inner_size([800.0, 600.0])
            .with_min_inner_size([600.0, 400.0]),
        ..Default::default()
    };
   
    eframe::run_native(
        "idSaveData Resigner",
        options,
        Box::new(|_cc| Ok(Box::new(SaveDataApp::new()))),
    )
}