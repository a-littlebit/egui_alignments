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

pub struct Container {
    pub id: Option<Id>,
    pub layout: Layout,
    pub padding: egui::Margin,
    pub max_size: Vec2,
    pub min_size: Vec2,
}

impl Default for Container {
    fn default() -> Self {
        Self {
            id: None,
            layout: Layout::default(),
            padding: egui::Margin::ZERO,
            max_size: Vec2::INFINITY,
            min_size: Vec2::ZERO,
        }
    }
}

impl Container {
    #[inline]
    pub fn new(layout: Layout) -> Self {
        Self {
            layout,
            ..Default::default()
        }
    }

    #[inline]
    pub fn id(mut self, id: Id) -> Self {
        self.id = Some(id);
        self
    }

    #[inline]
    pub fn layout(mut self, layout: Layout) -> Self {
        self.layout = layout;
        self
    }

    #[inline]
    pub fn padding(mut self, padding: egui::Margin) -> Self {
        self.padding = padding;
        self
    }

    #[inline]
    pub fn max_size(mut self, max_size: Vec2) -> Self {
        self.max_size = max_size;
        self
    }

    #[inline]
    pub fn min_size(mut self, min_size: Vec2) -> Self {
        self.min_size = min_size;
        self
    }

    pub fn show<R>(
        &self,
        ui: &mut Ui,
        add_contents: impl FnOnce(&mut Ui) -> R,
    ) -> InnerResponse<R> {
        // used to memorize content size
        let id = self.id.unwrap_or_else(|| {
            let id = ui.next_auto_id();
            ui.skip_ahead_auto_ids(1);
            id
        });

        // try to get content size from cache
        // if not cached, start a sizing pass
        let mut sizing_pass = false;
        // make sure available_rect shrinks when screen rect is shrinking
        let available_rect = ui
            .available_rect_before_wrap()
            .intersect(ui.ctx().content_rect());
        let desired_size = ui
            .ctx()
            .data_mut(|data| data.get_temp(id))
            .unwrap_or_else(|| {
                sizing_pass = true;
                // the current pass is a sizing pass, request a rendering pass
                ui.ctx().request_discard("new Container");
                available_rect.size()
            });

        // get the expected content rect
        let (_, expected_rect) = ui
            .new_child(UiBuilder::new())
            .allocate_space(desired_size.max(self.min_size).min(self.max_size));
        let content_rect =
            resize_layout_rect(expected_rect, available_rect.size(), &self.layout) - self.padding;

        // create child ui
        let mut content_ui = ui.new_child({
            let builder = UiBuilder::new().max_rect(content_rect);

            if sizing_pass {
                builder
                    .layout(
                        // in sizing pass, keep the layout size minimum
                        self.layout
                            .with_cross_align(egui::Align::Min)
                            .with_cross_justify(false),
                    )
                    .sizing_pass()
                    .invisible()
            } else {
                builder.layout(self.layout)
            }
        });

        // prepare data for stretch
        let stretch_space = if self.layout.is_horizontal() {
            available_rect.width() - desired_size.x
        } else {
            available_rect.height() - desired_size.y
        };
        let last_weights = prepare_stretch(&mut content_ui, stretch_space);

        // add contents and calculate space to be allocated
        let inner = add_contents(&mut content_ui);
        let new_rect = content_ui.min_rect() + self.padding;
        // allocate space and get response
        // when already arranged, even if the content has grown, we allocate the expected size
        // if we allocate the whole new rect, it's actually in the wrong place and could disrupt the layout
        let response = ui.allocate_rect(
            if sizing_pass { new_rect } else { expected_rect },
            Sense::hover(),
        );

        // finish stretch
        finish_stretch(&mut content_ui, last_weights);

        // cache content size
        if sizing_pass || new_rect.size() != desired_size {
            ui.ctx()
                .data_mut(|data| data.insert_temp(id, new_rect.size()));
        }

        InnerResponse { inner, response }
    }
}

const STRETCH_SPACE_ID_SALT: &'static str = "egui_alignments::container::STRETCH_SPACE_ID_SALT";
const STRETCH_WEIGHT_ID_SALT: &'static str = "egui_alignments::container::STRETCH_WEIGHT_ID_SALT";
const STRETCH_WRAPPED_ID_SALT: &'static str = "egui_alignments::container::STRETCH_WRAPPED_ID_SALT";

