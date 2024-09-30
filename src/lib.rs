use db::{ insert_log, get_all_logs, Log };
use device_query::{ DeviceQuery, DeviceState, Keycode };
use std::sync::mpsc::{ self, Receiver };
use std::thread;
use std::time::Instant;
use eframe::egui::{ self, Window };

mod db;

pub struct MyApp {
    time: u128,
    status_receiver: Receiver<(String, u128)>,
    tick_flag: bool,
    can_tick: bool,
    name: String,
    show_log_window: bool,
    logs: Vec<Log>,
    menu_config: MenuConfig,
}

impl Default for MyApp {
    fn default() -> Self {
        Self {
            time: 0,
            status_receiver,
            tick_flag: false,
            can_tick: true,
            name: "".to_string(),
            show_log_window: false,
            logs: Vec::new(),
            menu_config: MenuConfig::default(),
        }
    }
}

impl MyApp {
    pub fn new(cc: &eframe::CreationContext<'_>) -> Self {
        load_fonts(&cc.egui_ctx);
        // 创建另一个线程执行键盘监听
        let (status_sender, status_receiver) = mpsc::channel();
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
        // 主体 UI
        egui::CentralPanel::default().show(ctx, |ui| {
            egui::ScrollArea::vertical().show(ui, |ui| {
                ui.horizontal(|ui| {
                    ui.heading("Rustick 计时器");
                    ui.menu_button("菜单", |ui| {
                        ui.checkbox(&mut self.menu_config.dark_mode, "黑暗模式");
                        ui.checkbox(&mut self.menu_config.top, "置顶");
                    });
                    // 日志获取懒加载
                    if ui.button("日志").clicked() {
                        self.logs = get_all_logs();
                        self.show_log_window = !self.show_log_window;
                    }
                    ui.menu_button("提示", |ui| {
                        ui.label("按下左边的 Ctrl 开始计时\n再次按下结束计时");
                    });
                });
            });
            if self.show_log_window {
                Window::new("计时日志")
                    .open(&mut self.show_log_window)
                    .show(ctx, |ui| {
                        egui::ScrollArea::vertical().show(ui, |ui| {
                            for log in &self.logs {
                                ui.label(
                                    format!(
                                        "{}: {} - {} ms",
                                        log.datetime,
                                        log.message,
                                        log.ticktime
                                    )
                                );
                            }
                        });
                    });
            }

            ui.separator();
            ui.vertical_centered(|ui| {
                ui.add_space(5.0);
                ui.horizontal(|ui| {
                    ui.checkbox(&mut self.can_tick, "开启使用");
                    // 事项记录
                    ui.label("事项: ");
                    ui.text_edit_singleline(&mut self.name);
                });
                ui.add_space(10.0);
                // 显示时间
                if self.time == 0 {
                    ui.label("-");
                } else if self.time < 1000 {
                    ui.label(format!("{} 毫秒", self.time));
                } else if self.time > 1000 {
                    if self.time % 1000 < 100 {
                        ui.label(format!("{}.0{} 秒", self.time / 1000, self.time % 1000));
                    } else {
                        ui.label(format!("{}.{} 秒", self.time / 1000, self.time % 1000));
                    }
                }
                // 显示计时状态
                if self.tick_flag {
                    ui.label("正在计时...");
                } else {
                    ui.label("未开始计时👌");
                }
            });

            ui.add_space(19.0);
            ui.vertical_centered(|ui| {
                ui.label("v0.2.2");
            });
        });

        // 置顶功能
        if self.menu_config.top {
            ctx.send_viewport_cmd(
                egui::ViewportCommand::WindowLevel(egui::WindowLevel::AlwaysOnTop)
            );
        } else {
            ctx.send_viewport_cmd(egui::ViewportCommand::WindowLevel(egui::WindowLevel::Normal));
        }

        // 主题设置
        if self.menu_config.dark_mode {
            ctx.set_visuals(egui::Visuals::dark());
        } else {
            ctx.set_visuals(egui::Visuals::light());
        }

        // 计时消息
        if let Ok((status, time)) = self.status_receiver.try_recv() {
            if self.can_tick {
                if status == "init" {
                    // 计时完毕
                    self.time = time;
                    self.tick_flag = false;
                    // 自动弹出
                    ctx.send_viewport_cmd(egui::ViewportCommand::Focus);
                    // 日志记录
                    insert_log(self.name.clone(), self.time.to_string());
                } else {
                    // 开始计时
                    self.time = 0;
                    self.tick_flag = true;
                }
            }
        }
    }
}

struct MenuConfig {
    dark_mode: bool,
    top: bool,
}

impl Default for MenuConfig {
    fn default() -> Self {
        Self { dark_mode: true, top: false }
    }
}
