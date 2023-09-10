#![allow(unused)]

use std::{
    collections::{HashMap, HashSet},
    hash::Hash,
};

#[derive(Clone, Copy, PartialEq, Eq, Debug, Hash)]
pub struct Pos {
    pub x: u8,
    pub y: u8,
}

impl Pos {
    pub const fn new(x: u8, y: u8) -> Self {
        Self { x, y }
    }

    #[inline(always)]
    pub fn move_direction(&self, dir: Direction) -> Pos {
        let mut res = *self;
        match dir {
            Direction::Up => res.y = res.y.wrapping_add(1),
            Direction::Down => res.y = res.y.wrapping_sub(1),
            Direction::Right => res.x = res.x.wrapping_add(1),
            Direction::Left => res.x = res.x.wrapping_sub(1),
        }
        res
    }
    #[inline(always)]
    pub fn get_cell_edges(&self) -> [EdgePos; 4] {
        todo!()
    }
    #[inline(always)]
    pub fn get_neighbours(&self) -> [Pos; 4] {
        todo!()
    }
    /// Returns the direction to go from self to other, if self and other are adjacent
    pub fn get_direction_to(&self, other: &Self) -> Option<Direction> {
        match (other.x.wrapping_sub(self.x), other.y.wrapping_sub(self.y)) {
            (0, 1) => Some(Direction::Up),
            (0, 255) => Some(Direction::Down),
            (1, 0) => Some(Direction::Right),
            (255, 0) => Some(Direction::Left),
            _ => None,
        }
    }
}

#[derive(Clone, Copy, PartialEq, Eq, Debug, Hash)]
pub enum Direction {
    Up,
    Down,
    Right,
    Left,
}

impl Direction {
    pub const VARIANTS: [Self; 4] = [Self::Up, Self::Down, Self::Right, Self::Left];
}

#[derive(Eq, Clone, Copy, Debug)]
pub struct EdgePos {
    pub pos: Pos,
    pub dir: Direction,
}

impl Hash for EdgePos {
    fn hash<H: std::hash::Hasher>(&self, state: &mut H) {
        let norm = self.normalize();
        norm.pos.hash(state);
        norm.dir.hash(state);
    }
}

impl EdgePos {
    pub const fn new(x: u8, y: u8, dir: Direction) -> Self {
        Self {
            pos: Pos { x, y },
            dir,
        }
    }

    /// Return the cells left and right of the edge.
    pub fn get_neighbouring_cells(&self) -> [Pos; 2] {
        todo!()
    }

    fn normalize(&self) -> Self {
        let mut res = *self;
        match self.dir {
            Direction::Down => {
                res.pos.y -= 1;
                res.dir = Direction::Up
            }
            Direction::Left => {
                res.pos.x -= 1;
                res.dir = Direction::Right
            }
            Direction::Up | Direction::Right => {}
        }
        res
    }

    #[inline(always)]
    fn true_eq(self, other: &Self) -> bool {
        self.pos == other.pos && self.dir == other.dir
    }
}

impl PartialEq for EdgePos {
    #[inline(always)]
    fn eq(&self, other: &Self) -> bool {
        self.normalize().true_eq(&other.normalize())
    }
}

#[derive(Clone)]
pub enum CellType {
    /// Empty cell that has no constaints
    Empty,
    /// Squares may not be in the same area as different colored squares
    Square(u32),
    /// Start constraint, exactly 2 elements of that color may be
    /// present in the same area. The wrapped value is used to represent a color
    Star(u32),
    /// Triangle constraint, forces the solution path to be adjacent to this cell
    /// as many times as there are triangles in the cell
    Triangle(u8),
    /// Tetris constraint, represented as a list of coordinates
    /// Maybe swtiched out to a bitmask implementation in the future
    Tetris(Vec<Pos>),
    /// Canceller cancels out one constraint in the same area. If there are two, they may cancel each other.
    Canceller(u32),
}

