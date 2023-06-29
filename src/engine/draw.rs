use winit::{
    event::*,
    event_loop::{ControlFlow, EventLoop, EventLoopBuilder},
    window::{WindowBuilder, Window},
};
use wgpu::util::DeviceExt;

#[derive( Debug, PartialEq )]
struct Color {
    r: f32,
    g: f32,
    b: f32,
    a: f32,
}
impl From<[f32;3]> for Color {
    fn from(value: [f32;3]) -> Self {
        return Color{ 
            r: value[0], 
            g: value[1], 
            b: value[2], 
            a: 1., 
        };
    }
}
impl From<(f32, f32, f32)> for Color {
    fn from(value: (f32, f32, f32)) -> Self {
        return Color{ 
            r: value.0, 
            g: value.1, 
            b: value.2, 
            a: 1.,
        };
    }
}
impl From<[f32;4]> for Color {
    fn from(value: [f32;4]) -> Self {
        return Color{ 
            r: value[0], 
            g: value[1], 
            b: value[2], 
            a: value[3], 
        };
    }
}
impl From<(f32, f32, f32, f32)> for Color {
    fn from(value: (f32, f32, f32, f32)) -> Self {
        return Color{ 
            r: value.0, 
            g: value.1, 
            b: value.2, 
            a: value.3,
        };
    }
}
impl From<[u8;3]> for Color {
    fn from(value: [u8;3]) -> Self {
        return Color{ 
            r: value[0] as f32 / 255., 
            g: value[1] as f32 / 255., 
            b: value[2] as f32 / 255., 
            a: 1. 
        };
    }
}
impl From<(u8, u8, u8)> for Color {
    fn from(value: (u8, u8, u8)) -> Self {
        return Color{ 
            r: value.0 as f32 / 255., 
            g: value.1 as f32 / 255., 
            b: value.2 as f32 / 255., 
            a: 1.
        };
    }
}
impl From<[u8;4]> for Color {
    fn from(value: [u8;4]) -> Self {
        return Color{ 
            r: value[0] as f32 / 255., 
            g: value[1] as f32 / 255., 
            b: value[2] as f32 / 255., 
            a: value[3] as f32 / 100., 
        };
    }
}
impl From<(u8, u8, u8, u8)> for Color {
    fn from(value: (u8, u8, u8, u8)) -> Self {
        return Color{ 
            r: value.0 as f32 / 255., 
            g: value.1 as f32 / 255., 
            b: value.2 as f32 / 255., 
            a: value.3 as f32 / 100.,
        };
    }
}

impl From<Color> for [f32;3] {
    fn from(value: Color) -> Self {
        [
            value.r, 
            value.g, 
            value.b, 
        ]
    }
}
impl From<Color> for [f32;4] {
    fn from(value: Color) -> Self {
        [
            value.r, 
            value.g, 
            value.b,
            value.a,  
        ]
    }
}

impl std::ops::Add for Color {
    type Output = Self;
    fn add(self, rhs: Self) -> Self::Output {
        Color { 
            r: self.r + rhs.r, 
            g: self.g + rhs.g, 
            b: self.b + rhs.b, 
            a: self.a, 
        }
    }
}
impl std::ops::Sub for Color {
    type Output = Self;
    fn sub(self, rhs: Self) -> Self::Output {
        Color { 
            r: self.r - rhs.r, 
            g: self.g - rhs.g, 
            b: self.b - rhs.b, 
            a: self.a, 
        }
    }
}
impl Color {
    fn add_with_alpha(self, rhs: Self) -> Self {
        Color { 
            r: self.r + rhs.r, 
            g: self.g + rhs.g, 
            b: self.b + rhs.b, 
            a: self.a + rhs.a, 
        }
    }
    fn sub_with_alpha(self, rhs: Self) -> Self {
        Color { 
            r: self.r - rhs.r, 
            g: self.g - rhs.g, 
            b: self.b - rhs.b, 
            a: self.a - rhs.a, 
        }
    }
}

