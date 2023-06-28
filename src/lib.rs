use winit::{
    event::*,
    event_loop::{ControlFlow, EventLoop},
    window::{WindowBuilder, Window},
};
use wgpu::util::DeviceExt;
use std::ops::Add;
use std::collections::HashMap;
use wgpu_glyph::{ab_glyph, GlyphBrushBuilder, Section, Text};
use std::time::Duration;
use std::thread::sleep;
use std::fs::{
    File,
    self,
};
use std::error::Error;
use std::io::prelude::*;
use regex::Regex;
use chrono::prelude::{
    SecondsFormat,
    Local,
};
use std::fmt;
use std::path;


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

#[derive(Debug, Copy, Clone, PartialEq)]
struct Point {
    x: f32,
    y: f32,
}
impl Point {
    fn add_x(self, other: Self) -> Self {
        Self {
            x: self.x + other.x,
            y: self.y,
        }
    }
    fn add_y(self, other: Self) -> Self {
        Self {
            x: self.x,
            y: self.y + other.y,
        }
    }
}
impl Add for Point {
    type Output = Self;

    fn add(self, other: Self) -> Self {
        Self {
            x: self.x + other.x,
            y: self.y + other.y,
        }
    }
}


struct ColorRGB {
    r: f32,
    g: f32,
    b: f32,
}
impl ColorRGB {
    fn new(r: f32, g: f32, b: f32) -> ColorRGB {
        ColorRGB {
            r,
            g,
            b,
        }
    }
}

struct TwinBuffers {
    vertices: Vec<Vertex>,
    indices: Vec<u16>,
}
impl TwinBuffers {
    fn new() -> Self {
        TwinBuffers {
            vertices: Vec::new(),
            indices: Vec::new(),
        }
    }

    fn draw_triangle(&mut self, points: [[f32;2];3], color: [f32;3]) {
        self.vertices.push(Vertex::new([points[0][0], points[0][1], 0.0], color));
        self.vertices.push(Vertex::new([points[1][0], points[1][1], 0.0], color));
        self.vertices.push(Vertex::new([points[2][0], points[2][1], 0.0], color));

        let offset = self.indices.len();

        self.indices.push((offset + 0) as u16);
        self.indices.push((offset + 1) as u16);
        self.indices.push((offset + 2) as u16);
    }

    fn draw_rectangle(&mut self, corners: [[f32;2];2], color: [f32;3]) {
        self.draw_triangle([corners[0], [corners[0][0], corners[1][1]], corners[1]], color);
        self.draw_triangle([corners[0], [corners[1][0], corners[0][1]], corners[1]], color);
    }

    // experiment with replacing this with a way to draw lines using LineList primitive
    fn draw_rect_line(&mut self, points: [[f32;2];2], thickness: f32, color: [f32;3]) {
        let thickness = thickness / 2.;
        self.draw_triangle([[points[0][0] + thickness, points[0][1]], [points[0][0] - thickness, points[0][1]], [points[1][0] - thickness, points[1][1]]], color);
        self.draw_triangle([[points[0][0] + thickness, points[0][1]], [points[1][0] + thickness, points[1][1]], [points[1][0] - thickness, points[1][1]]], color);
    }

    fn draw_box(&mut self, corners: [[f32;2];2], thickness: f32, color: [f32;3]) {
        self.draw_rectangle([corners[0], [corners[0][0] + thickness, corners[1][1]]], color);
        self.draw_rectangle([[corners[0][0] + thickness, corners[1][1] + thickness], [corners[1][0] - thickness, corners[1][1]]], color);
        self.draw_rectangle([[corners[1][0] - thickness, corners[0][1]], corners[1]], color);
        self.draw_rectangle([[corners[0][0] + thickness, corners[0][1]], [corners[1][0] - thickness, corners[0][1] - thickness]], color);
    }

    fn draw_lined_box(&mut self, corners: [[f32;2];2], thickness: f32, lines: u8, color: [f32;3], mirrored: bool) {
        self.draw_box([corners[0], corners[1]], thickness, color);


        let width = corners[1][0] - corners[0][0];
        let height = corners[0][1] - corners[1][1];
        let two: f32 = 2.;
        let base_offset_x = ((width * two.sqrt()) / (lines as f32 + 1.) * (width * two.sqrt()) / (lines as f32 + 1.) * 2.).sqrt();
        let base_offset_y = ((height * two.sqrt()) / (lines as f32 + 1.) * (height * two.sqrt()) / (lines as f32 + 1.) * 2.).sqrt();

        let mut x = [0, 1];
        let mut flip = 1.;

        if mirrored {
            x = [1, 0];
            flip *= -1.;
        }


        for i in 0..lines {
            
            if 1. + i as f32 <= (lines / 2) as f32 {
                let offset_x = base_offset_x * (i + 1) as f32;
                let offset_y = base_offset_y * (i + 1) as f32;
                let p = [[corners[x[0]][0] + thickness * flip / 2., corners[0][1] + thickness / 2. - offset_y], [corners[x[0]][0] - thickness * flip / 2. + offset_x * flip, corners[0][1] - thickness / 2.]];
                self.draw_rect_line(p, thickness, color);
            } else {
                let offset_x = base_offset_x * (i + 1 - lines / 2) as f32;
                let offset_y = base_offset_y * (i + 1 - lines / 2) as f32;
                let p = [[corners[x[1]][0] + thickness * flip / 2. - offset_x * flip, corners[1][1] + thickness / 2.], [corners[x[1]][0] - thickness * flip / 2., corners[1][1] - thickness / 2. + offset_y]];
                self.draw_rect_line(p, thickness, color);
            }
        }
    }

