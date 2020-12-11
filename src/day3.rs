static INPUT: &'static str = include_str!("day3.input");

/// A point on the map
#[derive(Clone)]
struct Coordinate {
    x: usize,
    y: usize,
}
impl Coordinate {
    fn new(x: usize, y: usize) -> Self {
        Coordinate { x, y }
    }
}

/// A direction of travel
struct Vector {
    delta_x: usize,
    delta_y: usize,
}
impl Vector {
    fn new(delta_x: usize, delta_y: usize) -> Self {
        Vector { delta_x, delta_y }
    }
}

/// The contents of a point on the map
enum CellContent {
    Empty,
    Tree,
}
impl From<char> for CellContent {
    fn from(c: char) -> Self {
        match c {
            '.' => Self::Empty,
            '#' => Self::Tree,
            _ => panic!("yikes! bad character!"),
        }
    }
}

/// The map itself
struct TreeMap {
    // the map is a 2D vec of string refs to the input
    map: Vec<Vec<char>>,
}
impl TreeMap {
    fn new(map: &'static str) -> Self {
        // could we do validation of the map here? Yeah! Will we? No!
        Self {
            map: map
                .lines()
                .map(|ln| ln.trim())
                .filter(|ln| ln != &"")
                .map(|ln| ln.chars().collect::<Vec<char>>())
                .collect(),
        }
    }
    fn char_at(&self, coord: &Coordinate) -> Option<char> {
        // we'll say the coordinate grid starts at 0,0 (x, y) at the top left.
        // so a 2-by-2 map would have a max coord of 1,1

        // we assume the map is rectangular and has at least one column!
        let map_width = self.map.get(0).expect("this map has no cols!").len();
        let max_y_idx = self.map.len() - 1;

        // we want the map to be infinite OUT TO THE RIGHT but not down.
        // so if we're beyond the height, /shrug
        if coord.y > max_y_idx {
            return None;
        }

        // in any other case, we'll have something to return. If the x-coord
        // is beyond the map width, we just need to divide it and return the
        // modulo since the map repeats infinitely out to the right, so e.g. on
        // a 2-by-2 map, the coord 2,0 would be equivalent to the coord 0,0,
        // which 0 is 2 % 2.
        let x_coord = coord.x % map_width;

        Some(self.map[coord.y][x_coord])
    }

    /// The content at a particular coordinate
    fn content_at(&self, coord: &Coordinate) -> Option<CellContent> {
        self.char_at(coord).map(|c| c.into())
    }

    /// An iterable of the content along a particular path, starting at start
    fn content_along<'a>(
        // we've got to specify that the vector reference lives as long as the
        // map so the iterator can know the vec is good to keep a reference to
        &'a self,
        start: &Coordinate,
        vector: &'a Vector,
    ) -> MapTraverser {
        MapTraverser::new(self, vector, start.clone())
    }

    /// How many trees lie on a given path, starting in the top-left corner
    fn trees_on_path<'a>(&'a self, vector: &'a Vector) -> usize {
        self.content_along(&Coordinate::new(0, 0), vector)
            .filter_map(|c| match c {
                CellContent::Tree => Some(c),
                _ => None,
            })
            .count()
    }
}

struct MapTraverser<'a> {
    map: &'a TreeMap,
    vector: &'a Vector,
    coord: Coordinate,
}
impl<'a> MapTraverser<'a> {
    fn new(map: &'a TreeMap, vector: &'a Vector, start: Coordinate) -> Self {
        Self {
            map,
            vector,
            coord: start,
        }
    }
}
impl<'a> Iterator for MapTraverser<'a> {
    type Item = CellContent;

    fn next(&mut self) -> Option<Self::Item> {
        match self.map.content_at(&self.coord) {
            Some(c) => {
                self.coord = Coordinate::new(
                    self.coord.x + self.vector.delta_x,
                    self.coord.y + self.vector.delta_y,
                );
                Some(c)
            }
            None => None,
        }
    }
}

pub fn day_three_solution_one() -> usize {
    TreeMap::new(INPUT).trees_on_path(&Vector::new(3, 1))
}

pub fn day_three_solution_two() -> usize {
    // we're in good shape here b/c it just wants us to multiply results from
    // different vectors.
    let map = TreeMap::new(INPUT);
    map.trees_on_path(&Vector::new(1, 1))
        * map.trees_on_path(&Vector::new(3, 1))
        * map.trees_on_path(&Vector::new(5, 1))
        * map.trees_on_path(&Vector::new(7, 1))
        * map.trees_on_path(&Vector::new(1, 2))
}
