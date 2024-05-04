// main.rs

mod window;
// mod text_label;

use crate::window::window::AppWindow;

// use text_label::update_label;

fn main() {
    let app_window = AppWindow::new();
    app_window.run();

    // Assuming you have access to the window instance here
    // Update the label text
    // update_label(&app_window.window, "New text");
}

