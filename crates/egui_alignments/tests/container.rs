use egui::{Align, Rect};
use egui_alignments::{stretch, Row};
use egui_kittest::kittest::Queryable;
use egui_kittest::Harness;
use std::cell::Cell;

#[test]
fn container_stretch_rounding() {
    let container_rect = Cell::new(Rect::NAN);

    let mut harness = Harness::builder().with_size([750.0, 60.0]).build_ui(|ui| {
        let res = Row::new(Align::Min).wrapping(true).show(ui, |ui| {
            for i in 0..=8 {
                if i == 4 || i == 6 {
                    stretch(ui);
                }
                let _ = ui.button(format!("Button {i}"));
            }
        });
        container_rect.set(res.response.rect);
    });

    // Run several times to check the layout is stable across frames
    for _ in 0..4 {
        // Check all buttons are on the same line
        let buttons = harness.get_all_by_label_contains("Button");
        assert!(buttons.is_sorted_by(|l, r| l.rect().top() == r.rect().top()));

        // Check that the layout is stretched across the full row
        let button = harness.get_by_label("Button 8");
        assert_eq!(button.rect().right(), container_rect.get().right());

        harness.step();
    }
}
