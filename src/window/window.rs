
use winit::{
    event_loop::EventLoop,
    window::{Window, WindowBuilder},
};


    pub fn create_window(){
    let event_loop = EventLoop::new();
    let window = WindowBuilder::new().build(&event_loop).unwrap();

    event_loop.run(move |event, _, control_flow| {
        *control_flow = winit::event_loop::ControlFlow::Wait;


        match even {
            winit::event::Event::WindowEvent {
                event: winit::event::WindowEvent::CloseRequested,
                ..
                } => *control_flow = winint::event_loop::ControlFlow::Exit,
                _ => (),
        }
    })
}
