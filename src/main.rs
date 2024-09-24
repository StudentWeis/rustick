#![cfg_attr(not(debug_assertions), windows_subsystem = "windows")] // hide console window on Windows in release
#![allow(rustdoc::missing_crate_level_docs)] // it's an example

use eframe::egui;
use device_query::{ DeviceQuery, DeviceState, Keycode };
use std::sync::mpsc::{ self, Receiver };
use std::thread;
use std::time::Instant;

fn main() -> eframe::Result<()> {
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
    time: u128,
    status_receiver: Receiver<(String, u128)>,
}

impl Default for MyApp {
    fn default() -> Self {
        let (status_sender, status_receiver) = mpsc::channel();

        // Spawn a new thread to handle key input and status updates
        thread::spawn(move || {
            let device_state = DeviceState::new();
            let mut down_count = 0;
            let mut status = "init".to_string();
            let mut start_time = Instant::now();

            loop {
                let keys: Vec<Keycode> = device_state.get_keys();
                if keys.is_empty() {
                    down_count = 0;
                } else {
                    down_count += 1;
                }

                if down_count == 1 && keys.contains(&Keycode::LControl) {
                    match status.as_str() {
                        "init" => {
                            status = "started".to_string();
                            start_time = Instant::now();
                        }
                        "started" => {
                            let elapsed_time = start_time.elapsed().as_millis();
                            status = "init".to_string();
                            status_sender.send((status.clone(), elapsed_time)).unwrap();
                        }
                        _ => {}
                    }
                }
            }
        });

        Self {
            time: 0,
            status_receiver,
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
            ui.heading("计时器\n按下 Left Ctrl 键开始计时\n再次按下 Left Ctrl 键结束计时");
            ui.label(format!("{} 毫秒", self.time))
        });

        if let Ok((status, time)) = self.status_receiver.try_recv() {
            if status == "init" {
                self.time = time;
            }
        }
    }
}
