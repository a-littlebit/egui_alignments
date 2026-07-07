#![cfg_attr(not(debug_assertions), windows_subsystem = "windows")] // hide console window on Windows in release
#![allow(rustdoc::missing_crate_level_docs)] // it's an example

use eframe::{egui, Frame};
use egui::{vec2, Label, Ui, Widget};
use egui_alignments::{column, row, stretch, stretch_with_weight, Alignable, Row};

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
    fn ui(&mut self, ui: &mut Ui, _frame: &mut Frame) {
        egui::CentralPanel::default().show(ui, |ui| {
            ui.spacing_mut().item_spacing = vec2(6.0, 12.0);

            ui.centered_and_justified(|ui| {
                column(ui, egui::Align::Center, |ui| {
                    stretch_with_weight(ui, 2.0);

                    ui.heading("My egui Application");

                    stretch(ui);

                    let edit_row = Row::new(egui::Align::Min).wrapping(true);
                    edit_row.show(ui, |ui| {
                        let name_label = Label::new("Your name: ")
                            .wrap_mode(egui::TextWrapMode::Extend)
                            .ui(ui);

                        stretch(ui);

                        ui.text_edit_singleline(&mut self.name)
                            .labelled_by(name_label.id);
                    });

                    stretch(ui);

                    row(ui, egui::Align::Min, |ui| {
                        column(ui, egui::Align::Center, |ui| {
                            ui.spacing_mut().item_spacing.y = 6.0;
                            if ui.button("Increment").clicked() {
                                self.age += 1;
                            }
                            if ui.button("Decrement").clicked() {
                                self.age -= 1;
                            }
                        });

                        row(ui, egui::Align::Center, |ui| {
                            egui::Slider::new(&mut self.age, 0..=120)
                                .text("age")
                                .top(ui);
                        });
                    });

                    ui.label(format!("Hello '{}', age {}", self.name, self.age));

                    stretch(ui);

                    ui.image(egui::include_image!("../../assets/ferris.png"));

                    stretch_with_weight(ui, 2.0);
                });
            });
        });
    }
}