    fn draw_crossed_box(&mut self, corners: [[f32;2];2], thickness: f32, lines: u8, color: [f32;3]) {
        self.draw_box([corners[0], corners[1]], thickness, color);

        let width = corners[1][0] - corners[0][0];
        let height = corners[0][1] - corners[1][1];
        let two: f32 = 2.;
        let base_offset_x = ((width * two.sqrt()) / (lines as f32 + 1.) * (width * two.sqrt()) / (lines as f32 + 1.) * 2.).sqrt();
        let base_offset_y = ((height * two.sqrt()) / (lines as f32 + 1.) * (height * two.sqrt()) / (lines as f32 + 1.) * 2.).sqrt();


        for i in 0..lines {
            
            if 1. + i as f32 <= (lines / 2) as f32 {
                let offset_x = base_offset_x * (i + 1) as f32;
                let offset_y = base_offset_y * (i + 1) as f32;
                let p = [[corners[1][0] - thickness / 2., corners[0][1] + thickness / 2. - offset_y], [corners[1][0] + thickness / 2. - offset_x, corners[0][1] - thickness / 2.]];
                self.draw_rect_line(p, thickness, color);
            } else {
                let offset_x = base_offset_x * (i + 1 - lines / 2) as f32;
                let offset_y = base_offset_y * (i + 1 - lines / 2) as f32;
                let p = [[corners[0][0] - thickness / 2. + offset_x, corners[1][1] + thickness / 2.], [corners[0][0] + thickness / 2., corners[1][1] - thickness / 2. + offset_y]];
                self.draw_rect_line(p, thickness, color);
            }
        }

        for i in 0..lines {
            
            if 1. + i as f32 <= (lines / 2) as f32 {
                let offset_x = base_offset_x * (i + 1) as f32;
                let offset_y = base_offset_y * (i + 1) as f32;
                let p = [[corners[0][0] + thickness / 2., corners[0][1] + thickness / 2. - offset_y], [corners[0][0] - thickness / 2. + offset_x, corners[0][1] - thickness / 2.]];
                self.draw_rect_line(p, thickness, color);
            } else {
                let offset_x = base_offset_x * (i + 1 - lines / 2) as f32;
                let offset_y = base_offset_y * (i + 1 - lines / 2) as f32;
                let p = [[corners[1][0] + thickness / 2. - offset_x, corners[1][1] + thickness / 2.], [corners[1][0] - thickness / 2., corners[1][1] - thickness / 2. + offset_y]];
                self.draw_rect_line(p, thickness, color);
            }
        }
        
    }
}

#[derive(Debug, Copy, Clone, PartialEq)]
enum Tile {
    Air,
    Ground,
    Building { health: f32, tier: f32 , pressure: f32},
}

impl fmt::Display for Tile {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "{:?}", self)
    }
}

#[derive(Debug)]
struct SaveError(String);

impl std::fmt::Display for SaveError {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        write!(f, "An error ocured while parsing the save file: {}", self.0)
    }
}

impl Error for SaveError {}

#[derive(Debug)]
struct Settings {
    tile_ratio: [i32;2],
}

