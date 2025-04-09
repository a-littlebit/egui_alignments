use std::f32::INFINITY;

use egui::{vec2, Align, Id, InnerResponse, Layout, Margin, Ui};

use super::Container;

/// A container which aligns its contents horizontally.
/// See module [`crate::container`] for example usage.
pub struct Row {
    /// The id of the row. Used to memorize the size of the contents.
    /// If None, the id will be generated automatically.
    pub id: Option<Id>,

    /// The vertical alignment of the row items.
    pub valign: Align,

    /// The padding of the row items.
    pub padding: Margin,

    /// If the row should be right-to-left,
    /// set to None to follow the local preferrence
    pub right_to_left: Option<bool>,

    /// If the row should wrap its contents, instead of overflowing.
    pub wrapping: bool,

    /// The maximum height of the row.
    pub max_height: f32,

    /// The minimum height of the row.
    pub min_height: f32,
}

impl Row {
    #[inline]
    /// Create a new row with the given vertical alignment.
    pub fn new(valign: Align) -> Self {
        Self {
            id: None,
            valign,
            padding: Margin::ZERO,
            right_to_left: None,
            wrapping: false,
            max_height: INFINITY,
            min_height: 0.0,
        }
    }

    #[inline]
    /// Set the id of the row.
    pub fn id(mut self, id: Id) -> Self {
        self.id = Some(id);
        self
    }

    #[inline]
    /// Set the vertical alignment of the row items.
    pub fn valign(mut self, align: Align) -> Self {
        self.valign = align;
        self
    }

    #[inline]
    /// Set the padding of the row items.
    pub fn padding(mut self, padding: impl Into<Margin>) -> Self {
        self.padding = padding.into();
        self
    }

    #[inline]
    /// Set the right-to-left mode of the row.
    pub fn right_to_left(mut self, right_to_left: bool) -> Self {
        self.right_to_left = Some(right_to_left);
        self
    }

    #[inline]
    /// Set the wrapping mode of the row.
    pub fn wrapping(mut self, wrapping: bool) -> Self {
        self.wrapping = wrapping;
        self
    }

    #[inline]
    /// Set the maximum height of the row.
    pub fn max_height(mut self, max_height: f32) -> Self {
        self.max_height = max_height;
        self
    }

    #[inline]
    /// Set the minimum height of the row.
    pub fn min_height(mut self, min_height: f32) -> Self {
        self.min_height = min_height;
        self
    }
}

impl Default for Row {
    fn default() -> Self {
        Self::new(Align::Min)
    }
}

impl Row {
    /// Show the row in the given ui.
    pub fn show<R>(
        &self,
        ui: &mut Ui,
        add_contents: impl FnOnce(&mut Ui) -> R,
    ) -> InnerResponse<R> {
        let Self {
            id,
            valign,
            padding,
            max_height,
            min_height,
            ..
        } = *self;

        let right_to_left = self
            .right_to_left
            .unwrap_or(ui.layout().prefer_right_to_left());

        // If wrapping is enabled, Align::Center or Align::Max will cause the row to take up
        // the full height of the ui.
        let valign = if self.wrapping { Align::Min } else { valign };
        let layout = if right_to_left {
            Layout::right_to_left(valign)
        } else {
            Layout::left_to_right(valign)
        }
        .with_main_wrap(self.wrapping);

        Container {
            id,
            layout,
            padding,
            max_size: vec2(INFINITY, max_height),
            min_size: vec2(0.0, min_height),
        }
        .show(ui, add_contents)
    }
}

#[inline]
/// Create a new row
///
/// # Example
/// ```rust
/// use egui::Align;
/// use egui_alignments::row;
///
/// # egui::__run_test_ui(|ui| {
/// row(ui, Align::Center, |ui| {
///     ui.label("Left side");
///     ui.label("Middle");
///     ui.label("Right side");
/// });
/// # });
/// ```
pub fn row(ui: &mut Ui, valign: Align, add_contents: impl FnOnce(&mut Ui)) -> InnerResponse<()> {
    Row::new(valign).show(ui, add_contents)
}
