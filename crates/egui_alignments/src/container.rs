//! Simple layout containers
//! 
//! # Example
//! ```
//! use egui::Align;
//! use egui_alignments::{column, row};
//! 
//! # egui::__run_test_ui(|ui| {
//! column(ui, Align::Center, |ui| {
//!     ui.label("top");
//!     row(ui, Align::Center, |ui| {
//!         ui.label("left");
//!         ui.label("center");
//!         ui.label("right");
//!     });
//!     ui.label("bottom");
//! });
//! # });
//! ```

pub mod column;
pub mod row;

pub use column::*;
pub use row::*;

use egui::{Id, InnerResponse, Layout, Sense, Ui, UiBuilder, Vec2};

use crate::resize_layout_rect;

pub(crate) struct Container {
    pub(crate) id: Option<Id>,
    pub(crate) layout: Layout,
    pub(crate) padding: egui::Margin,
    pub(crate) max_size: Vec2,
    pub(crate) min_size: Vec2,
}

impl Container {
    pub(crate) fn show<R>(&self, ui: &mut Ui, add_contents: impl FnOnce(&mut Ui) -> R) -> InnerResponse<R> {
        // used to memorize content size
        let id = self.id.unwrap_or_else(|| {
            let id = ui.next_auto_id();
            ui.skip_ahead_auto_ids(1);
            id
        });

        // try to get content size from cache
        // if not cached, start a sizing pass
        let mut sizing_pass = false;
        let available_rect = ui.available_rect_before_wrap();
        let desired_size = ui.ctx().data_mut(|data| {
            data.get_temp(id)
        })
        .unwrap_or_else(|| {
            sizing_pass = true;
            // the current pass is a sizing pass, request a rendering pass
            ui.ctx().request_discard("new Container");
            available_rect.size()
        });

        // get the supposed content rect
        let content_rect = {
            let (_, next_rect) = ui.new_child(UiBuilder::new()).allocate_space(
                desired_size
                    .max(self.min_size)
                    .min(self.max_size)
            );
            let expanded_rect = resize_layout_rect(next_rect, available_rect.size(), &self.layout);
            expanded_rect - self.padding
        };

        // create child ui
        let mut content_ui = ui.new_child({
            let builder = UiBuilder::new()
                .max_rect(content_rect);
            
            if sizing_pass {
                builder.layout(
                        // in sizing pass, keep the layout size minimum
                        self.layout
                            .with_cross_align(egui::Align::Min)
                            .with_cross_justify(false)
                    )
                    .sizing_pass()
                    .invisible()
            } else {
                builder.layout(self.layout)
            }
        });

        // add contents and calculate space to be allocated
        let inner = add_contents(&mut content_ui);
        let new_rect = content_ui.min_rect() + self.padding;
        // allocate space and get response
        let response = ui.allocate_rect(new_rect, Sense::hover());

        // cache content size
        if sizing_pass || new_rect.size() != desired_size {
            ui.ctx().data_mut(|data| {
                data.insert_temp(id, new_rect.size())
            });
        }

        InnerResponse { inner, response, }
    }
}