struct Save<'a> {
    filepath: &'a str,
    date: &'a str,
    ver: &'a str,
    cam_pos: [i32;2],
    selected: [i32;2],
    money: i32,
    settings: Settings,
    grid: HashMap<i32, HashMap<i32, Tile>>,
}
impl Save<'_> {
    fn read(filepath: &str) -> Result<Save, Box<dyn Error>> {
        println!("beggining read at {}", filepath);
        let mut file = File::open(filepath)?;
        let mut contents = String::new();

        file.read_to_string(&mut contents)?;

        let contents = &*Regex::new(r"\s")?.replace_all(&contents, ""); // shadow content for itself with all whitespaces removed
        let contents = &contents.to_lowercase();

        let cam_pos = Regex::new(r"camera_position\{x:([0-9|-]+?),y:([0-9|-]+?),\}")?.captures(contents).unwrap();
        let cam_pos: [i32;2] = [cam_pos[1].parse()?, cam_pos[2].parse()?];

        let selected = Regex::new(r"selected\{x:([0-9|-]+?),y:([0-9|-]+?),\}")?.captures(contents).unwrap();
        let selected: [i32;2] = [selected[1].parse()?, selected[2].parse()?];

        let money = Regex::new(r"money\{([0-9|-]+?)\}")?.captures(contents).unwrap();
        let money: i32 = money[1].parse()?;

        let settings = Regex::new(r"settings\{tile_ratio:\[([0-9|-]+?),([0-9|-]+?)\],\}")?.captures(contents).unwrap();
        let settings: Settings = Settings { tile_ratio: [settings[1].parse()?, settings[2].parse()?] };

        let mut grid: HashMap<i32, HashMap<i32, Tile>> = HashMap::new();

        for t in Regex::new(r"tile\{x:([0-9|-]+?),y:([0-9|-]+?),filling:(.+?),\},")?.captures_iter(contents) {
            let x: i32 = t[1].parse()?;
            let y: i32 = t[2].parse()?;
            let filling = match &t[3] {
                "air" => { Tile::Air },
                "ground" => { Tile::Ground },
                _ if &t[3][..8] == "building" => { 

                    let b =  Regex::new(r"building\{health:([0-9|-|.]+?),tier:([0-9|-]+?)")?.captures(&t[3]).unwrap();

                    let health: f32 = b[1].parse()?;
                    let tier: f32 = b[2].parse()?;

                    Tile::Building {health: health, tier: tier, pressure: 0.}
                },
                _ => { Tile::Air },
            };


            if !grid.contains_key(&x) {
                grid.insert(x, HashMap::from([
                    (y, filling)
                ]));
            } else{
                grid.get_mut(&x)
                    .unwrap()
                    .insert(y, filling);
            }
        }

        for t in Regex::new(r"tile\{x:([0-9|-]+?),y:([0-9|-]+?),filling:([a-z]+?),\},\[repeat\(([0-9]+?)\)times\(([a-z]+?)\)\]")?.captures_iter(contents) {
            let x: i32 = t[1].parse()?;
            let y: i32 = t[2].parse()?;
            let filling = match &t[3] {
                "air" => { Tile::Air },
                "ground" => { Tile::Ground },
                _ if &t[3][..8] == "building" => { 

                    let b =  Regex::new(r"building\{health:([0-9|-|.]+?),tier:([0-9|-]+?)")?.captures(&t[3]).unwrap();

                    let health: f32 = b[1].parse()?;
                    let tier: f32 = b[2].parse()?;

                    Tile::Building {health: health, tier: tier, pressure: 0.}
                },
                _ => { Tile::Air },
            };

            let rep_num: i32 = t[4].parse()?;
            let rep_direction: i32 = match &t[5] {
                "up" => { 1 },
                "down" => { -1},
                _ => { return Err(Box::new(SaveError(format!("incorrect repetition direction in tile at x: {}, y: {}", x, y)))) },
            };

            for i in 0..rep_num{
                grid.get_mut(&x)
                        .unwrap()
                        .insert(y + ((i + 1) * rep_direction), filling);
            }
        }
      
        Ok(Save {
            filepath,
            date: "",
            ver: "",
            cam_pos,
            selected,
            money,
            settings,
            grid,
        })
    }

    fn write(
        filepath: &str,
        date: &str,
        ver: &str,
        cam_pos: [i32;2],
        selected: [i32;2],
        money: &i32,
        settings: Settings,
        grid: &HashMap<i32, HashMap<i32, Tile>>,
     ) -> Result<(), Box<dyn Error>>{

        let mut contents = String::new();
        // generating the contents in memory before pushing it all at once to the save file might suffer from drawbacks 
        // and not behave as desired during abrupt program termination

        contents.push_str(&format!("[Date: {}] \n", date));
        contents.push_str(&format!("[Version: {}] \n\n", ver));

        // values aren't space padded
        contents.push_str(&format!("camera_position {{ \n    x: {},\n    y: {},\n}}\n\n", cam_pos[0], cam_pos[1]));
        contents.push_str(&format!("selected {{ \n    x: {},\n    y: {},\n}}\n\n", selected[0], selected[1]));
        contents.push_str(&format!("money {{ {} }}\n\n", money));
        contents.push_str(&format!("settings {{ \n    tile_ratio: {:?},\n}}\n\n", settings.tile_ratio));
        
        contents.push_str(&format!("grid {{ \n"));


        let mut max_x_len = 1;
        let mut max_y_len = 1;
        let mut max_f_len = 1;
        let mut max_r_len = 1;

        let mut previous: Option<Tile> = None;
        let mut rep_count = 0;

        for col_key in grid.keys() {
            if col_key.to_string().len() > max_x_len {
                max_x_len = col_key.to_string().len();
            }
            let mut row_keys = grid.get(&col_key).unwrap().keys().collect::<Vec<&i32>>();
            row_keys.sort();
            for row_key in &row_keys {
                if row_key.to_string().len() > max_y_len {
                    max_y_len = row_key.to_string().len();
                }
                let filling = *grid.get(col_key).unwrap().get(row_key).unwrap();
                if filling.to_string().len() > max_f_len {
                    if !matches!(filling, Tile::Building{..}) {
                        max_f_len = filling.to_string().len();
                    }
                }

                if previous == Some(filling) && *row_key != row_keys[0] {
                    rep_count += 1;
                } else {
                    rep_count = 1;
                    previous = Some(filling);
                }

                if rep_count.to_string().len() > max_r_len {
                    max_r_len = rep_count.to_string().len();
                }

            }


        }

        let mut previous: Option<Tile> = None;
        let mut rep_count: i32 = 0;
        
        let mut col_keys = grid.keys().collect::<Vec<&i32>>();
        col_keys.sort();
        for col_key in col_keys.iter().rev() {
            let mut row_keys = grid.get(&col_key).unwrap().keys().collect::<Vec<&i32>>();
            row_keys.sort();
            for row_key in row_keys.iter().rev() {
                let mut row_key = **row_key;
                
                let filling = *grid.get(&col_key).unwrap().get(&row_key).unwrap();                      
                
                if previous == Some(filling) && row_key != *row_keys[0] {
                    rep_count += 1;
                } else{    
                    if row_key == *row_keys[0] {
                        rep_count += 1;
                        let row_key = &mut row_key;
                        *row_key -= 1
                    }
                    
                    let x_spacing = " ".repeat(max_x_len - col_key.to_string().len());

                    if rep_count == 1 && previous != None{
                        let y_spacing = " ".repeat(max_y_len - (row_key).to_string().len());
                        let f_spacing;
                        if let Tile::Building { health, tier, .. } = previous.unwrap() {
                            contents.push_str(&format!("    tile {{ x: {}{}, y: {}{}, filling: Building {{ health: {}, tier: {}, }}, }},\n", x_spacing, col_key, y_spacing, row_key + 1, health, tier));
                        } else {
                            f_spacing = " ".repeat(max_f_len - filling.to_string().len());
                            contents.push_str(&format!("    tile {{ x: {}{}, y: {}{}, filling: {}{}, }},\n", x_spacing, col_key, y_spacing, row_key + 1, f_spacing, previous.unwrap(),));
                        }
                    } else if rep_count > 1{
                        let y_spacing = " ".repeat(max_y_len - (row_key + 1).to_string().len());
                        let f_spacing;
                        if !matches!(filling, Tile::Building{..}) {
                            f_spacing = " ".repeat(max_f_len - previous.unwrap().to_string().len());
                        } else {
                            f_spacing = String::new();
                        }
                        let r_padding = " ".repeat(max_r_len - (rep_count - 1).to_string().len());
                        contents.push_str(&format!("    tile {{ x: {}{}, y: {}{}, filling: {}{}, }}, [ Repeat ({}{}) times (  up) ]\n", x_spacing, col_key, y_spacing, row_key + 1, f_spacing, previous.unwrap(), r_padding, rep_count - 1));
                    }


                    
                    previous = Some(filling);
                    rep_count = 1;
                }                
            }
            contents.push_str(&format!("\n"));
            previous = None;
        }
        
        contents.push_str(&format!("}}"));


        fs::write(filepath, &contents)?;

        Ok(())
    }
}


