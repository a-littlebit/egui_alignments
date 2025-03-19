use egui::{
    Align, Align2, Id, InnerResponse, Layout, Margin, Pos2, Rect, Sense, Ui, UiBuilder, Vec2,
};

use crate::resize_layout_rect;

/// Represents an alignment strategy.
/// You can directly use `egui::Align2` or closure `FnOnce(egui::Vec2, egui::Rect) -> egui::Rect`
/// to align the contents.
/// Or you can implement your own alignment.
pub trait Alignment {
    fn align(self, item_size: Vec2, bounds: Rect) -> Rect;
}

impl Alignment for egui::Align2 {
    fn align(self, item_size: Vec2, bounds: Rect) -> Rect {
        self.align_size_within_rect(item_size, bounds)
    }
}

impl<T> Alignment for T
where
    T: FnOnce(Vec2, Rect) -> Rect,
{
    fn align(self, item_size: Vec2, bounds: Rect) -> Rect {
        self(item_size, bounds)
    }
}

/// Determines how [`Aligner`] allocate space for the aligned contents.
#[derive(Copy, Clone, Debug, PartialEq, Eq)]
pub enum AllocateType {
    /// Allocate no space.
    None,

    /// Allocate only the space allocated by the contents
    Content,

    /// Allocate only the height allocated by the contents
    /// and the whole width specified to align the contents.
    ContentRow,

    /// Allocate only the width allocated by the contents
    /// and the whole height specified to align the contents.
    ContentColumn,

    /// Allocate the whole bounds specified to align the contents.
    Bounds,
}

/// The bounds in which its contents will be aligned.
#[derive(Copy, Clone, Debug, PartialEq)]
pub enum Bounds {
    /// Align in Ui's next widget position with the given size.
    AvailableRect(Vec2),

    /// Align in the whole Ui, ignoring the specified margin.
    MaxRect(Margin),
}

impl Bounds {
    #[inline]
    /// Align in all the available space.
    pub fn available_rect() -> Self {
        Bounds::AvailableRect(Vec2::INFINITY)
    }

    #[inline]
    /// Align in the whole Ui.
    pub fn max_rect() -> Self {
        Bounds::MaxRect(0.0.into())
    }
}

/// A container which aligns its contents
/// within the given alignment and bounds.
///
/// # Example
/// ```
/// use egui_alignments::Aligner;
///
/// # egui::__run_test_ui(|ui| {
/// Aligner::center()
///     .show(ui, |ui| {
///         ui.label("This label will be shown at the center");
///     });
/// # });
/// ```
pub struct Aligner<T: Alignment> {
    /// Used to memorize content size.
    /// If not set, the id will be generated automatically.
    pub id: Option<Id>,

    /// The alignment.
    /// Could be a `egui::Align2`, a closure or a custom [`Alignment`].
    pub align: T,

    /// The bounds in which its contents will be aligned.
    /// See [`Bounds`]
    pub bounds: Bounds,

    /// See [`AllocateType`]
    pub allocate_type: AllocateType,

    /// The layout of the contents.
    /// If None, use the layout of the current ui.
    pub layout: Option<Layout>,
}

impl Default for Aligner<Align2> {
    fn default() -> Self {
        Self {
            id: None,
            align: egui::Align2::LEFT_TOP,
            bounds: Bounds::available_rect(),
            allocate_type: AllocateType::Content,
            layout: None,
        }
    }
}

impl Aligner<Align2> {
    #[inline]
    /// Create an `Alignable`
    /// which aligns its contents to the center of all the available space.
    pub fn center() -> Self {
        Self::from_align(Align2::CENTER_CENTER)
    }

    #[inline]
    /// Create an `Alignable`
    /// which aligns its contents to the center-bottom of all the available space.
    pub fn center_top() -> Self {
        Self::from_align(Align2::CENTER_TOP)
    }

    #[inline]
    /// Create an `Alignable`
    /// which aligns its contents to the center-bottom of all the available space.
    pub fn center_bottom() -> Self {
        Self::from_align(Align2::CENTER_BOTTOM)
    }

    #[inline]
    /// Create an `Alignable`
    /// which aligns its contents to the left of the available space.
    pub fn left() -> Self {
        Self::from_align(Align2::LEFT_CENTER)
    }

    #[inline]
    /// Create an `Alignable`
    /// which aligns its contents to the left-top of all the available space.
    pub fn left_top() -> Self {
        Self::from_align(Align2::LEFT_TOP)
    }