impl std::ops::Mul<f32> for Color {
    type Output = Self;
    fn mul(self, rhs: f32) -> Self::Output {
        Color { 
            r: self.r * rhs, 
            g: self.g * rhs, 
            b: self.b * rhs, 
            a: self.a, 
        }
    }
}
impl std::ops::Mul<[f32;3]> for Color {
    type Output = Self;
    fn mul(self, rhs: [f32;3]) -> Self::Output {
        Color { 
            r: self.r * rhs[0], 
            g: self.g * rhs[1], 
            b: self.b * rhs[2], 
            a: self.a, 
        }
    }
}
impl std::ops::Mul<[f32;4]> for Color {
    type Output = Self;
    fn mul(self, rhs: [f32;4]) -> Self::Output {
        Color { 
            r: self.r * rhs[0], 
            g: self.g * rhs[1], 
            b: self.b * rhs[2], 
            a: self.a * rhs[3], 
        }
    }
}

impl Color {
    const RED:         Self = Color{ r: 1., g: 0., b: 0., a: 1. };
    const GREEN:       Self = Color{ r: 0., g: 1., b: 0., a: 1. };
    const BLUE:        Self = Color{ r: 0., g: 0., b: 1., a: 1. };
    const BLACK:       Self = Color{ r: 0., g: 0., b: 0., a: 1. };
    const WHITE:       Self = Color{ r: 1., g: 1., b: 1., a: 1. };
    const TRANSPARENT: Self = Color{ r: 0., g: 0., b: 0., a: 0. };
}
impl Color {
    fn with_red(&self, red: f32) -> Self {
        Color { r: red, g: self.g, b: self.b, a: self.a }
    }
    fn with_green(&self, green: f32) -> Self {
        Color { r: self.r, g: green, b: self.b, a: self.a }
    }
    fn with_blue(&self, blue: f32) -> Self {
        Color { r: self.r, g: self.g, b: blue, a: self.a }
    }
    fn with_alpha(&self, alpha: f32) -> Self {
        Color { r: self.r, g: self.g, b: self.b, a: alpha }
    }
}

#[repr(C)]
#[derive(Copy, Clone, Debug, bytemuck::Pod, bytemuck::Zeroable)]
pub struct Vertex {
    position: [f32; 3],
    color: [f32; 4], //change this to vec4 to include alpha
}
impl Vertex {
    fn new(position: [f32;3], color: [f32;4]) -> Vertex {
        Vertex { 
            position,
            color,
        }
    }
}

pub struct Renderer {
    size: winit::dpi::PhysicalSize<u32>,
    surface: wgpu::Surface,
    device: wgpu::Device,
    queue: wgpu::Queue,
    config: wgpu::SurfaceConfiguration,

    render_pipeline: wgpu::RenderPipeline,

    pub vertices: Vec<Vertex>,
    pub indices: Vec<u16>,
}
impl Renderer {
    pub fn draw_triangle<C: Into<Color>>(&mut self, points: [[f32;2];3], color: C) {
        let color: Color = color.into();
        let color: [f32;4] = color.into();

        self.vertices.push(Vertex::new([points[0][0], points[0][1], 0.0], color));
        self.vertices.push(Vertex::new([points[1][0], points[1][1], 0.0], color));
        self.vertices.push(Vertex::new([points[2][0], points[2][1], 0.0], color));

        let offset = self.indices.len();

        self.indices.push((offset + 0) as u16);
        self.indices.push((offset + 1) as u16);
        self.indices.push((offset + 2) as u16);
    }

