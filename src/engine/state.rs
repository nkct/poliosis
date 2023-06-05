use std::collections::HashMap;
use std::fmt;
use std::cmp::{ Ord, Ordering };

#[allow(dead_code)]
#[derive(Debug, Copy, Clone, PartialOrd)]
enum Tile {
    Air,
    Ground,
    Building { health: f32, tier: f32 , pressure: f32},
}
impl PartialEq for Tile {
    fn eq(&self, other: &Self) -> bool {
        match self {
            Tile::Air => { matches!(other, Tile::Air) },
            Tile::Ground => { matches!(other, Tile::Ground) },
            Tile::Building{ .. } => { matches!(other, Tile::Building { .. }) },
        }
    }
}
impl Eq for Tile {}
impl Ord for Tile {
    fn cmp(&self, other: &Self) -> Ordering {
        match self {
            Tile::Air => {
                match other {
                    Tile::Air => Ordering::Equal,
                    Tile::Ground => Ordering::Greater,
                    Tile::Building{..} => Ordering::Greater,
                }
            }
            Tile::Ground => {
                match other {
                    Tile::Air => Ordering::Less,
                    Tile::Ground => Ordering::Equal,
                    Tile::Building{..} => Ordering::Greater,
                }
            }
            Tile::Building { .. } => {
                match other {
                    Tile::Air => Ordering::Less,
                    Tile::Ground => Ordering::Less,
                    Tile::Building{..} => Ordering::Equal,
                }
            }
        }
    }
}

impl fmt::Display for Tile {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "{:?}", self)
    }
}

#[derive(Debug, Copy, Clone, PartialEq, Eq, PartialOrd, Ord, Hash)]
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
#[allow(dead_code)]
impl Coord {
    fn origin() -> Coord {
        return Coord{ x: 0, y: 0 };
    }

    fn spread(&self, other: Coord) -> Vec<Coord> {
        let mut output: Vec<Coord> = Vec::new();

        for x in self.x..=other.x {
            for y in self.y..=other.y {
                output.push(Coord { x, y })
            }
        }

        return output;
    }
}

#[derive(Debug, PartialEq)]
struct Grid {
    grid: HashMap<i32, HashMap<i32, Tile>>,
}

macro_rules! get_matching {
    ($collection: expr, $pattern: pat) => {
        {
            let mut matched = Vec::new(); 
            for (coord, tile) in $collection {
                if matches!((coord, tile), $pattern) {
                    matched.push((coord, tile));
                }
            }
            matched
        }
    };
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
    pub fn new<C: Into<Coord>>(tiles: Vec<(C, Tile)>) -> Grid {
        let mut grid = HashMap::new();

        for (coord_like, tile) in tiles {
            let coord: Coord = coord_like.into();
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

    pub fn get<C: Into<Coord>>(&self, coord_like: C) -> Option<&Tile> {
        let coord: Coord = coord_like.into();

        if let Some(col) = self.grid.get(&coord.x) {
            return col.get(&coord.y);
        } else {
            return None;
        }
    }
    pub fn get_mut<C: Into<Coord>>(&mut self, coord_like: C) -> Option<&mut Tile> {
        let coord: Coord = coord_like.into();

        if let Some(col) = self.grid.get_mut(&coord.x) {
            return col.get_mut(&coord.y);
        } else {
            return None;
        }
    }

    pub fn insert<C: Into<Coord>>(&mut self, coord_like: C, tile: Tile) {
        let coord: Coord = coord_like.into();
        let x = coord.x;
        let y = coord.y;

        if !self.grid.contains_key(&x) {
            self.grid.insert(x, HashMap::from([
                (y, tile)
            ]));
        } else{
            self.grid.get_mut(&x)
                .unwrap()
                .insert(y, tile);
        }
    }
}



// ----- TESTS -----
#[cfg(test)]
mod tests {
    use super::*;

    // ----- TEST MACROS -----
    #[test]
    fn test_get_matching() {
        let test_grid = Grid::new(Vec::from([
            (Coord{x: 0, y: 0}, Tile::Air),
            (Coord{x: 0, y: 1}, Tile::Air),
            (Coord{x: 0, y: 2}, Tile::Air),

            (Coord{x: 1, y: 0}, Tile::Ground),
            (Coord{x: 1, y: 1}, Tile::Ground),
            (Coord{x: 1, y: 2}, Tile::Ground),

            (Coord{x: 2, y: 0}, Tile::Air),
            (Coord{x: 2, y: 1}, Tile::Ground),
            (Coord{x: 2, y: 2}, Tile::Air),
        ]));

        let matched = get_matching!(test_grid, (Coord { x: _, y: 1}, Tile::Ground));
        assert_eq!(matched, Vec::from([
            (Coord{x: 1, y: 1}, Tile::Ground),
            (Coord{x: 2, y: 1}, Tile::Ground),
        ]))
    }

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
    fn test_coord_ord() {
        assert_eq!(Coord{ x: 1, y: 1 }.cmp(&Coord{ x: 0, y: 0 }), Ordering::Greater);
        assert_eq!(Coord{ x: 1, y: 0 }.cmp(&Coord{ x: 0, y: 0 }), Ordering::Greater);
        assert_eq!(Coord{ x: 1, y: 0 }.cmp(&Coord{ x: 0, y: 1 }), Ordering::Greater);

        assert_eq!(Coord{ x: 0, y: 0 }.cmp(&Coord{ x: 1, y: 1 }), Ordering::Less);
        assert_eq!(Coord{ x: 0, y: 0 }.cmp(&Coord{ x: 1, y: 0 }), Ordering::Less);
        assert_eq!(Coord{ x: 0, y: 1 }.cmp(&Coord{ x: 1, y: 1 }), Ordering::Less);

        assert_eq!(Coord{ x: 0, y: 0 }.cmp(&Coord{ x: 0, y: 0 }), Ordering::Equal);
        assert_eq!(Coord{ x: 1, y: 1 }.cmp(&Coord{ x: 1, y: 1 }), Ordering::Equal);
        assert_eq!(Coord{ x: 1, y: 0 }.cmp(&Coord{ x: 1, y: 0 }), Ordering::Equal);
        assert_eq!(Coord{ x: 0, y: 1 }.cmp(&Coord{ x: 0, y: 1 }), Ordering::Equal);
    }

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

    #[test]
    fn test_coord_origin() {
        assert_eq!(Coord::origin(), Coord{ x: 0, y: 0})
    }

    #[test]
    fn test_coord_spread() {
        assert_eq!(
            Coord::origin().spread(Coord { x: 2, y: 2 }),
            vec![
                Coord{ x: 0, y: 0},
                Coord{ x: 0, y: 1},
                Coord{ x: 0, y: 2},

                Coord{ x: 1, y: 0},
                Coord{ x: 1, y: 1},
                Coord{ x: 1, y: 2},

                Coord{ x: 2, y: 0},
                Coord{ x: 2, y: 1},
                Coord{ x: 2, y: 2},
            ]
        )
    }


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

    #[test]
    fn test_grid_insert() {
        let mut test_grid = create_test_grid();
        test_grid.insert((2, 0), Tile::Ground);
        
        assert_eq!(
            test_grid,
            Grid::new(Vec::from([
                (Coord{x: 0, y: 0}, Tile::Air),
                (Coord{x: 0, y: 1}, Tile::Air),
    
                (Coord{x: 1, y: 0}, Tile::Air),
                (Coord{x: 1, y: 1}, Tile::Air),
                
                (Coord{x: 2, y: 0}, Tile::Ground),
            ]))
        )
    }
}