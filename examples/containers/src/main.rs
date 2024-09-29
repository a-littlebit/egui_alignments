#![cfg_attr(not(debug_assertions), windows_subsystem = "windows")] // hide console window on Windows in release
#![allow(rustdoc::missing_crate_level_docs)] // it's an example

use eframe::egui;
use egui::vec2;
use egui_alignments::{column, row, AlignedWidget};

fn main() -> eframe::Result {
    let options = eframe::NativeOptions {
        viewport: egui::ViewportBuilder::default().with_inner_size([400.0, 320.0]),
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
            ui.spacing_mut().item_spacing = vec2(6.0, 12.0);
            
            ui.centered_and_justified(|ui| {
                column(ui, egui::Align::Center, |ui| {
                    ui.heading("My egui Application");
            
                    row(ui, egui::Align::Center, |ui| {
                        let name_label = ui.label("Your name: ");

                        column(ui, egui::Align::Center, |ui| {
                            ui.spacing_mut().item_spacing.y = 6.0;
                            if ui.button("Increment").clicked() {
                                self.age += 1;
                            }
                            if ui.button("Decrement").clicked() {
                                self.age -= 1;
                            }
                        });

                        ui.text_edit_singleline(&mut self.name)
                            .labelled_by(name_label.id);
                    });
                    
                    egui::Slider::new(&mut self.age, 0..=120).text("age").top(ui);
                    ui.label(format!("Hello '{}', age {}", self.name, self.age));

                    ui.image(egui::include_image!(
                        "../../assets/ferris.png"
                    ));
                });
            });
        });
    }
}