    pub async fn new(window: &Window) -> Self {

        let size = window.inner_size();
        
        let instance = wgpu::Instance::new(wgpu::InstanceDescriptor{ 
            backends: wgpu::Backends::all(), 
            //consider using Dxc here
            dx12_shader_compiler: wgpu::Dx12Compiler::Fxc 
        });

        let surface = unsafe { instance.create_surface(window).unwrap() };
        let adapter = instance.request_adapter(
            &wgpu::RequestAdapterOptions {
                power_preference: wgpu::PowerPreference::default(),
                compatible_surface: Some(&surface),
                force_fallback_adapter: false,
            },
        ).await.unwrap();

        let (device, queue) = adapter.request_device(
            &Default::default(),
            None,
        ).await.unwrap();

        let surface_caps = surface.get_capabilities(&adapter);
        let surface_format = surface_caps.formats.iter()
            .copied()
            .find(|f| f.is_srgb())            
            .unwrap_or(surface_caps.formats[0]);
        // try Surface::get_default_config
        let config = wgpu::SurfaceConfiguration {
            usage: wgpu::TextureUsages::RENDER_ATTACHMENT,
            format: surface_format,
            width: size.width,
            height: size.height,
            present_mode: wgpu::PresentMode::Fifo,
            alpha_mode: surface_caps.alpha_modes[0],
            view_formats: vec![],
        };
        surface.configure(&device, &config);

        let shader = device.create_shader_module(wgpu::include_wgsl!("shader.wgsl"));
        
        let render_pipeline = device.create_render_pipeline(&wgpu::RenderPipelineDescriptor {
            label: None,
            layout: None,
            vertex: wgpu::VertexState {
                module: &shader,
                entry_point: "vs_main",
                buffers: &[wgpu::VertexBufferLayout {
                    array_stride: std::mem::size_of::<Vertex>() as wgpu::BufferAddress,
                    step_mode: wgpu::VertexStepMode::Vertex,
                    attributes: &wgpu::vertex_attr_array![0 => Float32x3, 1 => Float32x4],
                }],
            },
            fragment: Some(wgpu::FragmentState {
                module: &shader,
                entry_point: "fs_main",
                targets: &[Some(wgpu::ColorTargetState {
                    format: config.format,
                    blend: Some(wgpu::BlendState::ALPHA_BLENDING), // experiment with overlapping shapes and this
                    write_mask: wgpu::ColorWrites::ALL,
                })],
            }),
            primitive: wgpu::PrimitiveState {
                topology: wgpu::PrimitiveTopology::TriangleList,
                strip_index_format: None,
                front_face: wgpu::FrontFace::Ccw,
                cull_mode: None,
                polygon_mode: wgpu::PolygonMode::Fill,
                unclipped_depth: false,
                conservative: false,
            },
            depth_stencil: None,
            multisample: wgpu::MultisampleState {
                count: 1,
                mask: !0,
                alpha_to_coverage_enabled: false,
            },
            multiview: None,
        });
        
        let vertices = Vec::new();

        let indices = Vec::new();


        Renderer {
            size,
            surface,
            device,
            queue,
            config,

            render_pipeline,

            vertices,
            indices,
        }
    }

    fn resize(&mut self, new_size: winit::dpi::PhysicalSize<u32>) {
        if new_size.width > 0 && new_size.height > 0 {
            self.size = new_size;
            self.config.width = new_size.width;
            self.config.height = new_size.height;
            self.surface.configure(&self.device, &self.config);
        }
    }

