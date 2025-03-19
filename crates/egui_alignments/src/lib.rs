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
//! use egui_alignments::Alignable;
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
//! # egui::__run_test_ui(|ui| {
//! let mut clicked_button = None;
//!
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
//!
//! ### Use containers
//!
//! Sometimes nested calls to alignment functions like `center_horizontal`, `top_vertical`, ...
//! may cause layout confusion due to interaction between inner and outer layouts.
//!
//! To prevent inner layouts from disrupting the outer ones, you may use containers for inner layouts
//!
//! Containers only cares about its inner alignments and act like a simple widget in the outer layout
//!
//! The following is an example usage of containers
//!
//! ```rust
//! use egui::Align;
//! use egui_alignments::{center_horizontal, column, row};
//!
//! # egui::__run_test_ui(|ui| {
//! center_horizontal(ui, |ui| {
//!     ui.image("path/to/left/image");
//!     column(ui, Align::Center, |ui| {
//!         ui.label("top of right text");
//!         row(ui, Align::Center, |ui| {
//!             ui.label("left");
//!             ui.label("middle");
//!             ui.label("right");
//!         });
//!         ui.label("bottom of right text");
//!     });
//! });
//! # });
//! ```
//!
//! This will show an image on the left, and a column of text on the right which contains a row of three labels in the middle.
//!
//! ### Use stretches
//!
//! Sometimes you may want to make a widget stretch to fill the remaining space
//! between widgets instead of besides them in a container.
//!
//! Use `stretch` to achieve this.
//!
//! ```rust
//! use egui::Align;
//! use egui_alignments::{column, stretch};
//!
//! # egui::__run_test_ui(|ui| {
//! column(ui, Align::Center, |ui| {
//!     ui.label("Top");
//!     stretch(ui);
//!     ui.label("Bottom");
//! });
//! # });
//! ```
//!
//! If you want to have stretches with different sizes, you may use `stretch_with_weight`.
//!
//! ```rust
//! use egui::Align;
//! use egui_alignments::{column, stretch_with_weight};
//!
//! # egui::__run_test_ui(|ui| {
//! column(ui, Align::Center, |ui| {
//!     ui.label("100% height");
//!     stretch_with_weight(ui, 1.0);
//!     ui.label("75% height");
//!     stretch_with_weight(ui, 3.0);
//!     ui.label("0% height");
//! });
//! # });
//! ```

pub mod alignable;
pub mod aligner;
pub mod container;

pub use alignable::*;
pub use aligner::*;
pub use container::*;

use egui::{Align, Direction, Layout, Rect, Vec2};

// resize layout rect without moving the inner content.
// this is useful for layouts that contain growable widgets like `ScrollArea`.
pub(crate) fn resize_layout_rect(rect: Rect, size: Vec2, layout: &Layout) -> Rect {
    let mut new_rect = rect;
    let x_expand = size.x - rect.width();
    let y_expand = size.y - rect.height();

    let (halign, valign) = match layout.main_dir() {
        Direction::LeftToRight => (Align::Min, layout.cross_align),
        Direction::RightToLeft => (Align::Max, layout.cross_align),
        Direction::TopDown => (layout.cross_align, Align::Min),
        Direction::BottomUp => (layout.cross_align, Align::Max),
    };

    match halign {
        Align::Min => {
            new_rect.max.x += x_expand;
        }
        Align::Center => {
            // if the layout always allocate the full width even if it doesn't need that much
            // then we should not expand the rect
            if !layout.horizontal_justify() && !layout.is_vertical() {
                new_rect.min.x -= x_expand / 2.0;
                new_rect.max.x += x_expand / 2.0;
            }
        }
        Align::Max => {
            new_rect.min.x -= x_expand;
        }
    };

    match valign {
        Align::Min => {
            new_rect.max.y += y_expand;
        }
        Align::Center => {
            // if the layout always allocate the full height even if it doesn't need that much
            // then we should not expand the rect
            if !layout.vertical_justify() && !layout.is_horizontal() {
                new_rect.min.y -= y_expand / 2.0;
                new_rect.max.y += y_expand / 2.0;
            }
        }
        Align::Max => {
            new_rect.min.y -= y_expand;
        }
    }

    new_rect
}
