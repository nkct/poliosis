use std::collections::HashMap;
use std::fmt;
#[allow(unused_imports)]
use std::cmp::{ Ord, Ordering };

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
    const ORIGIN: Coord = Coord{ x: 0, y: 0 };

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

#[allow(unused_macros)]
macro_rules! get_matching {
    ($collection: expr, $pattern: pat) => {
        {
            let mut matched = Vec::new(); 
            for item in $collection {
                if matches!(item, $pattern) {
                    matched.push(item);
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
impl<'a> IntoIterator for &'a Grid {
    type Item = (Coord, &'a Tile);
    type IntoIter = std::vec::IntoIter<Self::Item>;

    fn into_iter(self) -> Self::IntoIter {
        return self.to_ref_vec().into_iter();
    }
}
impl<'a> IntoIterator for &'a mut Grid {
    type Item = (Coord, &'a mut Tile);
    type IntoIter = std::vec::IntoIter<Self::Item>;

    fn into_iter(self) -> Self::IntoIter {
        return self.to_mut_ref_vec().into_iter();
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

    pub fn to_ref_vec(&self) -> Vec<(Coord, &Tile)> {
        let mut ref_vec: Vec<(Coord, &Tile)> = Vec::new();

        for (x, col) in &self.grid {
            for (y, tile) in col {
                ref_vec.push((Coord { x: *x, y: *y }, tile));
            }
        }

        ref_vec.sort_unstable_by( |a, b| a.0.cmp(&b.0) );
        return ref_vec;
    }

    pub fn to_mut_ref_vec(&mut self) -> Vec<(Coord, &mut Tile)> {
        let mut ref_vec: Vec<(Coord, &mut Tile)> = Vec::new();

        for (&x, col) in &mut self.grid {
            for (&y, tile) in col {
                ref_vec.push((Coord { x, y }, tile));
            }
        }

        ref_vec.sort_unstable_by( |a, b| a.0.cmp(&b.0) );
        return ref_vec;
    }

}



// ----- TESTS -----
#[cfg(test)]
mod tests {
    use super::*;

    // ----- TEST MACROS -----
    #[test]
    fn test_get_matching() {
        let mut test_grid = Grid::new(Vec::from([
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

        let matched = get_matching!(&test_grid, (Coord { x: _, y: 1}, Tile::Ground));
        assert_eq!(matched, Vec::from([
            (Coord{x: 1, y: 1}, &Tile::Ground),
            (Coord{x: 2, y: 1}, &Tile::Ground),
        ]));

        let matched = get_matching!(&mut test_grid, (Coord { x: _, y: 1}, Tile::Ground));
        assert_eq!(matched, Vec::from([
            (Coord{x: 1, y: 1}, &mut Tile::Ground),
            (Coord{x: 2, y: 1}, &mut Tile::Ground),
        ]));

        let matched = get_matching!(test_grid, (Coord { x: _, y: 1}, Tile::Ground));
        assert_eq!(matched, Vec::from([
            (Coord{x: 1, y: 1}, Tile::Ground),
            (Coord{x: 2, y: 1}, Tile::Ground),
        ]));
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

    #[test]
    fn test_coord_origin() {
        assert_eq!(Coord::ORIGIN, Coord{ x: 0, y: 0})
    }

    #[test]
    fn test_coord_spread() {
        assert_eq!(
            Coord::ORIGIN.spread(Coord { x: 2, y: 2 }),
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

        for elem in test_grid {
            assert!(matches!(elem, (Coord { .. }, Tile::Air)))
        }
    }

    #[test]
    fn test_grid_ref_intoiter() {
        let test_grid = create_test_grid();

        for elem in &test_grid {
            assert!(matches!(elem, (Coord { .. }, &Tile::Air)))
        }
    }

    #[test]
    fn test_grid_mut_ref_intoiter() {
        let mut test_grid = create_test_grid();

        for elem in &mut test_grid {
            assert!(matches!(elem, (Coord { .. }, &mut Tile::Air)))
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

    #[test]
    fn test_grid_to_ref_vec() {
        let test_grid = create_test_grid();

        assert_eq!(test_grid.to_ref_vec(), Vec::from([
            (Coord{x: 0, y: 0}, &Tile::Air),
            (Coord{x: 0, y: 1}, &Tile::Air),

            (Coord{x: 1, y: 0}, &Tile::Air),
            (Coord{x: 1, y: 1}, &Tile::Air),
        ]))
    }

    #[test]
    fn test_grid_to_mut_ref_vec() {
        let mut test_grid = create_test_grid();

        assert_eq!(test_grid.to_mut_ref_vec(), Vec::from([
            (Coord{x: 0, y: 0}, &mut Tile::Air),
            (Coord{x: 0, y: 1}, &mut Tile::Air),

            (Coord{x: 1, y: 0}, &mut Tile::Air),
            (Coord{x: 1, y: 1}, &mut Tile::Air),
        ]))
    }
}