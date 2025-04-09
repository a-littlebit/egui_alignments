# egui_alignments

Simple alignment tools for [egui](https://github.com/emilk/egui)

## Example Usage

### Align a single widget

```rust
use egui::{Button, Label};
use egui_alignments::Alignable;

Label::new("This label will be shown at the top")
    .top(ui);
  
if Button::new("This label will be shown at the center")
    .center(ui)
    .clicked()
{
    println!("Center button clicked!");
}
    
Label::new("This label will be shown at the bottom")
    .bottom(ui);
```

### Align multiple widgets

The following buttons will be shown at the center of the screen horizontally
with the tip text above and click results below.

```rust
use egui::{Button, Widget};
use egui_alignments::center_horizontal;

let mut clicked_button = None;

center_vertical(ui, |ui| {
    ui.label("Click a button");
    
    ui.add_space(20.0);
    
    center_horizontal(ui, |ui| {
        for i in 1..=10 {
            if Button::new(format!("Button {}", i))
                .ui(ui)
                .clicked()
            {
                clicked_button = Some(i);
            }
        }
    });
    
    ui.add_space(20.0);

    if let Some(i) = clicked_button {
        ui.label(format!("You clicked button {}", i));
    }
})
```

### Use containers

Sometimes nested calls to alignment functions like `center_horizontal`, `top_vertical`, ...
may cause layout confusion due to interaction between inner and outer layouts.

To prevent inner layouts from disrupting the outer ones, you may use containers for inner layouts.

Containers only cares about its inner alignments and act like a simple widget in the outer layout.

The following is an example usage of containers.

```rust
use egui::Align;
use egui_alignments::{center_horizontal, column, row};

center_horizontal(ui, |ui| {
    ui.image("path/to/left/image");
    column(ui, Align::Center, |ui| {
        ui.label("top of text right to image");
        row(ui, Align::Center, |ui| {
            ui.label("left");
            ui.label("middle");
            ui.label("right");
        });
        ui.label("bottom of text right to image");
    });
});
```

This will show an image on the left, and a column of text on the right which contains a row of three labels in the middle.

### Use stretches in containers

Sometimes you may not want the elements in a container to be closely aligned.
You may use `stretch` to make the elements in a container stretch to fill the available space.

```rust
use egui::Align;
use egui_alignments::{center_horizontal, column, row};

center_horizontal(ui, |ui| {
    ui.image("path/to/left/image");
    column(ui, Align::Center, |ui| {
        ui.label("top of text right to image");
        stretch(ui);
        row(ui, Align::Center, |ui| {
            ui.label("left");
            stretch(ui);
            ui.label("middle");
            stretch(ui);
            ui.label("right");
        });
        stretch(ui);
        ui.label("bottom of text right to image");
    });
});
```

This will make the text elements right to the image aligned as far as possible.

If you want the stretches in a container to have different weights, you may use `stretch_with_weight` instead.

```rust
use egui::Align;
use egui_alignments::{center_horizontal, column, row};

center_horizontal(ui, |ui| {
    stretch_with_weight(ui, 2.0);
    ui.image("path/to/left/image");
    stretch_with_weight(ui, 1.0);
    column(ui, Align::Center, |ui| {
        ui.label("top of text right to image");
        row(ui, Align::Center, |ui| {
            ui.label("left");
            ui.label("middle");
            ui.label("right");
        });
        ui.label("bottom of text right to image");
    });
    stretch_with_weight(ui, 2.0);
});
```

This will make the space left to the image and right to all the text elements
to be twice as large as the gap between the image and the text.
