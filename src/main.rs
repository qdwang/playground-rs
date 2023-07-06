#![cfg_attr(not(debug_assertions), windows_subsystem = "windows")]
use eframe::{self, egui};

fn main() {
    let options = eframe::NativeOptions {
        renderer: eframe::Renderer::Wgpu,
        initial_window_size: Some(egui::vec2(320.0, 240.0)),
        ..Default::default()
    };

    eframe::run_native(
        "Test",
        options,
        Box::new(|cc| Box::new(MyApp)),
    ).unwrap();
}

struct MyApp;

impl eframe::App for MyApp {
    fn update(&mut self, ctx: &egui::Context, frame: &mut eframe::Frame) {
        egui::CentralPanel::default().show(ctx, |ui| {
            ui.label("abc");
            if ui.button("123 cfggkj").clicked() {
                ui.label("clicking");
            }
        });
    }
}
