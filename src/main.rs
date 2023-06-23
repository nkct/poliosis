//use poliosis_test::run;


mod engine;
use engine::draw::Renderer;
use winit::{event_loop::EventLoop, window::Window};

fn main() {

    async fn run() {
        let event_loop = EventLoop::new();
        let window = Window::new(&event_loop).unwrap();
        let mut renderer = Renderer::new(&window).await;
        event_loop.run(move |_, _, _| {
            renderer.indices = Vec::new();
            renderer.vertices = Vec::new();

            renderer.draw_triangle([[-0.25, 0.5], [-0.75, -0.5], [0.25, -0.5]], [0.0, 0.0, 1.0, 0.5]);
            renderer.draw_triangle([[0.25, 0.5], [-0.25, -0.5], [0.75, -0.5]], [1.0, 0.0, 0.0, 0.5]);

            renderer.render().unwrap();
        });
    }

    pollster::block_on(run())
    
    //pollster::block_on(run());
}