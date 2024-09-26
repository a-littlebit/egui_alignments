#![cfg_attr(not(debug_assertions), windows_subsystem = "windows")] // hide console window on Windows in release
#![allow(rustdoc::missing_crate_level_docs)] // it's an example

use eframe::egui;
use egui::{vec2, Button, Image, Label, WidgetText};
use egui_alignments::{top_horizontal, AlignedWidget};

fn main() -> eframe::Result {
    let options = eframe::NativeOptions {
        viewport: egui::ViewportBuilder::default().with_inner_size([320.0, 240.0]),
        ..Default::default()
    };
    eframe::run_native(
        "My egui App",
        options,
        Box::new(|cc| {
            // This gives us image support:
            egui_extras::install_image_loaders(&cc.egui_ctx);

            Ok(Box::<MyApp>::default())
        }),
    )
}

struct MyApp {
    name: String,
    age: u32,
}

impl Default for MyApp {
    fn default() -> Self {
        Self {
            name: "Arthur".to_owned(),
            age: 42,
        }
    }
}

impl eframe::App for MyApp {
    fn update(&mut self, ctx: &egui::Context, _frame: &mut eframe::Frame) {
        egui::CentralPanel::default().show(ctx, |ui| {
            Label::new(WidgetText::from("My egui Application").heading()).top(ui);
            top_horizontal(ui, |ui| {
                let name_label = ui.label("Your name: ");
                ui.text_edit_singleline(&mut self.name)
                    .labelled_by(name_label.id);
            });
            egui::Slider::new(&mut self.age, 0..=120).text("age").top(ui);
            if Button::new("Increment").top(ui).clicked() {
                self.age += 1;
            }
            Label::new(format!("Hello '{}', age {}", self.name, self.age)).top(ui);

            Image::new(egui::include_image!(
                "assets/ferris.png"
            ))
            .fit_to_exact_size(vec2(200.0, 200.0))
            .center(ui);
        });
    }
}
