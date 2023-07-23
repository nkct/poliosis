use winit::event::{VirtualKeyCode, Event, WindowEvent};
use winit::event_loop::EventLoop;
use winit::window::WindowBuilder;
use winit::{event_loop::EventLoopBuilder, window::Window};

use crate::engine::draw::Renderer;

struct WindowHandler {
    window: Window,
    event_loop: EventLoop<()>,
    renderer: Renderer,
    input_handler: InputHandler,
}
impl WindowHandler {
    async fn new() -> Self {
        let event_loop = EventLoop::new();
        let window = Window::new(&event_loop).unwrap();
        let renderer = Renderer::new(&window).await;
        let input_handler = InputHandler::new();

        WindowHandler { 
            window, 
            event_loop,
            renderer, 
            input_handler,
        }
    }

    async fn from_builders(window_builder: WindowBuilder, event_loop_builder: &mut EventLoopBuilder<()>) -> Self {
        let event_loop = event_loop_builder.build();
        let window = window_builder.build(&event_loop).unwrap();
        let renderer = Renderer::new(&window).await;
        let input_handler = InputHandler::new();

        WindowHandler { 
            window, 
            event_loop,
            renderer, 
            input_handler,
        }
    }

    fn main_loop<F: FnMut(&mut Renderer, &mut InputHandler) -> () + 'static>(mut self, mut f: F) {   
        self.event_loop.run(move |event, _, control_flow| {
            match event {
                Event::WindowEvent {
                    event: WindowEvent::CloseRequested,
                    ..
                } => {
                    control_flow.set_exit();
                },
                Event::MainEventsCleared => {
                    f(&mut self.renderer, &mut self.input_handler)
                },
                _ => ()
            }
        });
    }
}
struct InputHandler {
    keyboard_callbacks: Vec<Box<dyn FnOnce(VirtualKeyCode) -> ()>>
}
impl InputHandler {
    fn new() -> Self {
        InputHandler { 
            keyboard_callbacks: Vec::new(), 
        }
    }
}

// ----- TESTS -----
#[cfg(test)]
mod tests {
    use crate::engine::draw::Color;

    use super::*;  
    
    use winit::platform::wayland::EventLoopBuilderExtWayland;

    #[test]
    fn test_windowhandler() {
        async fn run() {
            let window_handler = WindowHandler::from_builders(
                WindowBuilder::default(),
                EventLoopBuilder::default().with_any_thread(true),
            ).await;
            window_handler.main_loop(|renderer, input_handler| {
                renderer.draw_triangle([[0., 0.5], [-0.5, -0.5], [0.5, -0.5]], Color::RED);
                renderer.render().unwrap();
            });
        }
        pollster::block_on(run())
    }
}