    pub fn render(&mut self) -> Result<(), wgpu::SurfaceError> {
        let output = self.surface.get_current_texture()?;

        let view = output.texture.create_view(&wgpu::TextureViewDescriptor::default());

        let mut encoder = self.device.create_command_encoder(&wgpu::CommandEncoderDescriptor { label: None, });

        let vertex_buffer = self.device.create_buffer_init(
            &wgpu::util::BufferInitDescriptor {
                label: None,
                contents: bytemuck::cast_slice(&self.vertices),
                usage: wgpu::BufferUsages::VERTEX,
            }
        );

        let index_buffer = self.device.create_buffer_init(
            &wgpu::util::BufferInitDescriptor {
                label: None,
                contents: bytemuck::cast_slice(&self.indices),
                usage: wgpu::BufferUsages::INDEX,
            }
        );

        let mut render_pass = encoder.begin_render_pass(&wgpu::RenderPassDescriptor {
            label: None,
            color_attachments: &[Some(wgpu::RenderPassColorAttachment {
                view: &view,
                resolve_target: None,
                ops: wgpu::Operations {
                    load: wgpu::LoadOp::Clear(wgpu::Color::BLACK),
                    store: true,
                },
            })],
            depth_stencil_attachment: None,
        });

        render_pass.set_pipeline(&self.render_pipeline);
        render_pass.set_vertex_buffer(0, vertex_buffer.slice(..));
        render_pass.set_index_buffer(index_buffer.slice(..), wgpu::IndexFormat::Uint16);
        render_pass.draw_indexed(0..self.indices.len() as u32, 0, 0..1);

        drop(render_pass);
    
        self.queue.submit(Some(encoder.finish()));
        output.present();
    
        Ok(())
    }
}

// ----- TESTS -----
#[cfg(test)]
mod tests {
    use super::*;    

    // ----- COLOR TESTS -----
    #[test]
    fn test_color_consts() {
        assert_eq!(Color::RED,         Color{ r: 1., g: 0., b: 0., a: 1. });
        assert_eq!(Color::GREEN,       Color{ r: 0., g: 1., b: 0., a: 1. });
        assert_eq!(Color::BLUE,        Color{ r: 0., g: 0., b: 1., a: 1. });
        assert_eq!(Color::BLACK,       Color{ r: 0., g: 0., b: 0., a: 1. });
        assert_eq!(Color::WHITE,       Color{ r: 1., g: 1., b: 1., a: 1. });
        assert_eq!(Color::TRANSPARENT, Color{ r: 0., g: 0., b: 0., a: 0. });
    }
    #[test]
    fn test_color_convert() {
        assert_eq!(Color::from([1., 0., 0.]), Color{ r: 1., g: 0., b: 0., a: 1. },      "ERROR: Failed assertion while converting from [f32;3] to Color.");
        assert_eq!(Color::from([1., 0., 0., 1.]), Color{ r: 1., g: 0., b: 0., a: 1. },  "ERROR: Failed assertion while converting from [f32;4] to Color.");

        assert_eq!(Color::from((1., 0., 0.)), Color{ r: 1., g: 0., b: 0., a: 1. },      "ERROR: Failed assertion while converting from (f32, f32, f32) to Color.");
        assert_eq!(Color::from((1., 0., 0., 1.)), Color{ r: 1., g: 0., b: 0., a: 1. },  "ERROR: Failed assertion while converting from (f32, f32, f32, f32) to Color.");
    
        assert_eq!(Color::from([255, 0, 0]), Color{ r: 1., g: 0., b: 0., a: 1. },       "ERROR: Failed assertion while converting from [u8;3] to Color.");
        assert_eq!(Color::from([255, 0, 0, 100]), Color{ r: 1., g: 0., b: 0., a: 1. },  "ERROR: Failed assertion while converting from [u8;4] to Color.");

        assert_eq!(Color::from((255, 0, 0)), Color{ r: 1., g: 0., b: 0., a: 1. },       "ERROR: Failed assertion while converting from (u8, u8, u8) to Color.");
        assert_eq!(Color::from((255, 0, 0, 100)), Color{ r: 1., g: 0., b: 0., a: 1. },  "ERROR: Failed assertion while converting from (u8, u8, u8, u8) to Color.");
    }
    #[test]
    fn test_color_with() {
        assert_eq!(Color{ r: 0.5, g: 0., b: 0., a: 1. }, Color::RED.with_red(0.5),      "ERROR: Failed assertion while calling Color.with_red(f32).");
        assert_eq!(Color{ r: 0., g: 0.5, b: 0., a: 1. }, Color::GREEN.with_green(0.5),  "ERROR: Failed assertion while calling Color.with_green(f32).");
        assert_eq!(Color{ r: 0., g: 0., b: 0.5, a: 1. }, Color::BLUE.with_blue(0.5),    "ERROR: Failed assertion while calling Color.with_blue(f32).");
        assert_eq!(Color{ r: 0., g: 0., b: 0., a: 0.5 }, Color::BLACK.with_alpha(0.5),  "ERROR: Failed assertion while calling Color.with_alpha(f32).");
    }
    #[test]
    fn test_color_ops() {
        assert_eq!(Color{r: 1., g: 1., b: 0., a: 1.}, Color::RED + Color::GREEN, "ERROR: Failed assertion while adding Color and Color.");
        assert_eq!(Color{r: 0., g: 1., b: 1., a: 1.}, Color::WHITE - Color::RED, "ERROR: Failed assertion while subtracting Color and Color.");
        
        assert_eq!(Color{r: 1., g: 0., b: 0., a: 1.}, Color::RED.add_with_alpha(Color::TRANSPARENT), "ERROR: Failed assertion while calling Color.add_with_alpha().");
        assert_eq!(Color{r: 0., g: 1., b: 1., a: 0.}, Color::WHITE.sub_with_alpha(Color::RED), "ERROR: Failed assertion while calling Color.sub_with_alpha().");

        assert_eq!(Color{r: 0.5, g: 0.5, b: 0.5, a: 1.}, Color::WHITE * 0.5, "ERROR: Failed assertion while multiplying Color and f32.");
        assert_eq!(Color{r: 0., g: 0.5, b: 1., a: 1.}, Color::WHITE * [0., 0.5, 1.], "ERROR: Failed assertion while multiplying Color and [f32;3].");
        assert_eq!(Color{r: 1., g: 0.5, b: 0., a: 0.5}, Color::WHITE * [1., 0.5, 0., 0.5], "ERROR: Failed assertion while multiplying Color and [f32;4].");
    }