pub async fn run() {
    let event_loop = EventLoop::new();
    let window = WindowBuilder::new().build(&event_loop).unwrap();

    let mut buffers = TwinBuffers::new();
    
    let save = Save::read("saves/base.save").unwrap();
    let mut visible_size = save.settings.tile_ratio;

    let mut grid = save.grid;
    let mut cam_pos = save.cam_pos;
    let mut selected = save.selected;

    fn draw_grid(visible_size: &[i32;2], cam_pos: [i32;2], selected: [i32;2], grid: &HashMap<i32, HashMap<i32, Tile>>, buffers: &mut TwinBuffers) {
        let tile_width = 2. / visible_size[0] as f32;
        let tile_height = 2. / visible_size[1] as f32;

        for col_key in -(visible_size[0] - 1) / 2..(visible_size[0] + 1) / 2 {

            for row_key in -(visible_size[1] - 1) / 2..(visible_size[1] + 1) / 2 {
                let offset_x = tile_width * col_key as f32;
                let offset_y = tile_height * row_key as f32;

                let corner_a = [offset_x - tile_width / 2., offset_y + tile_height / 2.];
                let corner_c = [offset_x + tile_width / 2., offset_y - tile_height / 2.];

                let mut air_color = [0.01, 0.01, 0.01];
                let mut ground_color = [0.2, 0.08, 0.0];

                if col_key == selected[0] - cam_pos[0] && row_key == selected[1] - cam_pos[1] {
                    air_color = [1., 1., 0.];
                    ground_color = [1., 1., 0.];
                }

                let thickness = 0.01;

                match grid.get(&(col_key + cam_pos[0])) {
                    Some(col) => {
                        match col.get(&(row_key + cam_pos[1])) {
                            Some(Tile::Air) => {
                                buffers.draw_box([corner_a, corner_c], thickness, air_color)
                            },
                            Some(Tile::Ground) => {
                                buffers.draw_crossed_box([corner_a, corner_c], thickness, 3, ground_color)
                            },
                            Some(Tile::Building {health, tier, ..}) => {
                                let mut building_color = [0.3 - (((health / tier) - 100.) / 100.), 0.3 + (((health / tier) - 100.) / 200.), 0.3 + (((health / tier) - 100.) / 300.)];
                                if col_key == selected[0] - cam_pos[0] && row_key == selected[1] - cam_pos[1] {
                                    building_color = [1., 1., 0.];
                                }
                                buffers.draw_lined_box([corner_a, corner_c], thickness, *tier as u8, building_color, false)
                            },
                            None => {},
                        }
                    },
                    None => {},
                }
            }
        }
    }

    
    draw_grid(&visible_size, cam_pos, selected, &grid, &mut buffers);

    // reset selected
    
    
    


    //buffers.draw_triangle([[0.0, 0.5], [-0.5, -0.5], [0.5, -0.5]], [1.0, 0.0, 0.0]);
    //buffers.draw_triangle([[0.25, 0.75], [-0.25, 0.0], [0.75, 0.0]], [0.0, 0.0, 1.0]);

    //buffers.draw_rectangle([[-0.9, 0.9], [-0.4, 0.4]], [0.0, 1.0, 0.0]);

    //buffers.draw_box([[-0.5, 0.5], [0.5, -0.5]], 0.05, [1.0, 1.0, 1.0]);

    //buffers.draw_rect_line([[-0.5, 0.25], [0.25, -0.90]], 0.01, [1.0, 1.0, 0.0]);

    //buffers.draw_lined_box([[-1.0, 1.0], [0.0, 0.0]], 0.1, 3, [1.0, 0.0, 1.0], false);

    //buffers.draw_crossed_box([[-1.0, 1.0], [0.0, 0.0]], 0.1, 3, [1.0, 0.0, 1.0]);



    let mut state = State::new(&window, &buffers).await;

    let mut pause = false;
    let mut placing_menu_open = false;
    let mut save_menu_open = false;

    
    fn gravity(grid: &mut HashMap<i32, HashMap<i32, Tile>>) {
        let mut buildings: Vec<(i32, i32)> = Vec::new();
        for col in grid.keys() {
            for t in grid.get(col).unwrap() {
                match t.1 {
                    Tile::Building {..} => {
                        buildings.push( ( *col, *t.0 ) )
                    },
                    _ => {},
                }
            }
        }

        for b in buildings {
            if let Some(filling) = grid.get_mut(&b.0).unwrap().get_mut(&(b.1 - 1)) {
                match filling {
                    Tile::Air => {
                        *grid.get_mut(&b.0).unwrap().get_mut(&(b.1 - 1)).unwrap() = *grid.get(&b.0).unwrap().get(&(b.1)).unwrap();
                        
                        *grid.get_mut(&b.0).unwrap().get_mut(&(b.1)).unwrap() = Tile::Air;

                        // should probably deal with pressure instead of weight
                        if let Tile::Building{ health, tier, .. } = grid.get_mut(&b.0).unwrap().get_mut(&(b.1 - 1)).unwrap() {
                            *health -= *tier * 10. / (*tier * 0.3);
                        }
                    },
                    _ => {},
                }
            }
        }
    }

    // weight_distrib should be event driven instead of per frame
    fn weight_distrib(grid: &mut HashMap<i32, HashMap<i32, Tile>>) {
        let mut buildings: Vec<(i32, i32)> = Vec::new();
        for col in grid.keys() {
            for t in grid.get(col).unwrap() {
                match t.1 {
                    Tile::Building {..} => {
                        buildings.push( ( *col, *t.0 ) )
                    },
                    _ => {},
                }
            }
        }

        for b in buildings {
            let mut other_pressure: f32 = 0.;
            if let Some(Tile::Building {pressure, ..}) = grid.get(&b.0).unwrap().get(&(b.1 + 1)) {
                other_pressure = *pressure;
            }

            if let Tile::Building {tier, pressure, ..} = grid.get_mut(&b.0).unwrap().get_mut(&(b.1)).unwrap() {
                    *pressure = (*tier * 100.) + other_pressure;
            }            
        }
    }

    fn decay(grid: &mut HashMap<i32, HashMap<i32, Tile>>) {
        let mut buildings: Vec<(i32, i32)> = Vec::new();
        for col in grid.keys() {
            for t in grid.get(col).unwrap() {
                match t.1 {
                    Tile::Building {..} => {
                        buildings.push( ( *col, *t.0 ) )
                    },
                    _ => {},
                }
            }
        }

        for b in buildings {
            if let Tile::Building{ health, pressure, .. } = grid.get_mut(&b.0).unwrap().get_mut(&(b.1)).unwrap() {
                *health -= *pressure / 10000.;

                if *health <= 0. {
                    *grid.get_mut(&b.0).unwrap().get_mut(&(b.1)).unwrap() = Tile::Air;
                }
            }
        }
    }

    let mut money: i32 = save.money;

    fn rent_collection(grid: &mut HashMap<i32, HashMap<i32, Tile>>, money: &mut i32) {
        let mut buildings: Vec<(i32, i32)> = Vec::new();
        for col in grid.keys() {
            for t in grid.get(col).unwrap() {
                match t.1 {
                    Tile::Building {..} => {
                        buildings.push( ( *col, *t.0 ) )
                    },
                    _ => {},
                }
            }
        }

        for b in buildings {
            if let Tile::Building{ tier, .. } = grid.get_mut(&b.0).unwrap().get_mut(&(b.1)).unwrap() {
                *money += (*tier * 100. * 1.2 / 60.) as i32;
            }
        }

    }


    fn draw_menu(state: &mut State, buffers: &mut TwinBuffers, grid: &HashMap<i32, HashMap<i32, Tile>>, selected_pos: &[i32;2], money: &i32) {
        let mut selected_filling: Option<&Tile> = None;
        let selected_x: i32 = selected_pos[0];
        let selected_y: i32 = selected_pos[1];
        if let Some(col) = grid.get(&selected_pos[0]) {
            if let Some(tile) = col.get(&selected_pos[1]) {
                selected_filling = Some(tile);
            }
        }

        buffers.draw_rectangle([[0.25, 1.], [1., 0.7]], [0., 0., 0.]);
        buffers.draw_box([[0.25, 1.], [1., 0.7]], 0.01, [1., 1., 1.]);

        state.draw_text([0.275, 0.975], &format!("x: {}, y: {}", selected_x, selected_y), [1., 1., 1., 1.], 0.05);
        if let Some(Tile::Building { health, pressure, .. }) = selected_filling {
            state.draw_text([0.275, 0.925], &format!("filling: Building {{ \n pressure: {}, \n health {}, \n }}", pressure, health), [1., 1., 1., 1.], 0.05);
        } else {
            state.draw_text([0.275, 0.925], &format!("filling: {:?}", selected_filling), [1., 1., 1., 1.], 0.05);
        }


        buffers.draw_rectangle([[-1., 1.], [-0.5, 0.9]], [0., 0., 0.]);
        buffers.draw_box([[-1., 1.], [-0.5, 0.9]], 0.01, [1., 1., 1.]);

        state.draw_text([-0.975, 0.975], &format!("Money: {},", money), [1., 1., 1., 1.], 0.05);
    }

    fn draw_placing_menu(state: &mut State, buffers: &mut TwinBuffers) {
        buffers.draw_rectangle([[0.6, 0.7], [1., 0.375]], [0., 0., 0.]);
        buffers.draw_box([[0.6, 0.7], [1., 0.375]], 0.01, [1., 1., 1.]);

        state.draw_text([0.625, 0.675], &format!("1) Tier 1"), [1., 1., 1., 1.], 0.05);
        state.draw_text([0.625, 0.625], &format!("2) Tier 2"), [1., 1., 1., 1.], 0.05);
        state.draw_text([0.625, 0.575], &format!("3) Tier 3"), [1., 1., 1., 1.], 0.05);
        state.draw_text([0.625, 0.525], &format!("4) Tier 4"), [1., 1., 1., 1.], 0.05);
        state.draw_text([0.625, 0.475], &format!("5) Tier 5"), [1., 1., 1., 1.], 0.05);
    }

    fn place_building(tier: i32, selected: [i32;2], grid: &mut HashMap<i32, HashMap<i32, Tile>>, money: &mut i32, pause: bool) {
        let can_place = if let Some(col) = grid.get(&selected[0]) {
            match col.get(&(selected[1] - 1)) {
                Some(&Tile::Ground) => { true },
                Some(&Tile::Building {..}) => { true },
                _ => { false }
            }
        } else {
            false
        };
        


        if let Some(col) = grid.get_mut(&selected[0]) {
            if let Some(tile) = col.get_mut(&selected[1]) {
                if *tile == Tile::Air && can_place && *money >= tier * 10000 {
                    *money -= tier * 10000;
                    *tile = Tile::Building { health: tier as f32 * 100., tier: tier as f32, pressure: 0. };
                }
            }
        }
    }

    fn draw_save_menu(state: &mut State, buffers: &mut TwinBuffers) {
        buffers.draw_rectangle([[-0.925, 0.825], [0.175, -0.875]], [0., 0., 0.]);
        buffers.draw_box([[-0.925, 0.825], [0.175, -0.875]], 0.01, [1., 1., 1.]);


        let saves = fs::read_dir("./saves").unwrap();

        let mut i = 0;
        let mut y = 0.8;
        for save in saves {
            i += 1;
            state.draw_text([-0.9, y], &format!("{}) {}", i, &(save.unwrap().path().to_str().unwrap()[8..])), [1., 1., 1., 1.], 0.1);
            y -= 0.15;
        }

    }



    event_loop.run(move |event, _, control_flow| match event {
        Event::WindowEvent {
            event,
            ..
        } => match event {
            WindowEvent::CloseRequested | WindowEvent::KeyboardInput {
                input:
                    KeyboardInput {
                        virtual_keycode: Some(VirtualKeyCode::Escape),
                        ..
                    },
                ..
            } => *control_flow = ControlFlow::Exit,

            WindowEvent::KeyboardInput {
                input:
                    KeyboardInput {
                        virtual_keycode: Some(VirtualKeyCode::E),
                        state: ElementState::Pressed,
                        ..
                    },
                ..
            } => {
                visible_size = [visible_size[0] + 2, visible_size[1] + 2];
                window.request_redraw();
            },
            WindowEvent::KeyboardInput {
                input:
                    KeyboardInput {
                        virtual_keycode: Some(VirtualKeyCode::Q),
                        state: ElementState::Pressed,
                        ..
                    },
                ..
            } => {
                visible_size = [visible_size[0] - 2, visible_size[1] - 2];
                window.request_redraw();
            },

            WindowEvent::KeyboardInput {
                input:
                    KeyboardInput {
                        virtual_keycode: Some(VirtualKeyCode::W),
                        state: ElementState::Pressed,
                        ..
                    },
                ..
            } => {
                cam_pos[1] += 1;
                window.request_redraw();
            },
            WindowEvent::KeyboardInput {
                input:
                    KeyboardInput {
                        virtual_keycode: Some(VirtualKeyCode::S),
                        state: ElementState::Pressed,
                        ..
                    },
                ..
            } => {
                cam_pos[1] -= 1;
                window.request_redraw();
            },
            WindowEvent::KeyboardInput {
                input:
                    KeyboardInput {
                        virtual_keycode: Some(VirtualKeyCode::D),
                        state: ElementState::Pressed,
                        ..
                    },
                ..
            } => {
                cam_pos[0] += 1;
                window.request_redraw();
            },
            WindowEvent::KeyboardInput {
                input:
                    KeyboardInput {
                        virtual_keycode: Some(VirtualKeyCode::A),
                        state: ElementState::Pressed,
                        ..
                    },
                ..
            } => {
                cam_pos[0] -= 1;
                window.request_redraw();
            },

            WindowEvent::KeyboardInput {
                input:
                    KeyboardInput {
                        virtual_keycode: Some(VirtualKeyCode::Up),
                        state: ElementState::Pressed,
                        ..
                    },
                ..
            } => {
                selected[1] += 1;
                window.request_redraw();
            },
            WindowEvent::KeyboardInput {
                input:
                    KeyboardInput {
                        virtual_keycode: Some(VirtualKeyCode::Down),
                        state: ElementState::Pressed,
                        ..
                    },
                ..
            } => {
                selected[1] -= 1;
                window.request_redraw();
            },
            WindowEvent::KeyboardInput {
                input:
                    KeyboardInput {
                        virtual_keycode: Some(VirtualKeyCode::Right),
                        state: ElementState::Pressed,
                        ..
                    },
                ..
            } => {
                selected[0] += 1;
                window.request_redraw();
            },
            WindowEvent::KeyboardInput {
                input:
                    KeyboardInput {
                        virtual_keycode: Some(VirtualKeyCode::Left),
                        state: ElementState::Pressed,
                        ..
                    },
                ..
            } => {
                selected[0] -= 1;
                window.request_redraw();
            },

            WindowEvent::KeyboardInput {
                input:
                    KeyboardInput {
                        virtual_keycode: Some(VirtualKeyCode::Z),
                        state: ElementState::Pressed,
                        ..
                    },
                ..
            } => {
                if pause {
                    pause = false
                } else {
                    pause = true
                }
            },

            WindowEvent::KeyboardInput {
                input:
                    KeyboardInput {
                        virtual_keycode: Some(VirtualKeyCode::X),
                        state: ElementState::Pressed,
                        ..
                    },
                ..
            } => {
                let mut buildings: Vec<(i32, i32)> = Vec::new();
                for col in grid.keys() {
                    for t in grid.get(col).unwrap() {
                        match t.1 {
                            Tile::Building {..} => {
                                buildings.push( ( *col, *t.0 ) )
                            },
                            _ => {},
                        }
                    }
                }
                if !pause {
                    for b in buildings {
                        if let Some(filling) = grid.get_mut(&b.0).unwrap().get_mut(&(b.1)) {
                            match filling {
                                Tile::Building {health, ..} => {
                                    *health *= 0.1
                                },
                                _ => {},
                            }
                        }
                    }
                }
                
                
                window.request_redraw();
            },

            // save
            WindowEvent::KeyboardInput {
                input:
                    KeyboardInput {
                        // should be Ctrl + S
                        virtual_keycode: Some(VirtualKeyCode::C),
                        state: ElementState::Pressed,
                        ..
                    },
                ..
            } => {
                let mut i = 0;
                for _save in fs::read_dir("./saves").unwrap() {
                    i += 1;
                }

                let filepath = &format!("saves/test{}.save", i + 1);            
                let date = &Local::now().to_rfc3339_opts(SecondsFormat::Millis, false);
                let ver = env!("CARGO_PKG_VERSION");

                Save::write(
                    filepath,
                    date,
                    ver,
                    cam_pos,
                    selected,
                    &money,
                    Settings {tile_ratio: visible_size},
                    &grid,
                ).unwrap();
            },

            // delete saves
            WindowEvent::KeyboardInput {
                input:
                    KeyboardInput {
                        virtual_keycode: Some(VirtualKeyCode::V),
                        state: ElementState::Pressed,
                        ..
                    },
                ..
            } => {
                for save in fs::read_dir("./saves").unwrap() {
                    let path = save.unwrap().path();
                    if path != path::PathBuf::from("./saves/base.save") {
                        fs::remove_file(path).unwrap();
                    }
                }
            },
        
            WindowEvent::KeyboardInput {
                input:
                    KeyboardInput {
                        virtual_keycode: Some(VirtualKeyCode::Key1),
                        state: ElementState::Pressed,
                        ..
                    },
                ..
            } => {
                if save_menu_open {
                    let mut save_paths = Vec::new();
                    for save in fs::read_dir("./saves").unwrap() {
                        let path = save.unwrap().path();
                        save_paths.push(path);
                    }
                    let save = Save::read(&save_paths[0].to_str().unwrap()[2..]).unwrap();

                    visible_size = save.settings.tile_ratio;
                    grid = save.grid;
                    cam_pos = save.cam_pos;
                    selected = save.selected;
                    money = save.money;
                    
                } else if placing_menu_open {
                    place_building(1, selected, &mut grid, &mut money, pause);
                }

                window.request_redraw();
            },
            WindowEvent::KeyboardInput {
                input:
                    KeyboardInput {
                        virtual_keycode: Some(VirtualKeyCode::Key2),
                        state: ElementState::Pressed,
                        ..
                    },
                ..
            }  => {
                if save_menu_open {
                    let mut save_paths = Vec::new();
                    for save in fs::read_dir("./saves").unwrap() {
                        let path = save.unwrap().path();
                        save_paths.push(path);
                    }
                    let save = Save::read(&save_paths[1].to_str().unwrap()[2..]).unwrap();

                    visible_size = save.settings.tile_ratio;
                    grid = save.grid;
                    cam_pos = save.cam_pos;
                    selected = save.selected;
                    money = save.money;
                    
                } else if placing_menu_open {
                    place_building(2, selected, &mut grid, &mut money, pause);
                }

                window.request_redraw();
            },
            WindowEvent::KeyboardInput {
                input:
                    KeyboardInput {
                        virtual_keycode: Some(VirtualKeyCode::Key3),
                        state: ElementState::Pressed,
                        ..
                    },
                ..
            } => {
                if save_menu_open {
                    let mut save_paths = Vec::new();
                    for save in fs::read_dir("./saves").unwrap() {
                        let path = save.unwrap().path();
                        save_paths.push(path);
                    }
                    let save = Save::read(&save_paths[2].to_str().unwrap()[2..]).unwrap();

                    visible_size = save.settings.tile_ratio;
                    grid = save.grid;
                    cam_pos = save.cam_pos;
                    selected = save.selected;
                    money = save.money;
                    
                } else if placing_menu_open {
                    place_building(3, selected, &mut grid, &mut money, pause);
                }

                window.request_redraw();
            },
            WindowEvent::KeyboardInput {
                input:
                    KeyboardInput {
                        virtual_keycode: Some(VirtualKeyCode::Key4),
                        state: ElementState::Pressed,
                        ..
                    },
                ..
            } => {
                if save_menu_open {
                    let mut save_paths = Vec::new();
                    for save in fs::read_dir("./saves").unwrap() {
                        let path = save.unwrap().path();
                        save_paths.push(path);
                    }
                    let save = Save::read(&save_paths[3].to_str().unwrap()[2..]).unwrap();

                    visible_size = save.settings.tile_ratio;
                    grid = save.grid;
                    cam_pos = save.cam_pos;
                    selected = save.selected;
                    money = save.money;
                    
                } else if placing_menu_open {
                    place_building(4, selected, &mut grid, &mut money, pause);
                }

                window.request_redraw();
            },
            WindowEvent::KeyboardInput {
                input:
                    KeyboardInput {
                        virtual_keycode: Some(VirtualKeyCode::Key5),
                        state: ElementState::Pressed,
                        ..
                    },
                ..
            } => {
                if save_menu_open {
                    let mut save_paths = Vec::new();
                    for save in fs::read_dir("./saves").unwrap() {
                        let path = save.unwrap().path();
                        save_paths.push(path);
                    }
                    let save = Save::read(&save_paths[4].to_str().unwrap()[2..]).unwrap();

                    visible_size = save.settings.tile_ratio;
                    grid = save.grid;
                    cam_pos = save.cam_pos;
                    selected = save.selected;
                    money = save.money;
                    
                } else if placing_menu_open {
                    place_building(5, selected, &mut grid, &mut money, pause);
                }

                window.request_redraw();
            },

            // open place menu
            WindowEvent::KeyboardInput {
                input:
                    KeyboardInput {
                        virtual_keycode: Some(VirtualKeyCode::Return),
                        state: ElementState::Pressed,
                        ..
                    },
                ..
            } => {
                if placing_menu_open {
                    placing_menu_open = false;
                } else {
                    placing_menu_open = true;
                }
                window.request_redraw();
            },

            // open save menu
            WindowEvent::KeyboardInput {
                input:
                    KeyboardInput {
                        virtual_keycode: Some(VirtualKeyCode::B),
                        state: ElementState::Pressed,
                        ..
                    },
                ..
            } => {
                if save_menu_open {
                    save_menu_open = false;
                    pause = false;
                } else {
                    save_menu_open = true;
                    pause = true;
                }
                window.request_redraw();
            },

            WindowEvent::Resized(physical_size) => {
                state.resize(physical_size);
            },
            WindowEvent::ScaleFactorChanged { new_inner_size, .. } => {
                state.resize(*new_inner_size);
            },
            _ => {},
        },
        Event::MainEventsCleared => {
            window.request_redraw();
        },
        Event::RedrawRequested(_) => {
            buffers.vertices = Vec::new();
            buffers.indices = Vec::new();
            weight_distrib(&mut grid);
            if !pause {
                gravity(&mut grid);
                decay(&mut grid);
                rent_collection(&mut grid, &mut money);
            }
            draw_grid(&visible_size, cam_pos, selected, &grid, &mut buffers);
            if placing_menu_open {
                draw_placing_menu(&mut state, &mut buffers);
            }
            if save_menu_open {
                draw_save_menu(&mut state, &mut buffers);
            }
            draw_menu(&mut state, &mut buffers, &grid, &selected, &money);
            state.update_buffers(&mut buffers);

            match state.render(&buffers.indices) {
                Ok(_) => {}
                Err(wgpu::SurfaceError::Lost) => state.resize(state.size),
                Err(wgpu::SurfaceError::OutOfMemory) => *control_flow = ControlFlow::Exit,
                Err(e) => eprintln!("{:?}", e),
            }
        }
        _ => {}
    });
}

