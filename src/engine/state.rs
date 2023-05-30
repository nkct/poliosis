use std::collections::HashMap;
use std::fmt;

#[allow(dead_code)]
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

#[derive(Debug, Eq, PartialEq, Hash)]
struct Coord {
    x: i32,
    y: i32,
}
impl From<[i32;2]> for Coord {
    fn from(arr: [i32;2]) -> Self {
        return Coord{ x: arr[0], y: arr[1] };
    }
}
impl From<(i32, i32)> for Coord {
    fn from(tup: (i32, i32)) -> Self {
        return Coord{ x: tup.0, y: tup.1};
    }
}

#[derive(Debug, PartialEq)]
struct Grid {
    grid: HashMap<i32, HashMap<i32, Tile>>,
}
impl IntoIterator for Grid {
    type Item = (Coord, Tile);
    type IntoIter = std::collections::hash_map::IntoIter<Coord, Tile>;

    fn into_iter(self) -> Self::IntoIter {
        return self.flatten().into_iter();
    }
}
#[allow(dead_code)]
impl Grid {
    pub fn new<T: Into<Coord>>(tiles: Vec<(T, Tile)>) -> Grid {
        let mut grid = HashMap::new();

        for (raw_coord, tile) in tiles {
            let coord: Coord = raw_coord.into();
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

    pub fn flatten(&self) -> HashMap<Coord, Tile> {
        let mut flat_grid: HashMap<Coord, Tile> = HashMap::new();

        for (x, col) in &self.grid {
            for (y, tile) in col {
                flat_grid.insert(Coord { x: *x, y: *y }, *tile);
            }
        }

        return flat_grid;
    }

    pub fn get<T: Into<Coord>>(&self, raw_coord: T) -> Option<&Tile> {
        let coord: Coord = raw_coord.into();

        if let Some(col) = self.grid.get(&coord.x) {
            return col.get(&coord.y);
        } else {
            return None;
        }
    }
    pub fn get_mut<T: Into<Coord>>(&mut self, raw_coord: T) -> Option<&mut Tile> {
        let coord: Coord = raw_coord.into();

        if let Some(col) = self.grid.get_mut(&coord.x) {
            return col.get_mut(&coord.y);
        } else {
            return None;
        }
    }

    pub fn get_matching(&self, pattern: Tile) -> Vec<&Tile> {
        todo!();
    }
    pub fn get_matching_mut(&self, pattern: Tile) -> Vec<&mut Tile> {
        todo!();
    }
}



// ----- TESTS -----
#[cfg(test)]
mod tests {
    use super::*;

    // ----- HELPER FUNCTIONS -----
    fn create_test_grid() -> Grid {
        return Grid::new(Vec::from([
            (Coord{x: 0, y: 0}, Tile::Air),
            (Coord{x: 0, y: 1}, Tile::Air),

            (Coord{x: 1, y: 0}, Tile::Air),
            (Coord{x: 1, y: 1}, Tile::Air),
        ]))
    }

    // ----- COORD TESTS -----
    #[test]
    fn test_coord_from() {
        assert_eq!(Coord::from([3, 5]), Coord{ x: 3, y: 5 }, "ERROR: Failed assertion while converting from [i32;2] to Coord.");
        assert_eq!(Coord::from((3, 5)), Coord{ x: 3, y: 5 }, "ERROR: Failed assertion while converting from (i32, i32) to Coord.");
    }

    /* disabled because of problems with type coercion, unused anyway, not a priority
    #[test]
    fn test_coord_into() {
        let test_coord = Coord{ x: 3, y: 5 };
        let into_array: [i32;2] = test_coord.into();
        let into_tuple: (i32, i32) = test_coord.into();
        assert_eq!(into_array, [3, 5], "ERROR: Failed assertion while converting from Coord to [i32;2].");
        assert_eq!(into_tuple, (3, 5), "ERROR: Failed assertion while converting from Coord to (i32, i32).");
    }
    */


    // ----- GRID TESTS -----
    #[test]
    fn test_grid_intoiter() {
        let test_grid = create_test_grid();

        for (_, _) in test_grid {
            assert!(true)
        }
    }

    #[test]
    fn test_grid_flatten() {
        let test_grid = create_test_grid();
        
        assert_eq!(test_grid.flatten(), HashMap::from([
            (Coord{x: 0, y: 0}, Tile::Air),
            (Coord{x: 0, y: 1}, Tile::Air),

            (Coord{x: 1, y: 0}, Tile::Air),
            (Coord{x: 1, y: 1}, Tile::Air),
        ]))
    }

    #[test]
    fn test_grid_new() {
        let test_grid = create_test_grid();

        assert_eq!(test_grid.grid, HashMap::from([
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
        let test_grid = create_test_grid();

        assert_eq!(
            test_grid.get(Coord { x: 0, y: 0 }),
            Some(&Tile::Air)
        )
    }

    #[test]
    fn test_grid_get_mut() {
        let mut test_grid = create_test_grid();

        assert_eq!(
            test_grid.get_mut(Coord { x: 0, y: 0 }),
            Some(&mut Tile::Air)
        )
    }
}