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
        for menu in self.menus.iter_mut() {
            menu.draw_menu(self.renderer)
        }
    }
}

struct MenuStyle {
    frame_thickness: f32,
    bg_color: Color,
    frame_color: Color,
    spacing: f32,
    default_text_color: Color
}
impl Default for MenuStyle {
    fn default() -> Self {
        MenuStyle {
            frame_thickness: 0.01,
            bg_color: Color::BLACK,
            frame_color: Color::WHITE,
            spacing: 0.005,
            default_text_color: Color::WHITE
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
            default_text_color: self.default_text_color,
        }
    }
    fn with_bg_color(&self, bg_color: Color) -> Self {
        MenuStyle {
            frame_thickness: self.frame_thickness,
            bg_color,
            frame_color: self.frame_color,
            spacing: self.spacing,
            default_text_color: self.default_text_color,
        }
    }
    fn with_frame_color(&self, frame_color: Color) -> Self {
        MenuStyle {
            frame_thickness: self.frame_thickness,
            bg_color: self.bg_color,
            frame_color,
            spacing: self.spacing,
            default_text_color: self.default_text_color,
        }
    }
    fn with_spacing(&self, spacing: f32) -> Self {
        MenuStyle {
            frame_thickness: self.frame_thickness,
            bg_color: self.bg_color,
            frame_color: self.frame_color,
            spacing,
            default_text_color: self.default_text_color,
        }
    }
    fn with_default_text_color(&self, default_text_color: Color) -> Self {
        MenuStyle {
            frame_thickness: self.frame_thickness,
            bg_color: self.bg_color,
            frame_color: self.frame_color,
            spacing: self.spacing,
            default_text_color,
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
    default_text_color: Color
}
impl Menu {
    fn new(corners: [Point;2], frame_thickness: f32, bg_color: Color, frame_color: Color, spacing: f32, default_text_color: Color) -> Self {
        Menu {
            wigets: Vec::new(),
            corners,
            frame_thickness,
            bg_color,
            frame_color,
            spacing,
            default_text_color,
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
            default_text_color: style.default_text_color,
        }
    }

    fn draw_menu(&mut self, renderer: &mut Renderer) {
        renderer.draw_rect(self.corners, self.bg_color);
        renderer.draw_box(self.corners, self.frame_thickness, self.frame_color);

        let mut widget_offset = self.corners[0] - [-1. * (self.frame_thickness + self.spacing), self.frame_thickness + self.spacing].into();
        for (i, widget) in self.wigets.iter_mut().enumerate() {
            widget_offset = widget_offset - [0., self.spacing * i as f32].into();
            widget.draw_widget(renderer, widget_offset);
            widget_offset = widget_offset - [0., widget.height()].into();
        }
    }

    fn add_widget(&mut self, widget: Box<dyn Widget>) {
        let mut widget = widget;
        widget.set_text_color_if_none(self.default_text_color);
        self.wigets.push(widget)
    }
}

trait Widget {
    fn draw_widget(&mut self, renderer: &mut Renderer, position: Point);
    fn height(&self) -> f32;
    fn set_text_color_if_none(&mut self, text_color: Color);
}

struct Label {
    text: String,
    font_size: f32,
    text_color: Option<Color>,
}
impl Label {
    fn new(text: String, font_size: f32, text_color: Option<Color>) -> Label {
        Label {
            text,
            font_size,
            text_color,
        }
    }
}
impl Widget for Label {
    fn height(&self) -> f32 {
        self.font_size
    }

    fn draw_widget(&mut self, renderer: &mut Renderer, position: Point) {
        if let Some(text_color) = self.text_color {
            renderer.draw_text(position, &self.text, text_color, self.font_size)
        } else {
            panic!("ERROR: attempted to draw UI widget without a text_color")
        }
    }

    fn set_text_color_if_none(&mut self, text_color: Color) {
        if self.text_color == None {
            self.text_color = Some(text_color)
        }
    }
}

struct Button {
    text: String,
    font_size: f32,
    text_color: Option<Color>,
    padding: f32,
    frame_thickness: f32,
    frame_color: Color,
    bounds: Option<[Point;2]>,
    callback: Box<dyn FnOnce() -> ()>,
}
impl Button {
    fn new(
        text: String,
        font_size: f32,
        text_color: Option<Color>,
        padding: f32,
        frame_thickness: f32,
        frame_color: Color,
        callback: Box<dyn FnOnce() -> ()>,
    ) -> Self {
        Button { 
            text, 
            font_size, 
            text_color, 
            padding, 
            frame_thickness, 
            frame_color, 
            bounds: None,
            callback, 
        }
    }
    fn calculate_bounds(&mut self, position: Point) {
        self.bounds = Some([
            position - [0., self.frame_thickness + self.padding].into(),
            [
                position.x + self.text.len() as f32 * (self.font_size / 2.) as f32 + self.padding, 
                position.y - self.height() - self.frame_thickness - self.padding,
            ].into(),
        ])
    }
}
impl Widget for Button {
    fn height(&self) -> f32 {
        self.font_size
    }
    fn draw_widget(&mut self, renderer: &mut Renderer, position: Point) {
        self.calculate_bounds(position);
        if let Some(text_color) = self.text_color {
            if let Some(bounds) = self.bounds {
                renderer.draw_box(bounds, self.frame_thickness, self.frame_color);
            } else {
                panic!("ERROR attempted to draw UI Button without bounds")
            }
            renderer.draw_text(position.add_x_sub_y(self.frame_thickness + self.padding), &self.text, text_color, self.font_size)
        } else {
            panic!("ERROR: attempted to draw UI widget without a text_color")
        }
    }
    fn set_text_color_if_none(&mut self, text_color: Color) {
        if self.text_color == None {
            self.text_color = Some(text_color)
        }
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

                test_menu.add_widget(Box::new(Label::new("Hello World".to_string(), 0.1, None)));
                test_menu.add_widget(Box::new(Button::new(
                    "Hello World!".to_string(), 
                    0.1, 
                    None, 
                    0.01, 
                    0.01, 
                    Color::BLUE, 
                    Box::new(|| {}),
                )));

                ui.add_menu(test_menu);
                ui.draw_menus();

                renderer.render().unwrap();
            });
        }
        pollster::block_on(run())
    }
}