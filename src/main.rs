
mod keyboard;
mod display;

use std::sync::mpsc;
use egui::{Context, Window};

fn main() {
    // Create a channel for communication between threads
    let (sender, receiver) = mpsc::channel();

    // Start the keyboard listener in a separate thread
    std::thread::spawn(move || {
        let keys = keyboard::keyboard::listen_keyboard(5); // Adjust the array size as needed
        sender.send(keys).unwrap();
    });

    // Create a new egui context
    let mut ctx = Context::new();

    printf("test.rs");

    // Start the display
    display::display::start_display(receiver, &mut ctx);
}

