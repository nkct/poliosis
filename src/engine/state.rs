use std::collections::HashMap;
use std::fmt;

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

struct Coord {
    x: i32,
    y: i32,
}

#[derive(Debug, PartialEq)]
struct Grid {
    grid: HashMap<i32, HashMap<i32, Tile>>,
}
impl Grid {
    fn new(tiles: Vec<(Coord, Tile)>) -> Grid {
        let mut grid = HashMap::new();

        for (coord, tile) in tiles {
            let x = coord.x;
            let y = coord.y;

            if !grid.contains_key(&x) {
                grid.insert(x, HashMap::from([
                    (y, tile)
                ]));
            } else{
                grid.get_mut(&x)
                    .unwrap()
                    .insert(y, tile);
            }
        }
        
        return Grid{grid};
    }

    fn get(&self, coord: Coord) -> Option<&Tile> {
        if let Some(col) = self.grid.get(&coord.x) {
            return col.get(&coord.y);
        } else {
            return None;
        }
    }
    fn get_mut(&mut self, coord: Coord) -> Option<&mut Tile> {
        if let Some(col) = self.grid.get_mut(&coord.x) {
            return col.get_mut(&coord.y);
        } else {
            return None;
        }
    }

    fn get_matching(&self, pattern: Tile) -> Vec<&Tile> {
        todo!();
    }
    fn get_matching_mut(&self, pattern: Tile) -> Vec<&mut Tile> {
        todo!();
    }
}



// ----- TESTS -----
#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_grid_new() {
        let new_grid = Grid::new(Vec::from([
            (Coord{x: 0, y: 0}, Tile::Air),
            (Coord{x: 0, y: 1}, Tile::Air),

            (Coord{x: 1, y: 0}, Tile::Air),
            (Coord{x: 1, y: 1}, Tile::Air),
        ]));

        assert_eq!(new_grid.grid, HashMap::from([
            (0, HashMap::from([
                (0, Tile::Air),
                (1, Tile::Air),
            ])),
            (1, HashMap::from([
                (0, Tile::Air),
                (1, Tile::Air),
            ])),
        ]));
    }

    #[test]
    fn test_grid_get() {
        let test_grid = Grid::new(Vec::from([
            (Coord{x: 0, y: 0}, Tile::Air),
            (Coord{x: 0, y: 1}, Tile::Air),

            (Coord{x: 1, y: 0}, Tile::Air),
            (Coord{x: 1, y: 1}, Tile::Air),
        ]));

        assert_eq!(
            test_grid.get(Coord { x: 0, y: 0 }),
            Some(&Tile::Air)
        )
    }

    #[test]
    fn test_grid_get_mut() {
        let mut test_grid = Grid::new(Vec::from([
            (Coord{x: 0, y: 0}, Tile::Air),
            (Coord{x: 0, y: 1}, Tile::Air),

            (Coord{x: 1, y: 0}, Tile::Air),
            (Coord{x: 1, y: 1}, Tile::Air),
        ]));

        assert_eq!(
            test_grid.get_mut(Coord { x: 0, y: 0 }),
            Some(&mut Tile::Air)
        )
    }
}