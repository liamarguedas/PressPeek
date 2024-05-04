use winit::{
    event_loop::EventLoop,
    window:: {WindowBuilder, Window},
};

pub struct AppWindow {
    window: Window
}

impl AppWindow{

    pub fn new() -> Self {
        let with: f32 = 250.0;
        let height: f32 = 50.0;
        let event_loop = EventLoop::new();

        let window: Window = WindowBuilder::new()
            .with_title("")
            .with_inner_size(winit::dpi::LogicalSize::new(with, height))
            .with_decorations(true)
            .with_always_on_top(true)
            .build(&event_loop)
            .unwrap();
        Self { window }
    }
    pub fn run(self) {
        let window = self.window;
        
        window.run(move |event, _, control_flow| {
            *control_flow = winit::event_loop::ControlFlow::Wait;

            match event {
                winit::event::Event::WindowEvent { event, .. } => match event {
                    winit::event::WindowEvent::CloseRequested => *control_flow = winit::event_loop::ControlFlow::Exit,
                    _ => (),
                },
                _ => (),
            }
        });
    }
}

