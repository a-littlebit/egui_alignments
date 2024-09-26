use egui::{Align, Align2, Id, InnerResponse, Layout, Margin, Pos2, Rect, Sense, Ui, Vec2};

/// Represents an alignment strategy.
/// You can directly use `egui::Align2` or closure `FnOnce(egui::Vec2, egui::Rect) -> egui::Rect`
/// to align the contents.
/// Or you can implement your own aligner.
pub trait Aligner {
    fn align(self, item_size: Vec2, available_rect: Rect) -> Rect;
}

impl Aligner for egui::Align2 {
    fn align(self, item_size: Vec2, available_rect: Rect) -> Rect {
        self.align_size_within_rect(item_size, available_rect)
    }
}

impl<T> Aligner for T
where T: FnOnce(Vec2, Rect) -> Rect {
    fn align(self, item_size: Vec2, available_rect: Rect) -> Rect {
        self(item_size, available_rect)
    }
}

/// Determines how [`WidgetAligner`] allocate space for the aligned contents.
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

    /// Allocate the whole space specified to align the contents.
    AlignSpace,
}

/// The space in which its contents will be aligned.
pub enum AlignSpace {
    /// Align in Ui's next widget position with the given size.
    AvailableRect(Vec2),

    /// Align in the whole Ui.
    MaxRect,
}

/// A container which aligns its contents
/// within the given aligner and space.
/// 
/// # Example
/// ```
/// use egui_alignments::WidgetAligner;
/// 
/// # egui::__run_test_ui(|ui| {
/// WidgetAligner::center()
///     .show(ui, |ui| {
///         ui.label("This label will be shown at the center");
///     });
/// # });
/// ```
pub struct WidgetAligner<T: Aligner> {
    /// Used to memorize content size.
    /// If not set, the id will be generated automatically.
    pub id: Option<Id>,

    /// The aligner.
    /// Could be a `egui::Align2`, a closure or a custom aligner.
    pub align: T,

    /// The space in which its contents will be aligned.
    /// See [`AlignSpace`]
    pub align_space: AlignSpace,

    /// The reserved space.
    /// Space outside ([`WidgetAligner::align_space`] - [`WidgetAligner::reserved_space`] )
    /// will be reserved.
    pub reserved_space: Margin,

    /// See [`AllocateType`]
    pub allocate_type: AllocateType,

    /// The layout of the contents.
    /// If None, use the layout of the current ui.
    pub layout: Option<Layout>,
}

pub type Align2WidgetAligner = WidgetAligner<egui::Align2>;

impl Default for Align2WidgetAligner {
    fn default() -> Self {
        Self {
            id: None,
            align: egui::Align2::LEFT_TOP,
            align_space: AlignSpace::AvailableRect(Vec2::INFINITY),
            reserved_space: Margin::ZERO,
            allocate_type: AllocateType::Content,
            layout: None,
        }
    }
}

impl Align2WidgetAligner {
    #[inline]
    /// Create an `AlignedWidget`
    /// which aligns its contents to the center of all the available space.
    pub fn center() -> Self {
        Self::from_align(Align2::CENTER_CENTER)
    }

    #[inline]
    /// Create an `AlignedWidget`
    /// which aligns its contents to the center-bottom of all the available space.
    pub fn center_top() -> Self {
        Self::from_align(Align2::CENTER_TOP)
    }

    #[inline]
    /// Create an `AlignedWidget`
    /// which aligns its contents to the center-bottom of all the available space.
    pub fn center_bottom() -> Self {
        Self::from_align(Align2::CENTER_BOTTOM)
    }

    #[inline]
    /// Create an `AlignedWidget`
    /// which aligns its contents to the left of the available space.
    pub fn left() -> Self {
        Self::from_align(Align2::LEFT_CENTER)
    }

    #[inline]
    /// Create an `AlignedWidget`
    /// which aligns its contents to the left-top of all the available space.
    pub fn left_top() -> Self {
        Self::from_align(Align2::LEFT_TOP)
    }

    #[inline]
    /// Create an `AlignedWidget`
    /// which aligns its contents to the left-bottom of all the available space.
    pub fn left_bottom() -> Self {
        Self::from_align(Align2::LEFT_BOTTOM)
    }

    #[inline]
    /// Create an `AlignedWidget`
    /// which aligns its contents to the right of the available space.
    pub fn right() -> Self {
        Self::from_align(Align2::RIGHT_CENTER)
    }

