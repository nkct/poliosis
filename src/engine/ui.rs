use crate::engine::draw::{ Renderer, Point, Color };

struct UIContext<'a> {
    menus: Vec<Menu>,
    renderer: &'a mut Renderer,
}
impl<'a> UIContext<'a> {
    fn new(renderer: &'a mut Renderer) -> Self {
        
        UIContext { 
            menus: Vec::new(), 
            renderer, 
        }
    }

    fn add_menu(&mut self, menu: Menu) {
        self.menus.push(menu)
    }

    fn draw_menus(&mut self) {
        for menu in &self.menus {
            menu.draw_menu(self.renderer)
        }
    }
}

struct MenuStyle {
    frame_thickness: f32,
    bg_color: Color,
    frame_color: Color,
    spacing: f32,
}
impl Default for MenuStyle {
    fn default() -> Self {
        MenuStyle {
            frame_thickness: 0.01,
            bg_color: Color::BLACK,
            frame_color: Color::WHITE,
            spacing: 0.005,
        }
    }
}
impl MenuStyle {
    fn with_frame_thickness(&self, frame_thickness: f32) -> Self {
        MenuStyle {
            frame_thickness,
            bg_color: self.bg_color,
            frame_color: self.frame_color,
            spacing: self.spacing,
        }
    }
    fn with_bg_color(&self, bg_color: Color) -> Self {
        MenuStyle {
            frame_thickness: self.frame_thickness,
            bg_color,
            frame_color: self.frame_color,
            spacing: self.spacing,
        }
    }
    fn with_frame_color(&self, frame_color: Color) -> Self {
        MenuStyle {
            frame_thickness: self.frame_thickness,
            bg_color: self.bg_color,
            frame_color,
            spacing: self.spacing,
        }
    }
    fn with_spacing(&self, spacing: f32) -> Self {
        MenuStyle {
            frame_thickness: self.frame_thickness,
            bg_color: self.bg_color,
            frame_color: self.frame_color,
            spacing,
        }
    }
}
struct Menu {
    wigets: Vec<Box<dyn Widget>>,
    corners: [Point;2],
    frame_thickness: f32,
    bg_color: Color,
    frame_color: Color,
    spacing: f32,
}
impl Menu {
    fn new(corners: [Point;2], frame_thickness: f32, bg_color: Color, frame_color: Color, spacing: f32,) -> Self {
        Menu {
            wigets: Vec::new(),
            corners,
            frame_thickness,
            bg_color,
            frame_color,
            spacing,
        }
    }

    fn from_style(style: MenuStyle, corners: [Point;2]) -> Self {
        Menu {
            wigets: Vec::new(),
            corners,
            frame_thickness: style.frame_thickness,
            bg_color: style.bg_color,
            frame_color: style.frame_color,
            spacing: style.spacing,
        }
    }

    fn draw_menu(&self, renderer: &mut Renderer) {
        renderer.draw_rect(self.corners, self.bg_color);
        renderer.draw_box(self.corners, self.frame_thickness, self.frame_color);

        let mut widget_offset = self.corners[0] - [-1. * (self.frame_thickness + self.spacing), self.frame_thickness + self.spacing].into();
        for (i, widget) in self.wigets.iter().enumerate() {
            widget_offset = widget_offset - [0., self.spacing * i as f32].into();
            widget.draw_widget(renderer, widget_offset);
            widget_offset = widget_offset - [0., widget.height()].into();
        }
    }

    fn add_widget(&mut self, widget: Box<dyn Widget>) {
        self.wigets.push(widget)
    }
}

trait Widget {
    fn draw_widget(&self, renderer: &mut Renderer, position: Point);
    fn height(&self) -> f32;
}

struct Label {
    text: String,
    scale: f32,
    color: Color,
}
impl Label {
    fn new(text: String, scale: f32, color: Color) -> Label {
        Label {
            text,
            scale,
            color,
        }
    }
}
impl Widget for Label {
    fn height(&self) -> f32 {
        self.scale
    }

    fn draw_widget(&self, renderer: &mut Renderer, position: Point) {
        renderer.draw_text(position, &self.text, self.color, self.scale)
    }
}

// ----- TESTS -----
#[cfg(test)]
mod tests {
    use super::*;  
    
    use winit::{window::Window, event_loop::EventLoopBuilder};
    use winit::platform::wayland::EventLoopBuilderExtWayland;

    #[test]
    fn test_ui() {
        async fn run() {
            let event_loop = EventLoopBuilder::new().with_any_thread(true).build();
            let window = Window::new(&event_loop).unwrap();
            let mut renderer = Renderer::new(&window).await;
            event_loop.run(move |_, _, _| {

                let mut ui = UIContext::new(&mut renderer);
                
                let mut test_menu = Menu::from_style(
                    MenuStyle::default(),
                    [[-0.5, 0.5].into(), [0.5, -0.5].into()],
                );

                test_menu.add_widget(Box::new(Label::new("Hello World".to_string(), 0.1, Color::WHITE)));

                ui.add_menu(test_menu);
                ui.draw_menus();

                renderer.render().unwrap();
            });
        }
        pollster::block_on(run())
    }
}