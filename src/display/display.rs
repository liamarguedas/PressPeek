use egui::CtxRef;
use egui_glium::GliumPainter;
use egui_glium::glium::Display;

pub fn start_display(receiver: mpsc::Receiver<String>, ctx: &mut egui::Context) {
    let display = Display::from_glium(egui_glium::backend::create_display());

    let painter = GliumPainter::new(&display).unwrap();
    let mut last_frame = std::time::Instant::now();

    egui_glium::run(
        &display,
        ctx.clone(),
        move |ui: &mut egui::Ui, painter| {
            // Process incoming messages
            if let Ok(keys) = receiver.try_recv() {
                // Handle new keys
            }

            // Update UI
            display_ui(ui, ctx);

            // Paint UI
            painter.paint(ctx.texture(), ui);

            // Close window if requested
            if ui.input().raw.ended {
                *ui.quit = true;
            }
        },
        painter,
    );
}

fn display_ui(ui: &mut egui::Ui, ctx: &CtxRef) {
    egui::Window::new("Key Listener")
        .resizable(true)
        .scroll(false)
        .collapsible(true)
        .show(ui.ctx(), |ui| {
            // Display UI created in display.rs
            display::create_ui(ui);
        });
}