pub(crate) fn register_stretch(ui: &mut Ui, weight: f32) -> Option<f32> {
    if weight <= 0.0 {
        return None;
    }

    let space_id = ui.unique_id().with(STRETCH_SPACE_ID_SALT);
    let weight_id = ui.unique_id().with(STRETCH_WEIGHT_ID_SALT);

    let spaces: Vec<f32> = ui.data(|data| data.get_temp(space_id))?;
    let mut weights: Vec<f32> = ui.data(|data| data.get_temp(weight_id))?;

    // calculate index based on the number of registered stretches
    let index = weights.len();

    // register weight of self
    weights.push(weight);
    ui.data_mut(|data| {
        data.insert_temp(weight_id, weights);
    });

    Some(*spaces.get(index)?)
}

fn prepare_stretch(ui: &mut Ui, available_space: f32) -> Option<Vec<f32>> {
    let space_id = ui.unique_id().with(STRETCH_SPACE_ID_SALT);
    let weight_id = ui.unique_id().with(STRETCH_WEIGHT_ID_SALT);
    let wrapped_id = ui.unique_id().with(STRETCH_WRAPPED_ID_SALT);

    let (Some(last_spaces), Some(last_weights)) = ui.data(|data| {
        (
            data.get_temp::<Vec<f32>>(space_id),
            data.get_temp::<Vec<f32>>(weight_id),
        )
    }) else {
        ui.data_mut(|data| {
            data.insert_temp(space_id, Vec::<f32>::new());
            data.insert_temp(weight_id, Vec::<f32>::new());
        });
        return None;
    };
    let mut available_space = (available_space + last_spaces.iter().sum::<f32>()).max(0.0);
    let wrapped = ui.data(|data| data.get_temp::<bool>(wrapped_id).unwrap_or(false));
    if wrapped {
        // if the layout is wrapped, we consider that there is no space left
        available_space = 0.0;
    }
    let last_weights_sum: f32 = last_weights.iter().sum();

    // calculate sizes
    let mut spaces = Vec::with_capacity(last_weights.len());
    for weight in last_weights.iter() {
        let space = available_space * weight / last_weights_sum;
        spaces.push(space);
    }

    // prepare data for stretch
    ui.data_mut(|data| {
        data.insert_temp(space_id, spaces);
        data.insert_temp(weight_id, Vec::<f32>::new());
    });

    Some(last_weights)
}

fn finish_stretch(ui: &mut Ui, last_weights: Option<Vec<f32>>) {
    let weight_id = ui.unique_id().with(STRETCH_WEIGHT_ID_SALT);
    let wrapped_id = ui.unique_id().with(STRETCH_WRAPPED_ID_SALT);

    let Some(last_weights) = last_weights else {
        return;
    };

    // release data and check if the weights changed
    let Some(new_weights) = ui.data(|r| r.get_temp::<Vec<f32>>(weight_id)) else {
        return;
    };

    // check if the layout is wrapped
    let mut wrapped = false;
    if ui.layout().main_wrap {
        wrapped = if ui.layout().is_horizontal() {
            ui.cursor().top() > ui.min_rect().top()
        } else {
            ui.cursor().left() > ui.min_rect().left()
        };
    }
    ui.data_mut(|data| {
        data.insert_temp(wrapped_id, wrapped);
    });

    // request another pass if the weights changed
    if last_weights != new_weights {
        ui.ctx().request_discard("container stretch changed");
    }
}

/// Stretch the available space with the given weight. Only available in a container.
///
/// # Example
/// ```rust
/// use egui::Align;
/// use egui_alignments::{column, stretch_with_weight};
///
/// # egui::__run_test_ui(|ui| {
/// column(ui, Align::Center, |ui| {
///     ui.label("100% height");
///     stretch_with_weight(ui, 1.0);
///     ui.label("75% height");
///     stretch_with_weight(ui, 3.0);
///     ui.label("0% height");
/// });
/// # });
/// ```
pub fn stretch_with_weight(ui: &mut Ui, weight: f32) -> f32 {
    let space = register_stretch(ui, weight);

    if let Some(space) = space {
        if space > 0.0 {
            ui.add_space(space);
        }
        space
    } else {
        0.0
    }
}

#[inline]
/// Stretch the available space. Only available in a container.
/// If there are multiple stretches in a container, they will share the available space in average.
/// If you want to have stretches with different sizes in a container, use [`stretch_with_weight`] instead.
///
/// # Example
/// ```rust
/// use egui::Align;
/// use egui_alignments::{column, stretch};
///
/// # egui::__run_test_ui(|ui| {
/// column(ui, Align::Center, |ui| {
///     ui.label("Top");
///     stretch(ui);
///     ui.label("Bottom");
/// });
/// # });
/// ```

pub fn stretch(ui: &mut Ui) -> f32 {
    stretch_with_weight(ui, 1.0)
}
