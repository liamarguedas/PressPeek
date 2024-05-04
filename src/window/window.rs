use egui::{CentralPanel, Label, Ui, Window};

pub struct MyWindow {
    label_text: String,
}

impl MyWindow {
   pub fn new() -> Self {
        Self {
            label_text: String::from("Hello, World!"),
        }
    }

    fn update_label(&mut self, new_text: &str) {
        self.label_text = String::from(new_text);
    }

    fn draw_ui(&mut self, ui: &mut Ui) {
        CentralPanel::default().show(ui, |ui| {
            ui.label(self.label_text.clone());
        });
    }
}
