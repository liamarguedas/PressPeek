
mod window;

use crate::window::window::MyWindow;


// fn main() {
//     let app_window = AppWindow::new();
//     app_window.run();
//
//     // Assuming you have access to the window instance here
//     // Update the label text
//     // update_label(&app_window.window, "New text");
// }
//

use egui::{Ui, Label, RichText};

fn main() {
    // Initialize your UI context (e.g., egui::Context).

    // Create a label:
    let label_text = "Hello, World!";
    Ui.label(label_text); // Simple label
    Ui.add(Label::new(label_text)); // Equivalent label

    // You can also add formatting to the label:
    Ui.add(Label::new("With Options").wrap(false)); // Label without word wrapping
    Ui.label(RichText::new("With formatting").underline()); // Label with formatting
}
