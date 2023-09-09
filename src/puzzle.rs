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
    /// Start constraint, only 2 elements of that color may be
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
}
