use std::collections::HashMap;

use winit::event::{VirtualKeyCode, Event, WindowEvent, KeyboardInput, ElementState, MouseButton};
use winit::event_loop::EventLoop;
use winit::window::WindowBuilder;
use winit::{event_loop::EventLoopBuilder, window::Window};

use crate::engine::draw::Renderer;

use super::draw::Point;

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
                Event::WindowEvent { event, .. } => {
                    match event {
                        WindowEvent::Resized(new_size) => {
                            self.renderer.resize(new_size);
                        },
                        WindowEvent::CloseRequested => {
                            control_flow.set_exit();
                        },
                        WindowEvent::CursorMoved { position, .. } => {
                            self.input_handler.cursor_position = [
                                position.x / (self.renderer.size.width  as f64 / 2.) - 1., 
                                -1. * position.y / (self.renderer.size.height as f64 / 2.) + 1.,
                                ].into();
                        },
                        WindowEvent::KeyboardInput { input: KeyboardInput{ state: key_state, virtual_keycode: Some(event_key), .. }, .. } => { 
                            for (callback_key, callback) in self.input_handler.key_event_callbacks.iter() {
                                if callback_key == &event_key {
                                    callback(key_state);
                                }
                            }
                        }
                        _ => ()
                    }
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
    key_event_callbacks: HashMap<VirtualKeyCode, Box<dyn Fn(ElementState) -> ()>>,
    mouse_click_event_callbacks: HashMap<MouseButton, Box<dyn Fn(ElementState) -> ()>>,
    pub cursor_position: Point,
}
impl InputHandler {
    fn new() -> Self {
        InputHandler { 
            key_event_callbacks: HashMap::new(), 
            mouse_click_event_callbacks: HashMap::new(), 
            cursor_position: Point::ZERO,
        }
    }
    fn add_key_event_callback<F: Fn(ElementState) -> () + 'static>(&mut self, key: VirtualKeyCode, callback: F) {
        self.key_event_callbacks.insert(key, Box::new(callback));
    }
    fn add_mouse_click_event_callback<F: Fn(ElementState) -> () + 'static>(&mut self, key: MouseButton, bounds: [Point;2], callback: F) {
        
    }
}

// ----- TESTS -----
#[cfg(test)]
mod tests {
    use super::*;  
    
    use winit::platform::wayland::EventLoopBuilderExtWayland;

    #[test]
    fn test_windowhandler() {
        async fn run() {
            let window_handler = WindowHandler::from_builders(
                WindowBuilder::default(),
                EventLoopBuilder::default().with_any_thread(true),
            ).await;
            window_handler.main_loop(|renderer, _| {
                renderer.render().unwrap();
            });
        }
        pollster::block_on(run())
    }

    #[test]
    fn test_windowhandler_inputhandler_key_event() {
        async fn run() {
            let window_handler = WindowHandler::from_builders(
                WindowBuilder::default(),
                EventLoopBuilder::default().with_any_thread(true),
            ).await;
            window_handler.main_loop(|renderer, input_handler| {
                input_handler.add_key_event_callback(VirtualKeyCode::Space, |key_state| {
                    if key_state == ElementState::Pressed {
                        println!("pressed space")
                    }
                });
                renderer.render().unwrap();
            });
        }
        pollster::block_on(run())
    }

    #[test]
    fn test_windowhandler_inputhandler_cursor_position() {
        async fn run() {
            let window_handler = WindowHandler::from_builders(
                WindowBuilder::default(),
                EventLoopBuilder::default().with_any_thread(true),
            ).await;
            window_handler.main_loop(|renderer, input_handler| {
                println!("{:?}", input_handler.cursor_position);
                renderer.render().unwrap();
            });
        }
        pollster::block_on(run())
    }
}