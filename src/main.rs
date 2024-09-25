#![cfg_attr(not(debug_assertions), windows_subsystem = "windows")] // hide console window on Windows in release
#![allow(rustdoc::missing_crate_level_docs)] // it's an example

use eframe::egui;
use rustick::MyApp;

fn main() -> eframe::Result<()> {
    let options = eframe::NativeOptions {
        viewport: egui::ViewportBuilder::default().with_inner_size([250.0, 120.0]),
        ..Default::default()
    };
    eframe::run_native(
        "Rustick",
        options,
        Box::new(|cc| Ok(Box::new(MyApp::new(cc))))
    )
}
