#![cfg_attr(not(debug_assertions), windows_subsystem = "windows")] // hide console window on Windows in release
#![allow(rustdoc::missing_crate_level_docs)] // it's an example

use eframe::egui;
use device_query::{ DeviceQuery, DeviceState, Keycode };
use std::time::Instant;

fn main() -> eframe::Result {
    let options = eframe::NativeOptions {
        viewport: egui::ViewportBuilder::default().with_inner_size([320.0, 160.0]),
        ..Default::default()
    };
    eframe::run_native(
        "Rustick",
        options,
        Box::new(|cc| Ok(Box::new(MyApp::new(cc))))
    )
}

struct MyApp {
    status: String,
    start_time: Instant,
    down_count: u32,
    device_state: DeviceState,
    time: u128,
}

impl Default for MyApp {
    fn default() -> Self {
        Self {
            status: "init".to_string(),
            start_time: Instant::now(),
            down_count: 0,
            device_state: DeviceState::new(),
            time: 0,
        }
    }
}

impl MyApp {
    fn new(cc: &eframe::CreationContext<'_>) -> Self {
        load_fonts(&cc.egui_ctx);
        Self::default()
    }
}

fn load_fonts(ctx: &egui::Context) {
    let mut fonts = egui::FontDefinitions::default();
    fonts.font_data.insert(
        "my_font".to_owned(),
        egui::FontData::from_static(include_bytes!("C:\\Windows\\Fonts\\simsun.ttc"))
    );
    fonts.families
        .get_mut(&egui::FontFamily::Proportional)
        .unwrap()
        .insert(0, "my_font".to_owned());
    fonts.families.get_mut(&egui::FontFamily::Monospace).unwrap().push("my_font".to_owned());
    ctx.set_fonts(fonts);
}

impl eframe::App for MyApp {
    fn update(&mut self, ctx: &egui::Context, _frame: &mut eframe::Frame) {
        egui::CentralPanel::default().show(ctx, |ui| {
            ui.heading("计时器，按下 Left Ctrl 键开始计时，再按下 Left Ctrl 键结束计时");
            ui.label(format!("{} 毫秒", self.time))
        });

        let keys: Vec<Keycode> = self.device_state.get_keys();
        if keys.len() == 0 {
            self.down_count = 0;
            return;
        }
        self.down_count += 1;

        // 跳过除 Left Ctrl 的按键
        if keys[0] != Keycode::LControl || self.down_count != 1 {
            return;
        }
        let status: &str = &self.status;
        match status {
            "init" => {
                self.status = "started".to_owned();
                self.start_time = Instant::now();
            }
            "started" => {
                self.time = self.start_time.elapsed().as_millis();
                self.status = "init".to_owned();
                return;
            }
            _ => {
                return;
            }
        }
    }
}