    // ----- RENDERER TESTS -----
    // creating a winit EventLoop in non main thread requires this import and to configure it with .with_any_thread(true)
    use winit::platform::wayland::EventLoopBuilderExtWayland;

    #[test]
    #[ignore = "requires manual validation, run separetely"]
    fn test_renderer() {
        async fn run() {
            let event_loop = EventLoopBuilder::new().with_any_thread(true).build();
            let window = Window::new(&event_loop).unwrap();
            let mut renderer = Renderer::new(&window).await;
            event_loop.run(move |_, _, _| {
                renderer.render().unwrap();
            });
        }

        pollster::block_on(run())
    }

    #[test]
    #[ignore = "requires manual validation, run separetely"]
    fn test_renderer_draw_triangle() {
        async fn run() {
            let event_loop = EventLoopBuilder::new().with_any_thread(true).build();
            let window = Window::new(&event_loop).unwrap();
            let mut renderer = Renderer::new(&window).await;
            event_loop.run(move |_, _, _| {
                renderer.draw_triangle([[0.0, 0.5], [-0.5, -0.5], [0.5, -0.5]], Color::RED);

                renderer.render().unwrap();
            });
        }

        pollster::block_on(run())
    }

    #[test]
    #[ignore = "requires manual validation, run separetely"]
    fn test_renderer_alpha() {
        async fn run() {
            let event_loop = EventLoopBuilder::new().with_any_thread(true).build();
            let window = Window::new(&event_loop).unwrap();
            let mut renderer = Renderer::new(&window).await;
            event_loop.run(move |_, _, _| {
                renderer.draw_triangle([[0.25, 0.5], [-0.25, -0.5], [0.75, -0.5]], Color::BLUE);
                renderer.draw_triangle([[-0.25, 0.5], [-0.75, -0.5], [0.25, -0.5]], Color::RED.with_alpha(0.5));

                renderer.render().unwrap();
            });
        }

        pollster::block_on(run())
    }
}