#[derive(Clone)]
pub struct Puzzle {
    pub width: u8,
    pub height: u8,

    /// List of start vertices of the puzzle
    pub starts: Vec<Pos>,
    /// List of end vertices of the puzzle
    pub ends: Vec<Pos>,
    // List of blocked edges
    pub blocked_edges: HashSet<EdgePos>,

    // pub symmetry: represented by a transform?

    // Constraints
    pub squares: HashMap<Pos, u32>,
    pub stars: HashMap<Pos, u32>,
    pub triangles: HashMap<Pos, u8>,
    pub tetris: HashMap<Pos, Vec<Pos>>,
    pub cancels: HashMap<Pos, u32>,
    pub vertex_stones: HashSet<Pos>,
    pub edge_stones: HashSet<EdgePos>,
}

impl Default for Puzzle {
    fn default() -> Self {
        Self {
            width: 1,
            height: 1,
            starts: vec![Pos { x: 0, y: 0 }],
            ends: vec![Pos { x: 1, y: 1 }],
            blocked_edges: Default::default(),
            squares: Default::default(),
            stars: Default::default(),
            triangles: Default::default(),
            tetris: Default::default(),
            cancels: Default::default(),
            vertex_stones: Default::default(),
            edge_stones: Default::default(),
        }
    }
}

impl Puzzle {
    /// Check if the vertex is inside the puzzle
    #[inline(always)]
    pub fn contains_vertex(&self, pos: &Pos) -> bool {
        pos.x <= self.width && pos.y <= self.height
    }

    /// Check if the cell is inside the puzzle
    #[inline(always)]
    pub fn contains_cell(&self, pos: &Pos) -> bool {
        pos.x < self.width && pos.y < self.height
    }

    /// Check if the position is on the puzzle boundary
    #[inline(always)]
    pub fn is_outer(&self, pos: &Pos) -> bool {
        pos.x == 0 || pos.x == self.width || pos.y == 0 || pos.y == self.height
    }

    /// Check if the given edge is broken
    #[inline(always)]
    pub fn is_blocked(&self, edge: &EdgePos) -> bool {
        self.blocked_edges.contains(edge)
    }

    /// Check that the proposed solution is valid
    /// This is quite expensive, and should only be used
    /// for testing, as a ground truth
    pub fn is_solution(&self, path: &[Pos]) -> bool {
        if path.len() < 2 {
            return false;
        }

        // Check start and end
        let start = self.starts.contains(path.first().unwrap());
        let end = self.ends.contains(path.last().unwrap());

        // Get edges
        let mut edges = Vec::with_capacity(path.len() - 1);
        for window in path.windows(2) {
            let dir = window[0].get_direction_to(&window[1]);

            if dir.is_none() {
                return false;
            }

            edges.push(EdgePos::new(window[0].x, window[0].y, dir.unwrap()))
        }

        // Check stones
        let vertex_stone = self.vertex_stones.iter().all(|pos| path.contains(pos));
        let edge_stone = self.edge_stones.iter().all(|edge| edges.contains(edge));

        // Compute areas
        let mut areas: Vec<HashSet<Pos>> = vec![];
        for x in 0..self.width {
            for y in 0..self.height {
                let pos = Pos::new(x, y);
                if !areas.iter().any(|area| area.contains(&pos)) {
                    // Floodfill from this cell
                    let mut area = HashSet::new();
                    self.floodfill(pos, &edges, &mut area);
                    areas.push(area);
                }
            }
        }
        let areas_valid = areas.iter().all(|area| self.is_valid(area));

        start && end && vertex_stone && edge_stone && areas_valid
    }

    fn is_valid(&self, area: &HashSet<Pos>) -> bool {
        // Check squares
        let mut color: Option<u32> = None;
        for cell in area.iter() {
            match self.squares.get(cell) {
                Some(col) if color.get_or_insert(*col) != col => return false,
                _ => {},
            }
        }
        true
    }

