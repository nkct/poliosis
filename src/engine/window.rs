use std::collections::HashMap;

use winit::event::{VirtualKeyCode, Event, WindowEvent, KeyboardInput, ElementState, MouseButton};
use winit::event_loop::EventLoop;
use winit::window::WindowBuilder;
use winit::{event_loop::EventLoopBuilder, window::Window};

use crate::engine::draw::Renderer;

use super::draw::Point;

pub struct WindowHandler {
    window: Window,
    event_loop: EventLoop<()>,
    renderer: Renderer,
    input_handler: InputHandler,
}
impl WindowHandler {
    pub async fn new() -> Self {
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

    pub async fn from_builders(window_builder: WindowBuilder, event_loop_builder: &mut EventLoopBuilder<()>) -> Self {
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

    pub fn main_loop<F>(mut self, mut f: F) 
    where
        F: FnMut(&mut Renderer, &mut InputHandler) -> () + 'static
    {   
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
                        WindowEvent::MouseInput { state: key_state, button: event_button, .. } => {
                            for (callback_button, (bounds, callback)) in self.input_handler.mouse_click_event_callbacks.iter() {
                                if callback_button == &event_button && self.input_handler.cursor_position.within(*bounds) {
                                    callback(key_state);
                                }
                            }
                        }
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
                    f(&mut self.renderer, &mut self.input_handler);
                },
                _ => ()
            }
        });
    }
}
pub struct InputHandler {
    key_event_callbacks: HashMap<VirtualKeyCode, fn(ElementState) -> ()>,
    mouse_click_event_callbacks: HashMap<MouseButton, ([Point;2], fn(ElementState) -> ())>,
    pub cursor_position: Point,
}
impl InputHandler {
    pub fn new() -> Self {
        InputHandler { 
            key_event_callbacks: HashMap::new(), 
            mouse_click_event_callbacks: HashMap::new(), 
            cursor_position: Point::ZERO,
        }
    }
    pub fn add_key_event_callback(&mut self, key: VirtualKeyCode, callback: fn(ElementState)) {
        self.key_event_callbacks.insert(key, callback);
    }
    pub fn add_mouse_click_event_callback(&mut self, button: MouseButton, bounds: Option<[Point;2]>, callback: fn(ElementState)) {
        if let Some(bounds) = bounds {
            self.mouse_click_event_callbacks.insert(button, (bounds, callback));
        } else {
            let bounds = [[-1., 1.].into(), [1., -1.].into()];
            self.mouse_click_event_callbacks.insert(button, (bounds, callback));
        }
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
    fn test_inputhandler_key_event() {
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
    fn test_inputhandler_cursor_position() {
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

    #[test]
    fn test_inputhandler_mouse_click() {
        async fn run() {
            let window_handler = WindowHandler::from_builders(
                WindowBuilder::default(),
                EventLoopBuilder::default().with_any_thread(true),
            ).await;
            window_handler.main_loop(|renderer, input_handler| {
                input_handler.add_mouse_click_event_callback(MouseButton::Left, Some([[-0.5, 0.5].into(), [0.5, -0.5].into()]), |button_state| {
                    if button_state == ElementState::Pressed {
                        println!("pressed left click within bounds")
                    }
                });
                renderer.render().unwrap();
            });
        }
        pollster::block_on(run())
    }
}