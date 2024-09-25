use device_query::{ DeviceQuery, DeviceState, Keycode };
use std::sync::mpsc::{ self, Receiver };
use std::thread;
use std::time::Instant;
use eframe::egui;

pub struct MyApp {
    time: u128,
    status_receiver: Receiver<(String, u128)>,
    tick_flag: bool,
    menu_config: MenuConfig,
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
                            status_sender.send((status.clone(), 0)).unwrap();
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
            tick_flag: false,
            menu_config: MenuConfig::default(),
        }
    }
}

impl MyApp {
    pub fn new(cc: &eframe::CreationContext<'_>) -> Self {
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
        // 置顶功能
        if self.menu_config.top {
            ctx.send_viewport_cmd(
                egui::ViewportCommand::WindowLevel(egui::WindowLevel::AlwaysOnTop)
            );
        } else {
            ctx.send_viewport_cmd(egui::ViewportCommand::WindowLevel(egui::WindowLevel::Normal));
        }

        egui::CentralPanel::default().show(ctx, |ui| {
            egui::ScrollArea::vertical().show(ui, |ui| {
                ui.horizontal(|ui| {
                    ui.heading("Rustick 计时器");
                    ui.menu_button("菜单", |ui| {
                        ui.checkbox(&mut self.menu_config.dark_mode, "黑暗模式");
                        ui.checkbox(&mut self.menu_config.top, "置顶");
                    });
                    ui.menu_button("提示", |ui| {
                        ui.label("按下左边的 Ctrl 开始计时\n再次按下结束计时");
                    });
                });
            });
            ui.separator();
            ui.vertical_centered(|ui| {
                ui.add_space(10.0);
                ui.label(format!("{} 毫秒", self.time));
                if self.tick_flag {
                    ui.label("正在计时...");
                } else {
                    ui.label("未开始计时👌");
                }
            });
            ui.add_space(19.0);
            ui.vertical_centered(|ui| {
                ui.label("v0.1.7");
            });
        });

        // 主题设置
        if self.menu_config.dark_mode {
            ctx.set_visuals(egui::Visuals::dark());
        } else {
            ctx.set_visuals(egui::Visuals::light());
        }

        // 计时消息
        if let Ok((status, time)) = self.status_receiver.try_recv() {
            if status == "init" {
                self.time = time;
                self.tick_flag = false;
                ctx.send_viewport_cmd(egui::ViewportCommand::Focus); // 计时完毕自动弹出
            } else {
                self.time = 0;
                self.tick_flag = true;
            }
        }
    }
}

#[derive(Default)]
struct MenuConfig {
    dark_mode: bool,
    top: bool,
}