    #[inline]
    /// Create an `AlignedWidget`
    /// which aligns its contents to the right-top of all the available space.
    pub fn right_top() -> Self {
        Self::from_align(Align2::RIGHT_TOP)
    }

    #[inline]
    /// Create an `AlignedWidget`
    /// which aligns its contents to the right-bottom of all the available space.
    pub fn right_bottom() -> Self {
        Self::from_align(Align2::RIGHT_BOTTOM)
    }
}

impl<T: Aligner> WidgetAligner<T> {
    /// Create an `AlignedWidget`
    /// which aligns its contents using the given aligner.
    pub fn from_align(align: T) -> Self {
        Self {
            id: None,
            align,
            align_space: AlignSpace::AvailableRect(Vec2::INFINITY),
            reserved_space: Margin::ZERO,
            allocate_type: AllocateType::Content,
            layout: None,
        }
    }
}

impl<T: Aligner> WidgetAligner<T> {
    #[inline]
    /// Set the id of the aligned widget.
    /// The id is used to memorize the content size.
    /// If not set, the id will be generated automatically.
    pub fn id(mut self, id: Id) -> Self {
        self.id = Some(id);
        self
    }

    #[inline]
    /// Set the aligner.
    /// The aligner is used to align the contents.
    /// Could be a `egui::Align2`, a closure or a custom aligner.
    pub fn align(mut self, align: T) -> Self {
        self.align = align;
        self
    }

    #[inline]
    /// Set the space in which its contents will be aligned.
    pub fn align_space(mut self, align_space: AlignSpace) -> Self {
        self.align_space = align_space;
        self
    }

