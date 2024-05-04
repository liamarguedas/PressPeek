use winit::{
    event_loop::EventLoop,
    window:: {WindowBuilder, Window},
};

pub fn presspeek_window() {
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

    event_loop.run(move |event, _, control_flow| {
        *control_flow = winit::event_loop::ControlFlow::Wait;

        match event {
            winit::event::Event::WindowEvent {
                event: winit::event::WindowEvent::CloseRequested,
                ..
            } => *control_flow = winit::event_loop::ControlFlow::Exit,
            _ => (),
        }
    });
}