    #[inline]
    /// Create an `Alignable`
    /// which aligns its contents to the left-bottom of all the available space.
    pub fn left_bottom() -> Self {
        Self::from_align(Align2::LEFT_BOTTOM)
    }

    #[inline]
    /// Create an `Alignable`
    /// which aligns its contents to the right of the available space.
    pub fn right() -> Self {
        Self::from_align(Align2::RIGHT_CENTER)
    }

    #[inline]
    /// Create an `Alignable`
    /// which aligns its contents to the right-top of all the available space.
    pub fn right_top() -> Self {
        Self::from_align(Align2::RIGHT_TOP)
    }

    #[inline]
    /// Create an `Alignable`
    /// which aligns its contents to the right-bottom of all the available space.
    pub fn right_bottom() -> Self {
        Self::from_align(Align2::RIGHT_BOTTOM)
    }
}

impl<T: Alignment> Aligner<T> {
    /// Create an `Alignable`
    /// which aligns its contents using the given alignment.
    pub fn from_align(align: T) -> Self {
        Self {
            id: None,
            align,
            bounds: Bounds::AvailableRect(Vec2::INFINITY),
            allocate_type: AllocateType::Content,
            layout: None,
        }
    }
}

impl<T: Alignment> Aligner<T> {
    #[inline]
    /// Set the id of the aligned widget.
    /// The id is used to memorize the content size.
    /// If not set, the id will be generated automatically.
    pub fn id(mut self, id: Id) -> Self {
        self.id = Some(id);
        self
    }

    #[inline]
    /// Set the alignment.
    /// The alignment is used to align the contents.
    /// Could be a `egui::Align2`, a closure or a custom [`Alignment`].
    pub fn align(mut self, align: T) -> Self {
        self.align = align;
        self
    }

    #[inline]
    /// Set the space in which its contents will be aligned.
    pub fn bounds(mut self, bounds: Bounds) -> Self {
        self.bounds = bounds;
        self
    }

    #[inline]
    /// See [`AllocateType`]
    pub fn allocate_type(mut self, allocate_type: AllocateType) -> Self {
        self.allocate_type = allocate_type;
        self
    }

    #[inline]
    /// Set the layout of the contents.
    /// If not set, use the layout of the current ui.
    pub fn layout(mut self, layout: Layout) -> Self {
        self.layout = Some(layout);
        self
    }
}

impl<T: Alignment> Aligner<T> {
    /// Show the aligned contents.
    pub fn show<R>(
        self,
        ui: &mut Ui,
        add_contents: impl FnOnce(&mut egui::Ui) -> R,
    ) -> InnerResponse<R> {
        let id = self.id.unwrap_or_else(|| {
            let id = ui.next_auto_id();
            // hold the id
            ui.skip_ahead_auto_ids(1);
            id
        });

        let layout = self.layout.unwrap_or(*ui.layout());

        // calculate the bounds
        let bounds = match self.bounds {
            Bounds::AvailableRect(size) => {
                ui.new_child(UiBuilder::new())
                    .allocate_space(size.min(ui.available_size()))
                    .1
            }
            Bounds::MaxRect(margin) => ui.max_rect() - margin,
        };

        // try to read content size from context memory
        // if not found, use the whole available rect to draw the contents
        let mut memorized = true;
        let content_size = ui.ctx().data(|r| r.get_temp(id)).unwrap_or_else(|| {
            memorized = false;
            bounds.size()
        });

        // get the expected rect
        let expected_rect = self.align.align(content_size, bounds);
        // expand to allow content grow
        let content_rect = resize_layout_rect(expected_rect, bounds.size(), &layout);

        // create child ui
        let mut child_ui = ui.new_child({
            let builder = UiBuilder::new().max_rect(content_rect).layout(layout);

            if memorized {
                builder
            } else {
                // no size memorized, set the pass to sizing pass
                ui.ctx().request_discard("new Aligner");
                builder.sizing_pass().invisible()
            }
        });

        // paint the contents
        let inner = add_contents(&mut child_ui);

        // allocate space and get response
        // when already arranged, even if the content has grown, we allocate the expected size
        // if we allocate the whole new rect, it's actually in the wrong place and could disrupt the layout
        let content_rect = if memorized {
            expected_rect
        } else {
            content_rect
        };
        let response = ui.allocate_rect(
            match self.allocate_type {
                AllocateType::None => Rect::from_min_size(ui.next_widget_position(), Vec2::ZERO),
                AllocateType::Content => content_rect,
                AllocateType::ContentRow => {
                    let min = Pos2::new(bounds.left(), content_rect.top());
                    let max = Pos2::new(bounds.right(), content_rect.bottom());
                    Rect::from_min_max(min, max)
                }
                AllocateType::ContentColumn => {
                    let min = Pos2::new(content_rect.left(), bounds.top());
                    let max = Pos2::new(content_rect.right(), bounds.bottom());
                    Rect::from_min_max(min, max)
                }
                AllocateType::Bounds => bounds,
            },
            Sense::hover(),
        );

        // if the content changed size or not memorized, update the memorized size
        let new_size = child_ui.min_size();
        if new_size != content_size || !memorized {
            ui.ctx().data_mut(|w| w.insert_temp(id, new_size));
        }

        InnerResponse { inner, response }
    }