    fn floodfill(&self, pos: Pos, edges: &Vec<EdgePos>, area: &mut HashSet<Pos>) {
        // Perf: maybe switch the edge list for Hashset?
        // Return immediately if cell is outside of the puzzle or
        // if the cell was already in the area
        if !self.contains_cell(&pos) || !area.insert(pos) {
            return;
        }

        for dir in Direction::VARIANTS {
            let crossing_edge = match dir {
                Direction::Up => EdgePos::new(pos.x, pos.y + 1, Direction::Right),
                Direction::Down => EdgePos::new(pos.x, pos.y, Direction::Right),
                Direction::Right => EdgePos::new(pos.x + 1, pos.y, Direction::Up),
                Direction::Left => EdgePos::new(pos.x, pos.y, Direction::Up),
            };

            if !edges.contains(&crossing_edge) {
                self.floodfill(pos.move_direction(dir), edges, area);
            }
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    const UP_EDGE: EdgePos = EdgePos::new(0, 0, Direction::Up);
    const DOWN_EDGE: EdgePos = EdgePos::new(0, 1, Direction::Down);
    const RIGHT_EDGE: EdgePos = EdgePos::new(0, 0, Direction::Right);
    const LEFT_EDGE: EdgePos = EdgePos::new(1, 0, Direction::Left);

    #[test]
    fn test_edge_true_eq() {
        assert!(UP_EDGE.true_eq(&UP_EDGE));
        assert!(!UP_EDGE.true_eq(&DOWN_EDGE));
    }

    #[test]
    fn test_edge_eq() {
        assert!(UP_EDGE == UP_EDGE);
        assert!(UP_EDGE == DOWN_EDGE);
        assert!(RIGHT_EDGE == LEFT_EDGE);
    }

    #[test]
    fn test_edge_normalise() {
        assert!(DOWN_EDGE.normalize().true_eq(&UP_EDGE));
        assert!(LEFT_EDGE.normalize().true_eq(&RIGHT_EDGE));
    }

    #[test]
    fn test_broken_edges() {
        let puzzle = Puzzle {
            blocked_edges: [UP_EDGE].into(),
            ..Default::default()
        };

        assert!(puzzle.is_blocked(&UP_EDGE));
        assert!(puzzle.is_blocked(&DOWN_EDGE));
        assert!(!puzzle.is_blocked(&RIGHT_EDGE));
        assert!(!puzzle.is_blocked(&LEFT_EDGE));
    }

    #[test]
    fn test_get_direction() {
        let pos1 = Pos::new(0, 0);
        let pos2 = Pos::new(0, 1);
        let pos3 = Pos::new(1, 0);
        let pos4 = Pos::new(1, 1);

        assert!(pos1.get_direction_to(&pos2) == Some(Direction::Up));
        assert!(pos2.get_direction_to(&pos1) == Some(Direction::Down));
        assert!(pos1.get_direction_to(&pos3) == Some(Direction::Right));
        assert!(pos3.get_direction_to(&pos1) == Some(Direction::Left));
        assert!(pos1.get_direction_to(&pos4) == None);
        assert!(pos4.get_direction_to(&pos1) == None);
        assert!(pos2.get_direction_to(&pos3) == None);
        assert!(pos3.get_direction_to(&pos2) == None);
    }

    #[test]
    fn test_2x1_squares() {
        // This puzzle is a 2x1 puzzle with the start in the middle
        // at the bottom and the exit above it. The left cell contains
        // a black square, the right one contains a white square
        let puzzle = Puzzle {
            width: 2,
            height: 1,
            starts: vec![Pos::new(1, 0)],
            ends: vec![Pos::new(1, 1)],
            squares: [(Pos::new(0, 0), 0), (Pos::new(1, 0), 1)].into(),
            ..Default::default()
        };
        let path = [Pos::new(1, 0), Pos::new(1, 1)];

        assert!(puzzle.is_solution(&path));
    }
}