// wgpu setup boilerplate

struct State {
    size: winit::dpi::PhysicalSize<u32>,
    surface: wgpu::Surface,
    device: wgpu::Device,
    queue: wgpu::Queue,
    config: wgpu::SurfaceConfiguration,

    render_pipeline: wgpu::RenderPipeline,

    vertex_buffer: wgpu::Buffer,
    index_buffer: wgpu::Buffer,

    staging_belt: wgpu::util::StagingBelt,
    glyph_brush: wgpu_glyph::GlyphBrush<()>,
}

impl State {
    async fn new(window: &Window, buffers: &TwinBuffers) -> Self {

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
            None, // Trace path
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

        let shader = device.create_shader_module(wgpu::include_wgsl!("./engine/shader.wgsl"));
        
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
        
        let vertex_buffer = device.create_buffer_init(
            &wgpu::util::BufferInitDescriptor {
                label: None,
                contents: bytemuck::cast_slice(&buffers.vertices),
                usage: wgpu::BufferUsages::VERTEX,
            }
        );

        let index_buffer = device.create_buffer_init(
            &wgpu::util::BufferInitDescriptor {
                label: None,
                contents: bytemuck::cast_slice(&buffers.indices),
                usage: wgpu::BufferUsages::INDEX,
            }
        );


        let staging_belt = wgpu::util::StagingBelt::new(1024);

        let inconsolata = ab_glyph::FontArc::try_from_slice(include_bytes!(
            "Inconsolata-Regular.ttf"
        )).unwrap();
    
        let glyph_brush = GlyphBrushBuilder::using_font(inconsolata)
            .build(&device, wgpu::TextureFormat::Bgra8UnormSrgb);

        State {
            size,
            surface,
            device,
            queue,
            config,

            render_pipeline,

            vertex_buffer,
            index_buffer,

            staging_belt,
            glyph_brush,
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

    fn update_buffers(&mut self, buffers: &mut TwinBuffers) {
        self.vertex_buffer = self.device.create_buffer_init(
            &wgpu::util::BufferInitDescriptor {
                label: None,
                contents: bytemuck::cast_slice(&buffers.vertices),
                usage: wgpu::BufferUsages::VERTEX,
            }
        );

        self.index_buffer = self.device.create_buffer_init(
            &wgpu::util::BufferInitDescriptor {
                label: None,
                contents: bytemuck::cast_slice(&buffers.indices),
                usage: wgpu::BufferUsages::INDEX,
            }
        );
    }

    fn render(&mut self, indices: &Vec<u16>) -> Result<(), wgpu::SurfaceError> {
        let output = self.surface.get_current_texture()?;

        let view = output.texture.create_view(&wgpu::TextureViewDescriptor::default());

        let mut encoder = self.device.create_command_encoder(&wgpu::CommandEncoderDescriptor { label: None, });

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
        render_pass.set_vertex_buffer(0, self.vertex_buffer.slice(..));
        render_pass.set_index_buffer(self.index_buffer.slice(..), wgpu::IndexFormat::Uint16);
        render_pass.draw_indexed(0..indices.len() as u32, 0, 0..1);

        drop(render_pass);

        self.glyph_brush.draw_queued(
            &self.device,
            &mut self.staging_belt,
            &mut encoder,
            &view,
            self.size.width,
            self.size.height,
        ).unwrap();

        self.staging_belt.finish();
    
        self.queue.submit(std::iter::once(encoder.finish()));
        output.present();
    
        Ok(())
    }

    fn draw_text(&mut self, position: [f32;2], text: &str, color: [f32;4], scale: f32) {
        let width = self.size.width as f32;
        let height = self.size.height as f32;

        self.glyph_brush.queue(Section {
            screen_position: ((width / 2.) + (position[0] / 2.) * width + 1., (height / 2.) + ((position[1] * -1.) / 2.) * height),
            text: vec![Text::new(text)
                .with_color(color)
                .with_scale(ab_glyph::PxScale {x: (scale / 2.) * width, y: (scale / 2.) * height})],
            ..Section::default()
        });
    }
}