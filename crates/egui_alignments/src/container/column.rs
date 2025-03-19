use std::f32::INFINITY;

use egui::{vec2, Align, Id, InnerResponse, Layout, Margin, Ui};

use super::Container;

/// A container which aligns its contents vertically.
/// See module [`crate::container`] for example usage.
pub struct Column {
    /// The id of the column. Used for memorize content size.
    /// If `None`, the id will be generated automatically.
    pub id: Option<Id>,

    /// The horizontal alignment of the column items.
    pub halign: Align,

    /// The padding of the column items.
    pub padding: Margin,

    /// If `true`, the items will be arranged from bottom to top.
    /// If `false`, the items will be arranged from top to bottom.
    /// Default: `false`.
    pub bottom_up: bool,

    /// The maximum width of the column.
    pub max_width: f32,

    /// The minimum width of the column.
    pub min_width: f32,
}

impl Column {
    #[inline]
    /// Create a new column with the given horizontal alignment.
    pub fn new(halign: Align) -> Self {
        Self {
            id: None,
            halign,
            padding: Margin::ZERO,
            bottom_up: false,
            max_width: INFINITY,
            min_width: 0.0,
        }
    }

    #[inline]
    /// Set the id of the column.
    pub fn id(mut self, id: Id) -> Self {
        self.id = Some(id);
        self
    }

    #[inline]
    /// Set the horizontal alignment of the column items.
    pub fn halign(mut self, align: Align) -> Self {
        self.halign = align;
        self
    }

    #[inline]
    /// Set the padding of the column items.
    pub fn padding(mut self, padding: impl Into<Margin>) -> Self {
        self.padding = padding.into();
        self
    }

    #[inline]
    /// Set the bottom-up mode of the column.
    pub fn bottom_up(mut self, bottom_up: bool) -> Self {
        self.bottom_up = bottom_up;
        self
    }

    #[inline]
    /// Set the fixed width of the column.
    pub fn width(mut self, width: f32) -> Self {
        self.min_width = width;
        self.max_width = width;
        self
    }

    #[inline]
    /// Set the maximum width of the column.
    pub fn max_width(mut self, width: f32) -> Self {
        self.max_width = width;
        self
    }

    #[inline]
    /// Set the minimum width of the column.
    pub fn min_width(mut self, width: f32) -> Self {
        self.min_width = width;
        self
    }
}

impl Default for Column {
    fn default() -> Self {
        Self::new(Align::Min)
    }
}

impl Column {
    /// Show the column in the given ui.
    pub fn show<R>(
        &self,
        ui: &mut Ui,
        add_contents: impl FnOnce(&mut Ui) -> R,
    ) -> InnerResponse<R> {
        let Self {
            id,
            halign,
            padding,
            max_width,
            min_width,
            ..
        } = *self;

        let layout = if self.bottom_up {
            Layout::bottom_up(halign)
        } else {
            Layout::top_down(halign)
        };

        Container {
            id,
            layout,
            padding,
            max_size: vec2(max_width, INFINITY),
            min_size: vec2(min_width, 0.0),
        }
        .show(ui, add_contents)
    }
}

#[inline]
/// Create a new column
///
/// # Example
/// ```rust
/// use egui::Align;
/// use egui_alignments::column;
///
/// # egui::__run_test_ui(|ui| {
/// column(ui, Align::Center, |ui| {
///     ui.label("Top floor");
///     ui.label("Second floor");
///     ui.label("First floor");
/// });
/// # });
/// ```
pub fn column<R>(
    ui: &mut Ui,
    halign: Align,
    add_contents: impl FnOnce(&mut Ui) -> R,
) -> InnerResponse<R> {
    Column::new(halign).show(ui, add_contents)
}
