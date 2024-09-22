#![cfg_attr(not(debug_assertions), windows_subsystem = "windows")] // hide console window on Windows in release
#![allow(rustdoc::missing_crate_level_docs)] // it's an example

use eframe::egui;
use egui::Button;

use crate::backend::metadata;

pub fn main() -> eframe::Result {
    env_logger::init(); // Log to stderr (if you run with `RUST_LOG=debug`).
    let options = eframe::NativeOptions {
        viewport: egui::ViewportBuilder::default().with_inner_size([1280.0, 720.0]),
        ..Default::default()
    };
    eframe::run_native(
        "Solace",
        options,
        Box::new(|cc| {
            // This gives us image support:
            egui_extras::install_image_loaders(&cc.egui_ctx);

            Ok(Box::<MyApp>::default())
        }),
    )
}

struct MyApp {
    servers: Vec<String>,
}

impl Default for MyApp {
    fn default() -> Self {
        Self {
            servers: metadata::get_server_list(),
        }
    }
}
impl eframe::App for MyApp {
    fn update(&mut self, ctx: &egui::Context, _frame: &mut eframe::Frame) {
        let screen_size = ctx.screen_rect().size();
        egui::CentralPanel::default().show(ctx, |ui| {
            egui::SidePanel::left("side_left")
                .exact_width(screen_size.x * 0.09)
                .show(ctx, |ui| {
                    ui.heading("Servers");
                    for item in &self.servers {
                        if ui.button(item).clicked() {
                            println!("Button was clciked")
                        }
                    }
                })
        });
    }
}
