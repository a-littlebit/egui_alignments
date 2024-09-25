use egui::{Align2, Response, Ui, Widget};

use crate::*;

/// A widget that can adjust its position using an [`Aligner`]
/// or a widget aligns its itself using the specified [`Aligner`]
/// before rendering.
/// 
/// # Examples
/// ```
/// use egui::{Button, Label};
/// use egui_alignments::AlignedWidget;
/// 
/// # egui::__run_test_ui(|ui| {
/// Label::new("This label will be shown at the top")
///     .top(ui);
/// if Button::new("This label will be shown at the center")
///     .center(ui)
///     .clicked() {
///         println!("Center button clicked!");
///     }
/// Label::new("This label will be shown at the bottom")
///     .bottom(ui);
/// # });
/// ```
pub trait AlignedWidget: Widget + Sized {
    /// Show the widget at the position specified by the [`Aligner`].
    fn align(self, ui: &mut Ui, align: impl Aligner) -> Response;

    /// Show the widget at the center of the available space.
    fn center(self, ui: &mut Ui) -> Response {
        self.align(ui, Align2::CENTER_CENTER)
    }

    /// Show the widget at the top of the available space.
    fn top(self, ui: &mut Ui) -> Response {
        self.align(ui, Align2::CENTER_TOP)
    }

    /// Show the widget at the bottom of the available space.
    fn bottom(self, ui: &mut Ui) -> Response {
        self.align(ui, Align2::CENTER_BOTTOM)
    }

    /// Show the widget at the left of the available space.
    fn left(self, ui: &mut Ui) -> Response {
        self.align(ui, Align2::LEFT_CENTER)
    }

    /// Show the widget at the right of the available space.
    fn right(self, ui: &mut Ui) -> Response {
        self.align(ui, Align2::RIGHT_CENTER)
    }

    /// Show the widget at the top left of the available space.
    fn top_left(self, ui: &mut Ui) -> Response {
        self.align(ui, Align2::LEFT_TOP)
    }

    /// Show the widget at the top right of the available space.
    fn top_right(self, ui: &mut Ui) -> Response {
        self.align(ui, Align2::RIGHT_TOP)
    }

    /// Show the widget at the bottom left of the available space.
    fn bottom_left(self, ui: &mut Ui) -> Response {
        self.align(ui, Align2::LEFT_BOTTOM)
    }

    /// Show the widget at the bottom right of the available space.
    fn bottom_right(self, ui: &mut Ui) -> Response {
        self.align(ui, Align2::RIGHT_BOTTOM)
    }
}

/// Implements [`AlignedWidget`] for all [`Widget`]s
/// by adjust their positions using [`WidgetAligner`].
impl<T: Widget> AlignedWidget for T {
    fn align(self, ui: &mut Ui, align: impl Aligner) -> Response {
        WidgetAligner::from_align(align)
            .show(ui, |ui| self.ui(ui))
            .inner
    }
}