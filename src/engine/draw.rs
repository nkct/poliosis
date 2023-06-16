use winit::{
    event::*,
    event_loop::{ControlFlow, EventLoop, EventLoopBuilder},
    window::{WindowBuilder, Window},
    platform::wayland::EventLoopBuilderExtWayland
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
    fn from(arr: [f32;3]) -> Self {
        return Color{ r: arr[0], g: arr[1], b: arr[2], a: 1. };
    }
}
impl From<(f32, f32, f32)> for Color {
    fn from(tup: (f32, f32, f32)) -> Self {
        return Color{ r: tup.0, g: tup.1, b: tup.2, a: 1.};
    }
}
impl From<[f32;4]> for Color {
    fn from(arr: [f32;4]) -> Self {
        return Color{ r: arr[0], g: arr[1], b: arr[2], a: arr[3] };
    }
}
impl From<(f32, f32, f32, f32)> for Color {
    fn from(tup: (f32, f32, f32, f32)) -> Self {
        return Color{ r: tup.0, g: tup.1, b: tup.2, a: tup.3};
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

#[repr(C)]
#[derive(Copy, Clone, Debug, bytemuck::Pod, bytemuck::Zeroable)]
struct Vertex {
    position: [f32; 3],
    color: [f32; 3], //change this to vec4 to include alpha
}
impl Vertex {
    fn new(position: [f32;3], color: [f32;3]) -> Vertex {
        Vertex { 
            position,
            color,
        }
    }
}

struct Renderer {
    size: winit::dpi::PhysicalSize<u32>,
    surface: wgpu::Surface,
    device: wgpu::Device,
    queue: wgpu::Queue,
    config: wgpu::SurfaceConfiguration,

    render_pipeline: wgpu::RenderPipeline,

    vertices: Vec<Vertex>,
    indices: Vec<u16>,
}
impl Renderer {
    fn draw_triangle(&mut self, points: [[f32;2];3], color: [f32;3]) {
        self.vertices.push(Vertex::new([points[0][0], points[0][1], 0.0], color));
        self.vertices.push(Vertex::new([points[1][0], points[1][1], 0.0], color));
        self.vertices.push(Vertex::new([points[2][0], points[2][1], 0.0], color));

        let offset = self.indices.len();

        self.indices.push((offset + 0) as u16);
        self.indices.push((offset + 1) as u16);
        self.indices.push((offset + 2) as u16);
    }

    async fn new(window: &Window) -> Self {

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
            &wgpu::DeviceDescriptor {
                features: wgpu::Features::empty(),
                limits: wgpu::Limits::default(),
                label: None,
            },
            None,
        ).await.unwrap();

        let surface_caps = surface.get_capabilities(&adapter);
        let surface_format = surface_caps.formats.iter()
            .copied()
            .find(|f| f.is_srgb())            
            .unwrap_or(surface_caps.formats[0]);
        let config = wgpu::SurfaceConfiguration {
            usage: wgpu::TextureUsages::RENDER_ATTACHMENT,
            format: surface_format,
            width: size.width,
            height: size.height,
            present_mode: surface_caps.present_modes[0],
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
                buffers: &[
                    wgpu::VertexBufferLayout {
                        array_stride: std::mem::size_of::<Vertex>() as wgpu::BufferAddress,
                        step_mode: wgpu::VertexStepMode::Vertex,
                        attributes: &[
                            wgpu::VertexAttribute {
                                offset: 0,
                                shader_location: 0,
                                format: wgpu::VertexFormat::Float32x3,
                            },
                            wgpu::VertexAttribute {
                                offset: std::mem::size_of::<[f32; 3]>() as wgpu::BufferAddress,
                                shader_location: 1,
                                format: wgpu::VertexFormat::Float32x3,
                            }
                        ]
                    }
                ],
            },
            fragment: Some(wgpu::FragmentState {
                module: &shader,
                entry_point: "fs_main",
                targets: &[Some(wgpu::ColorTargetState {
                    format: config.format,
                    blend: Some(wgpu::BlendState::REPLACE), // experiment with overlapping shapes and this
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

    fn render(&mut self) -> Result<(), wgpu::SurfaceError> {
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
                    load: wgpu::LoadOp::Clear(wgpu::Color {
                        r: 0.0,
                        g: 0.0,
                        b: 0.0,
                        a: 1.0,
                    }),
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
    
        self.queue.submit(std::iter::once(encoder.finish()));
        output.present();
    
        Ok(())
    }
}

// ----- TESTS -----
#[cfg(test)]
mod tests {
    use super::*;    

    #[test]
    fn test_renderer() {
        async fn run() {
            let event_loop = EventLoopBuilder::new().with_any_thread(true).build();
            let window = Window::new(&event_loop).unwrap();

            let mut renderer = Renderer::new(&window).await;

            event_loop.run(move |event, _, _| match event {
                Event::RedrawRequested(_) => {
                    renderer.draw_triangle([[0.0, 0.5], [-0.5, -0.5], [0.5, -0.5]], [1.0, 0.0, 0.0]);
        
                    renderer.render();
                }
                _ => {}
            });
        }

        pollster::block_on(run())
    }
}