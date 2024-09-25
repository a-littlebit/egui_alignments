#![cfg_attr(not(debug_assertions), windows_subsystem = "windows")] // hide console window on Windows in release
#![allow(rustdoc::missing_crate_level_docs)] // it's an example

use eframe::egui::{self, Label, Widget};
use egui::{Button, WidgetText};
use egui_alignments::{center_horizontal, AlignedWidget};

fn main() -> eframe::Result {
    let options = eframe::NativeOptions {
        viewport: egui::ViewportBuilder::default().with_inner_size([800.0, 200.0]),
        ..Default::default()
    };

    let mut clicked_button = None;

    eframe::run_simple_native("My egui App", options, move |ctx, _frame| {
        egui::CentralPanel::default().show(ctx, |ui| {
            Label::new(WidgetText::from("My egui alignments").heading()).top(ui);
            
            ui.horizontal(|ui| {
                Label::new("Try resize the window").left(ui);
                Label::new("Buttons should always be centered").right(ui);
            });
            
            center_horizontal(ui, |ui| {
                for i in 1..=10 {
                    if Button::new(format!("Button {}", i))
                        .ui(ui)
                        .clicked() {
                            clicked_button = Some(i);
                        }
                }
            });
            
            if let Some(button) = clicked_button {
                Label::new(format!("You clicked the button {}!", button))
                    .bottom(ui);
            } else {
                Label::new("Click a button!")
                    .bottom(ui);
            }
        });
    })
}
