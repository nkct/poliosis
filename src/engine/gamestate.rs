use crate::engine::grid::{ Grid, Coord };
use crate::engine::settings::Settings;

struct GameState {
    grid: Grid,
    cam_pos: Coord,
    selected: Coord,
    money: i128,
    settings: Settings,
}