    #[inline]
    /// Show the contents horizontally.
    pub fn show_horizontal<R>(
        self,
        ui: &mut Ui,
        add_contents: impl FnOnce(&mut Ui) -> R,
    ) -> InnerResponse<R> {
        let layout = if ui.layout().prefer_right_to_left() {
            Layout::right_to_left(Align::Center)
        } else {
            Layout::left_to_right(Align::Center)
        }
        .with_main_wrap(false);

        self.layout(layout).show(ui, add_contents)
    }

    #[inline]
    /// Show the contents horizontally and wrap them when necessary.
    pub fn show_horizontal_wrapped<R>(
        self,
        ui: &mut Ui,
        add_contents: impl FnOnce(&mut Ui) -> R,
    ) -> InnerResponse<R> {
        let layout = if ui.layout().prefer_right_to_left() {
            Layout::right_to_left(Align::Center)
        } else {
            Layout::left_to_right(Align::Center)
        }
        .with_main_wrap(true);

        self.layout(layout).show(ui, add_contents)
    }

    #[inline]
    /// Show the contents vertically.
    pub fn show_vertical<R>(
        self,
        ui: &mut Ui,
        add_contents: impl FnOnce(&mut Ui) -> R,
    ) -> InnerResponse<R> {
        let layout = Layout::top_down(Align::Center);

        self.layout(layout).show(ui, add_contents)
    }
}

#[inline]
/// Center the contents horizontally.
pub fn center_horizontal<R>(
    ui: &mut Ui,
    add_contents: impl FnOnce(&mut Ui) -> R,
) -> InnerResponse<R> {
    let layout = if ui.layout().prefer_right_to_left() {
        Layout::right_to_left(Align::Center)
    } else {
        Layout::left_to_right(Align::Center)
    };

    Aligner::center().layout(layout).show(ui, add_contents)
}

#[inline]
/// Center the contents horizontally and wrap them when necessary.
pub fn center_horizontal_wrapped<R>(
    ui: &mut Ui,
    add_contents: impl FnOnce(&mut Ui) -> R,
) -> InnerResponse<R> {
    let layout = if ui.layout().prefer_right_to_left() {
        Layout::right_to_left(Align::Center)
    } else {
        Layout::left_to_right(Align::Center)
    }
    .with_main_wrap(true);

    Aligner::center().layout(layout).show(ui, add_contents)
}

#[inline]
/// Center the contents vertically.
pub fn center_vertical<R>(
    ui: &mut Ui,
    add_contents: impl FnOnce(&mut Ui) -> R,
) -> InnerResponse<R> {
    Aligner::center()
        .layout(Layout::top_down(Align::Center))
        .show(ui, add_contents)
}

#[inline]
/// Align the contents to the top horizontally.
pub fn top_horizontal<R>(ui: &mut Ui, add_contents: impl FnOnce(&mut Ui) -> R) -> InnerResponse<R> {
    let layout = if ui.layout().prefer_right_to_left() {
        Layout::right_to_left(Align::TOP)
    } else {
        Layout::left_to_right(Align::TOP)
    };

    Aligner::from_align(egui::Align2::CENTER_TOP)
        .layout(layout)
        .show(ui, add_contents)
}

