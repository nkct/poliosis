use winit::event::ElementState;

use crate::engine::draw::{ Renderer, Point, Color };
use crate::engine::window::InputHandler;

struct UIContext<'a> {
    menus: Vec<Menu>,
    renderer: &'a mut Renderer,
    input_handler: &'a mut InputHandler,
}
impl<'a> UIContext<'a> {
    fn new(renderer: &'a mut Renderer, input_handler: &'a mut InputHandler) -> Self {
        
        UIContext { 
            menus: Vec::new(), 
            renderer, 
            input_handler,
        }
    }

    fn add_menu(&mut self, menu: Menu) -> &mut Menu{
        self.menus.push(menu);
        return self.menus.iter_mut().last().unwrap();
    }

    fn remove_menu(&mut self, index: usize) {
        self.menus.remove(index);
    }

    fn draw_menus(&mut self) {
        for menu in self.menus.iter_mut() {
            menu.draw_menu(self.renderer, self.input_handler)
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
impl Default for Menu {
    fn default() -> Self {
        Menu {
            wigets: Vec::new(),
            corners: [Point::ZERO, Point::ZERO],
            frame_thickness: 0.01,
            bg_color: Color::BLACK,
            frame_color: Color::WHITE,
            spacing: 0.005,
            default_text_color: Color::WHITE
        }  
    }
}
impl Menu {
    fn new<P: Into<Point>>(corners: [P;2], frame_thickness: f32, bg_color: Color, frame_color: Color, spacing: f32, default_text_color: Color) -> Self {
        Menu {
            wigets: Vec::new(),
            corners: corners.map(|p| p.into()),
            frame_thickness,
            bg_color,
            frame_color,
            spacing,
            default_text_color,
        }
    }
    fn from_corners<P: Into<Point>>(corners: [P;2]) -> Self{
        Menu {
            corners: corners.map(|p| p.into()),
            ..Default::default()
        }
    }

    fn draw_menu(&mut self, renderer: &mut Renderer, input_handler: &mut InputHandler) {
        renderer.draw_rect(self.corners, self.bg_color);
        renderer.draw_box(self.corners, self.frame_thickness, self.frame_color);

        let mut widget_offset = self.corners[0] - [-1. * (self.frame_thickness + self.spacing), self.frame_thickness + self.spacing].into();
        for (i, widget) in self.wigets.iter_mut().enumerate() {
            widget_offset = widget_offset - [0., self.spacing * i as f32].into();
            widget.display_widget(renderer, input_handler,  widget_offset);
            widget_offset = widget_offset - [0., widget.height()].into();
        }
    }

    fn add_widget<W: Widget + 'static>(&mut self, widget: W) {
        let mut widget = widget;
        widget.set_text_color_if_none(self.default_text_color);
        self.wigets.push(Box::new(widget))
    }
}

trait Widget {
    fn display_widget(&mut self, renderer: &mut Renderer, input_handler: &mut InputHandler, position: Point);
    fn height(&self) -> f32;
    fn set_text_color_if_none(&mut self, text_color: Color);
}

struct Label<'a> {
    text: &'a str,
    font_size: f32,
    text_color: Option<Color>,
}
impl Default for Label<'_> {
    fn default() -> Self {
        Label {
            text: "",
            font_size: 0.1,
            text_color: None,
        }
    }
}
impl Label<'_> {
    fn new(text: &str, font_size: f32, text_color: Option<Color>) -> Label {
        Label {
            text,
            font_size,
            text_color,
        }
    }
}
impl Widget for Label<'_> {
    fn height(&self) -> f32 {
        self.font_size * self.text.split("\n").collect::<Vec<_>>().len() as f32
    }

    fn display_widget(&mut self, renderer: &mut Renderer, _input_handler: &mut InputHandler, position: Point) {
        if let Some(text_color) = self.text_color {
            renderer.draw_text(position, self.text, text_color, self.font_size)
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

struct Button<'a> {
    text: &'a str,
    font_size: f32,
    text_color: Option<Color>,
    padding: f32,
    frame_thickness: f32,
    frame_color: Color,
    bounds: Option<[Point;2]>,
    callback: fn(ElementState),
}
impl Default for Button<'_> {
    fn default() -> Self {
            Button {
            text: "",
            font_size: 0.1,
            text_color: None,
            padding: 0.01,
            frame_thickness: 0.01,
            frame_color: Color::WHITE,
            bounds: None,
            callback: |_| {},
        }
    }
}
impl<'a> Button<'a> {
    fn new(
        text: &'a str,
        font_size: f32,
        text_color: Option<Color>,
        padding: f32,
        frame_thickness: f32,
        frame_color: Color,
        callback: fn(ElementState),
    ) -> Self {
        Button { 
            text: text, 
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
        let num_of_lines = self.text.split("\n").collect::<Vec<_>>().len() as f32;
        self.bounds = Some([
            position,
            [
                position.x + (((self.text.len() as f32 / num_of_lines) * (self.font_size / 2.)) * 0.95) + (2. * (self.frame_thickness + self.padding)), 
                position.y - (self.font_size * num_of_lines) - (2. * (self.frame_thickness + self.padding)),
            ].into(),
        ])
    }
}
impl Widget for Button<'_> {
    fn height(&self) -> f32 {
        self.font_size * self.text.split("\n").collect::<Vec<_>>().len() as f32 + (2. * (self.frame_thickness + self.padding))
    }
    fn display_widget(&mut self, renderer: &mut Renderer, input_handler: &mut InputHandler, position: Point) {
        self.calculate_bounds(position);
        input_handler.add_mouse_click_event_callback(winit::event::MouseButton::Left, self.bounds, self.callback);
        
        if let Some(text_color) = self.text_color {
            if let Some(bounds) = self.bounds {
                renderer.draw_box(bounds, self.frame_thickness, self.frame_color);
            } else {
                panic!("ERROR attempted to draw UI Button without bounds")
            }
            renderer.draw_text(position.add_x_sub_y(self.frame_thickness + self.padding), self.text, text_color, self.font_size)
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
    use crate::engine::window::WindowHandler;

    use super::*;  
    
    use winit::window::WindowBuilder;
    use winit::event_loop::EventLoopBuilder;
    use winit::platform::wayland::EventLoopBuilderExtWayland;

    #[test]
    #[ignore = "requires manual validation, run separetely"]
    fn test_ui_widgets() {
        async fn run() {
            let window_handler = WindowHandler::from_builders(
                WindowBuilder::default(),
                EventLoopBuilder::default().with_any_thread(true),
            ).await;
            window_handler.main_loop(|renderer, input_handler| {

                let mut ui = UIContext::new(renderer, input_handler);
                
                let test_menu = ui.add_menu(Menu::from_corners([[-0.5, 0.5], [0.5, -0.5]]));

                test_menu.add_widget(
                    Label{
                        text: "Hello World",
                        ..Default::default()
                    }
                );
                test_menu.add_widget(
                    Button{
                        text: "Hello World!",
                        frame_color: Color::BLUE, 
                        callback: |bttn_state| { if bttn_state == ElementState::Pressed {println!("button clicked")} },
                        ..Default::default()
                    }
                );

                ui.draw_menus();
                renderer.render().unwrap();
            });
        }
        pollster::block_on(run())
    }

    // showcasing how to define and use a custom default stle for a widget
    #[test]
    #[ignore = "requires manual validation, run separetely"]
    fn test_ui_custom_defaults() {
        async fn run() {
            let window_handler = WindowHandler::from_builders(
                WindowBuilder::default(),
                EventLoopBuilder::default().with_any_thread(true),
            ).await;
            window_handler.main_loop(|renderer, input_handler| {

                let my_custom_default_button: Button = Button{
                    text: "HELLO!",
                    frame_color: Color::BLUE,
                    ..Button::default()
                };

                let mut ui = UIContext::new(renderer, input_handler);
                
                let mut test_menu = Menu::from_corners([[-0.5, 0.5], [0.5, -0.5]]);

                test_menu.add_widget(Button{
                    callback: |bttn_state| { if bttn_state == ElementState::Pressed {println!("button clicked")} },
                    ..my_custom_default_button
                });

                ui.add_menu(test_menu);
                ui.draw_menus();

                renderer.render().unwrap();
            });
        }
        pollster::block_on(run())
    }

    #[test]
    #[ignore = "requires manual validation, run separetely"]
    fn test_ui_multiline() {
        async fn run() {
            let window_handler = WindowHandler::from_builders(
                WindowBuilder::default(),
                EventLoopBuilder::default().with_any_thread(true),
            ).await;
            window_handler.main_loop(|renderer, input_handler| {
                let mut ui = UIContext::new(renderer, input_handler);
                let first_menu = ui.add_menu(Menu::from_corners([[-0.5, 0.5], [0.5, -0.5]]));
                first_menu.add_widget(
                    Label{
                        text: "Hello World \nHello Wordl!",
                        ..Default::default()
                    }
                );
                first_menu.add_widget(
                    Label{
                        text: "Goodbye World \nGoodbye Wordl!",
                        ..Default::default()
                    }
                );
                first_menu.add_widget(
                    Button{
                        text: "Hello World \nHello Wordl!",
                        ..Default::default()
                    }
                );
                first_menu.add_widget(
                    Button{
                        text: "Goodbye World \nGoodbye Wordl!",
                        ..Default::default()
                    }
                );

                ui.draw_menus();
                renderer.render().unwrap();
            });
        }
        pollster::block_on(run())
    }

}