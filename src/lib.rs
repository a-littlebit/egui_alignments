//! # egui_alignments
//!
//! Simple alignment tools for egui
//!
//! ## Example Usage
//!
//! ### Align a single widget
//!
//! ```rust
//! use egui::{Button, Label};
//! use egui_alignments::AlignedWidget;
//!
//! # egui::__run_test_ui(|ui| {
//! Label::new("This label will be shown at the top")
//!     .top(ui);
//!
//! if Button::new("This label will be shown at the center")
//!     .center(ui)
//!     .clicked()
//! {
//!     println!("Center button clicked!");
//! }
//!
//! Label::new("This label will be shown at the bottom")
//!     .bottom(ui);
//! # });
//! ```
//!
//! ### Align multiple widgets
//!
//! The following buttons will be shown at the center of the screen horizontally
//! with the tip text above and click results below.
//!
//! ```rust
//! use egui::{Button, Widget};
//! use egui_alignments::{center_horizontal, center_vertical};
//!
//! let mut clicked_button = None;
//!
//! # egui::__run_test_ui(|ui| {
//! center_vertical(ui, |ui| {
//!     ui.label("Click a button");
//!
//!     ui.add_space(20.0);
//!
//!     center_horizontal(ui, |ui| {
//!         for i in 1..=10 {
//!             if Button::new(format!("Button {}", i))
//!                 .ui(ui)
//!                 .clicked()
//!             {
//!                 clicked_button = Some(i);
//!             }
//!         }
//!     });
//!
//!     ui.add_space(20.0);
//!
//!     if let Some(i) = clicked_button {
//!         ui.label(format!("You clicked button {}", i));
//!     };
//! });
//! # });
//! ```


pub mod aligned_widget;
pub mod aligner;

pub use aligned_widget::*;
pub use aligner::*;