#[inline]
/// Align the contents to the top horizontally and wrap them when necessary.
pub fn top_horizontal_wrapped<R>(
    ui: &mut Ui,
    add_contents: impl FnOnce(&mut Ui) -> R,
) -> InnerResponse<R> {
    let layout = if ui.layout().prefer_right_to_left() {
        Layout::right_to_left(Align::TOP)
    } else {
        Layout::left_to_right(Align::TOP)
    }
    .with_main_wrap(true);

    Aligner::from_align(Align2::CENTER_TOP)
        .layout(layout)
        .show(ui, add_contents)
}

#[inline]
/// Align the contents to the top vertically.
pub fn top_vertical<R>(ui: &mut Ui, add_contents: impl FnOnce(&mut Ui) -> R) -> InnerResponse<R> {
    ui.vertical_centered(add_contents)
}

#[inline]
/// Align the contents to the bottom horizontally.
pub fn bottom_horizontal<R>(
    ui: &mut Ui,
    add_contents: impl FnOnce(&mut Ui) -> R,
) -> InnerResponse<R> {
    let layout = if ui.layout().prefer_right_to_left() {
        Layout::right_to_left(Align::BOTTOM)
    } else {
        Layout::left_to_right(Align::BOTTOM)
    };

    Aligner::from_align(egui::Align2::CENTER_BOTTOM)
        .layout(layout)
        .show(ui, add_contents)
}

#[inline]
/// Align the contents to the bottom horizontally and wrap them when necessary.
pub fn bottom_horizontal_wrapped<R>(
    ui: &mut Ui,
    add_contents: impl FnOnce(&mut Ui) -> R,
) -> InnerResponse<R> {
    let layout = if ui.layout().prefer_right_to_left() {
        Layout::right_to_left(Align::BOTTOM)
    } else {
        Layout::left_to_right(Align::BOTTOM)
    }
    .with_main_wrap(true);

    Aligner::from_align(egui::Align2::CENTER_BOTTOM)
        .layout(layout)
        .show(ui, add_contents)
}

#[inline]
/// Align the contents to the bottom vertically.
pub fn bottom_vertical<R>(
    ui: &mut Ui,
    add_contents: impl FnOnce(&mut Ui) -> R,
) -> InnerResponse<R> {
    ui.with_layout(Layout::bottom_up(Align::Center), add_contents)
}

#[inline]
/// Align the contents to the left horizontally.
pub fn left_horizontal<R>(
    ui: &mut Ui,
    add_contents: impl FnOnce(&mut Ui) -> R,
) -> InnerResponse<R> {
    ui.horizontal_centered(add_contents)
}

#[inline]
/// Align the contents to the left horizontally and wrap them when necessary.
pub fn left_horizontal_wrapped<R>(
    ui: &mut Ui,
    add_contents: impl FnOnce(&mut Ui) -> R,
) -> InnerResponse<R> {
    let layout = Layout::left_to_right(Align::Center).with_main_wrap(true);

    Aligner::from_align(egui::Align2::LEFT_CENTER)
        .layout(layout)
        .show(ui, add_contents)
}

#[inline]
/// Align the contents to the left vertically.
pub fn left_vertical<R>(ui: &mut Ui, add_contents: impl FnOnce(&mut Ui) -> R) -> InnerResponse<R> {
    Aligner::from_align(egui::Align2::LEFT_CENTER)
        .layout(Layout::top_down(Align::Min))
        .show(ui, add_contents)
}

#[inline]
/// Align the contents to the right horizontally.
pub fn right_horizontal<R>(
    ui: &mut Ui,
    add_contents: impl FnOnce(&mut Ui) -> R,
) -> InnerResponse<R> {
    ui.with_layout(Layout::right_to_left(Align::Center), add_contents)
}

#[inline]
/// Align the contents to the right horizontally and wrap them when necessary.
pub fn right_horizontal_wrapped<R>(
    ui: &mut Ui,
    add_contents: impl FnOnce(&mut Ui) -> R,
) -> InnerResponse<R> {
    let layout = Layout::right_to_left(Align::Center).with_main_wrap(true);
    Aligner::from_align(Align2::RIGHT_CENTER)
        .layout(layout)
        .show(ui, add_contents)
}

#[inline]
/// Align the contents to the right vertically.
pub fn right_vertical<R>(ui: &mut Ui, add_contents: impl FnOnce(&mut Ui) -> R) -> InnerResponse<R> {
    Aligner::from_align(Align2::RIGHT_CENTER)
        .layout(Layout::top_down(Align::Max))
        .show(ui, add_contents)
}