    #[inline]
    /// Set the reserved space.
    /// Space outside ([`WidgetAligner::align_space`] - [`WidgetAligner::reserved_space`] )
    /// will be reserved.
    pub fn reserved_space(mut self, reserved_space: Margin) -> Self {
        self.reserved_space = reserved_space;
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

impl<T: Aligner> WidgetAligner<T> {
    /// Show the aligned contents.
    pub fn show<R>(self, ui: &mut Ui, add_contents: impl FnOnce(&mut egui::Ui) -> R) -> InnerResponse<R> {
        let id = self.id.unwrap_or_else(|| {
            let id = ui.next_auto_id();
            // hold the id
            ui.skip_ahead_auto_ids(1);
            id
        });

        let layout = self.layout.unwrap_or(*ui.layout());

        // calculate available rect for alignment
        let align_rect = match self.align_space {
            AlignSpace::AvailableRect(align_size) => {
                let mut align_rect = ui.available_rect_before_wrap() - self.reserved_space;
                align_rect = align_rect.intersect(
                    layout.align_size_within_rect(align_size, align_rect)
                );
                align_rect
            },
            AlignSpace::MaxRect => {
                ui.max_rect() - self.reserved_space
            }
        };

        // try to read content size from context memory
        // if not found, use the whole available rect to draw the contents
        let mut memorized = true;
        let content_size = ui.ctx()
            .data(|r| r.get_temp(id))
            .unwrap_or_else(|| {
                memorized = false;
                align_rect.size()
            });

        // align within available rect
        let mut content_rect = self.align.align(content_size, align_rect);
        // extend child ui to allow contents to become larger than the memorized
        if layout.horizontal_placement() == Align::Max {
            content_rect.min.x = align_rect.min.x;
        } else {
            content_rect.max.x = align_rect.max.x;
        }
        if layout.vertical_align() == Align::Max {
            content_rect.min.y = align_rect.min.y;
        } else {
            content_rect.max.y = align_rect.max.y;
        }
        // create child ui
        let mut child_ui = ui.child_ui(
            content_rect, 
            layout, 
            None
        );

        // hide the child ui if we didn't memorize the size
        if !memorized {
            child_ui.set_invisible();
        }

        // paint the contents
        let inner = add_contents(&mut child_ui);
        
        // hold the content place
        let response = ui.allocate_rect(
            match self.allocate_type {
                AllocateType::None => Rect::from_min_size(ui.next_widget_position(), Vec2::ZERO),
                AllocateType::Content => child_ui.min_rect(),
                AllocateType::ContentRow => {
                    let content_rect = child_ui.min_rect();
                    let min = Pos2::new(align_rect.left(), content_rect.top());
                    let max = Pos2::new(align_rect.right(), content_rect.bottom());
                    Rect::from_min_max(min, max)
                },
                AllocateType::ContentColumn => {
                    let content_rect = child_ui.min_rect();
                    let min = Pos2::new(content_rect.left(), align_rect.top());
                    let max = Pos2::new(content_rect.right(), align_rect.bottom());
                    Rect::from_min_max(min, max)
                },
                AllocateType::AlignSpace => align_rect,
            },
            Sense::hover(),
        );

        // if the content changed size or was hidden, update the memorized size request a repaint
        let new_size = child_ui.min_size();
        if new_size != content_size || !memorized {
            ui.ctx().data_mut(|w| w.insert_temp(id, new_size));
            ui.ctx().request_repaint();
        }

        InnerResponse { inner, response }
    }
}

#[inline]
/// Center the contents horizontally.
pub fn center_horizontal<R>(
    ui: &mut Ui,
    add_contents: impl FnOnce(&mut Ui) -> R
) -> InnerResponse<R> {
    let layout = if ui.layout().prefer_right_to_left() {
        Layout::right_to_left(Align::Center)
    } else {
        Layout::left_to_right(Align::Center)
    };

    WidgetAligner::center()
        .align_space(AlignSpace::MaxRect)
        .allocate_type(AllocateType::None)
        .layout(layout)
        .show(ui, add_contents)
}

#[inline]
/// Center the contents horizontally and wrap them when necessary.
pub fn center_horizontal_wrapped<R>(
    ui: &mut Ui,
    add_contents: impl FnOnce(&mut Ui) -> R
) -> InnerResponse<R> {
    let layout = if ui.layout().prefer_right_to_left() {
        Layout::right_to_left(Align::TOP)
    } else {
        Layout::left_to_right(Align::TOP)
    }
    .with_main_wrap(true);
    WidgetAligner::center()
        .align_space(AlignSpace::MaxRect)
        .allocate_type(AllocateType::None)
        .layout(layout)
        .show(ui, add_contents)
}

#[inline]
/// Center the contents vertically.
pub fn center_vertical<R>(
    ui: &mut Ui,
    add_contents: impl FnOnce(&mut Ui) -> R
) -> InnerResponse<R> {
    WidgetAligner::center()
        .align_space(AlignSpace::MaxRect)
        .allocate_type(AllocateType::None)
        .layout(Layout::top_down(Align::Center))
        .show(ui, add_contents)
}

#[inline]
/// Align the contents to the top horizontally.
pub fn top_horizontal<R>(
    ui: &mut Ui,
    add_contents: impl FnOnce(&mut Ui) -> R
) -> InnerResponse<R> {
    let layout = if ui.layout().prefer_right_to_left() {
        Layout::right_to_left(Align::TOP)
    } else {
        Layout::left_to_right(Align::TOP)
    };

    WidgetAligner::from_align(egui::Align2::CENTER_TOP)
        .allocate_type(
            if ui.layout().vertical_align() == Align::Min {
                AllocateType::ContentRow
            } else {
                AllocateType::None
            }
        )
        .layout(layout)
        .show(ui, add_contents)
}

#[inline]
/// Align the contents to the top horizontally and wrap them when necessary.
pub fn top_horizontal_wrapped<R>(
    ui: &mut Ui,
    add_contents: impl FnOnce(&mut Ui) -> R
) -> InnerResponse<R> {
    let layout = if ui.layout().prefer_right_to_left() {
        Layout::right_to_left(Align::TOP)
    } else {
        Layout::left_to_right(Align::TOP)
    }
    .with_main_wrap(true);
    WidgetAligner::from_align(Align2::CENTER_TOP)
        .allocate_type(
            if ui.layout().vertical_align() == Align::Min {
                AllocateType::ContentRow
            } else {
                AllocateType::None
            }
        )
        .layout(layout)
        .show(ui, add_contents)
}

#[inline]
/// Align the contents to the top vertically.
pub fn top_vertical<R>(
    ui: &mut Ui,
    add_contents: impl FnOnce(&mut Ui) -> R
) -> InnerResponse<R> {
    ui.vertical_centered(add_contents)
}

#[inline]
/// Align the contents to the bottom horizontally.
pub fn bottom_horizontal<R>(
    ui: &mut Ui,
    add_contents: impl FnOnce(&mut Ui) -> R
) -> InnerResponse<R> {
    let layout = if ui.layout().prefer_right_to_left() {
        Layout::right_to_left(Align::BOTTOM)
    } else {
        Layout::left_to_right(Align::BOTTOM)
    };
    WidgetAligner::from_align(egui::Align2::CENTER_BOTTOM)
        .allocate_type(
            if ui.layout().vertical_align() == Align::Max {
                AllocateType::ContentRow
            } else {
                AllocateType::None
            }
        )
        .layout(layout)
        .show(ui, add_contents)
}

#[inline]
/// Align the contents to the bottom horizontally and wrap them when necessary.
pub fn bottom_horizontal_wrapped<R>(
    ui: &mut Ui,
    add_contents: impl FnOnce(&mut Ui) -> R
) -> InnerResponse<R> {
    let layout = if ui.layout().prefer_right_to_left() {
        Layout::right_to_left(Align::BOTTOM)
    } else {
        Layout::left_to_right(Align::BOTTOM)
    }
    .with_main_wrap(true);
    WidgetAligner::from_align(egui::Align2::CENTER_BOTTOM)
        .allocate_type(
            if ui.layout().vertical_align() == Align::Max {
                AllocateType::ContentRow
            } else {
                AllocateType::None
            }
        )
        .layout(layout)
        .show(ui, add_contents)
}

#[inline]
/// Align the contents to the bottom vertically.
pub fn bottom_vertical<R>(
    ui: &mut Ui,
    add_contents: impl FnOnce(&mut Ui) -> R
) -> InnerResponse<R> {
    ui.with_layout(Layout::bottom_up(Align::Center), add_contents)
}

#[inline]
/// Align the contents to the left horizontally.
pub fn left_horizontal<R>(
    ui: &mut Ui,
    add_contents: impl FnOnce(&mut Ui) -> R
) -> InnerResponse<R> {
    ui.horizontal_centered(add_contents)
}

#[inline]
/// Align the contents to the left horizontally and wrap them when necessary.
pub fn left_horizontal_wrapped<R>(
    ui: &mut Ui,
    add_contents: impl FnOnce(&mut Ui) -> R
) -> InnerResponse<R> {
    let layout = Layout::left_to_right(Align::TOP)
        .with_main_wrap(true);
    WidgetAligner::from_align(egui::Align2::LEFT_CENTER)
        .allocate_type(
            if ui.layout().horizontal_align() == Align::Min {
                AllocateType::ContentColumn
            } else {
                AllocateType::None
            }
        )
        .layout(layout)
        .show(ui, add_contents)
}

#[inline]
/// Align the contents to the left vertically.
pub fn left_vertical<R>(
    ui: &mut Ui,
    add_contents: impl FnOnce(&mut Ui) -> R
) -> InnerResponse<R> {
    WidgetAligner::from_align(egui::Align2::LEFT_CENTER)
        .allocate_type(
            if ui.layout().horizontal_align() == Align::Min {
                AllocateType::ContentColumn
            } else {
                AllocateType::None
            }
        )
        .layout(Layout::top_down(Align::Min))
        .show(ui, add_contents)
}

#[inline]
/// Align the contents to the right horizontally.
pub fn right_horizontal<R>(
    ui: &mut Ui,
    add_contents: impl FnOnce(&mut Ui) -> R
) -> InnerResponse<R> {
    ui.with_layout(Layout::right_to_left(Align::Center), add_contents)
}

#[inline]
/// Align the contents to the right horizontally and wrap them when necessary.
pub fn right_horizontal_wrapped<R>(
    ui: &mut Ui,
    add_contents: impl FnOnce(&mut Ui) -> R
) -> InnerResponse<R> {
    let layout = Layout::right_to_left(Align::TOP)
        .with_main_wrap(true);
    WidgetAligner::from_align(Align2::RIGHT_CENTER)
        .allocate_type(
            if ui.layout().horizontal_align() == Align::Max {
                AllocateType::ContentColumn
            } else {
                AllocateType::None
            }
        )
        .layout(layout)
        .show(ui, add_contents)
}

#[inline]
/// Align the contents to the right vertically.
pub fn right_vertical<R>(
    ui: &mut Ui,
    add_contents: impl FnOnce(&mut Ui) -> R
) -> InnerResponse<R> {
    WidgetAligner::from_align(Align2::RIGHT_CENTER)
        .allocate_type(
            if ui.layout().horizontal_align() == Align::Max {
                AllocateType::ContentColumn
            } else {
                AllocateType::None
            }
        )
        .layout(Layout::top_down(Align::Max))
        .show(ui, add_contents)
}