#![allow(unused)]

use std::{
    collections::{HashMap, HashSet},
    fmt::Display,
    hash::Hash,
    ops::{Add, Deref, DerefMut, Sub},
};

#[derive(Clone, Copy, PartialEq, Eq, Debug, Hash)]
pub struct Pos {
    pub x: i8,
    pub y: i8,
}

impl Display for Pos {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "({}, {})", self.x, self.y)
    }
}

impl Add for Pos {
    type Output = Self;

    fn add(self, rhs: Self) -> Self::Output {
        Pos {
            x: self.x + rhs.x,
            y: self.y + rhs.y,
        }
    }
}

impl Sub for Pos {
    type Output = Self;

    fn sub(self, rhs: Self) -> Self::Output {
        Pos {
            x: self.x - rhs.x,
            y: self.y - rhs.y,
        }
    }
}

impl Pos {
    pub const fn new(x: i8, y: i8) -> Self {
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

    /// Returns the 4 edges that surround the given cell, in this order:
    /// Up, Down, Right, Left
    #[inline(always)]
    pub fn get_cell_edges(&self) -> [EdgePos; 4] {
        [
            EdgePos::new(self.x, self.y + 1, Direction::Right),
            EdgePos::new(self.x, self.y, Direction::Right),
            EdgePos::new(self.x + 1, self.y, Direction::Up),
            EdgePos::new(self.x, self.y, Direction::Up),
        ]
    }

    #[inline(always)]
    pub fn get_neighbours(&self) -> [Pos; 4] {
        todo!()
    }

    /// Returns the direction to go from self to other, if self and other are adjacent
    pub fn get_direction_to(&self, other: &Self) -> Option<Direction> {
        match (other.x.wrapping_sub(self.x), other.y.wrapping_sub(self.y)) {
            (0, 1) => Some(Direction::Up),
            (0, -1) => Some(Direction::Down),
            (1, 0) => Some(Direction::Right),
            (-1, 0) => Some(Direction::Left),
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

impl Display for Direction {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(
            f,
            "{}",
            match self {
                Direction::Up => "U",
                Direction::Down => "D",
                Direction::Right => "R",
                Direction::Left => "L",
            }
        )
    }
}

#[derive(Clone, Debug, PartialEq)]
pub struct SolutionPath(Vec<Pos>);

impl SolutionPath {
    pub fn new(start: Pos, dirs: String) -> Result<Self, ()> {
        let mut res = vec![start];
        let mut prev = start;

        for char in dirs.chars() {
            let dir = match char {
                'U' => Direction::Up,
                'D' => Direction::Down,
                'R' => Direction::Right,
                'L' => Direction::Left,
                _ => return Err(()),
            };

            let next = prev.move_direction(dir);
            res.push(next);
            prev = next;
        }

        Ok(Self(res))
    }
}

impl From<Vec<Pos>> for SolutionPath {
    fn from(value: Vec<Pos>) -> Self {
        Self(value)
    }
}

impl Display for SolutionPath {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        if self.is_empty() {
            write!(f, "Empty solution");
            return Ok(());
        }

        write!(f, "{} ", self[0]);

        for win in self.windows(2) {
            let prev = win[0];
            let next = win[1];

            write!(
                f,
                "{}",
                prev.get_direction_to(&next).ok_or(std::fmt::Error)?
            );
        }

        Ok(())
    }
}

impl Deref for SolutionPath {
    type Target = Vec<Pos>;

    fn deref(&self) -> &Self::Target {
        &self.0
    }
}

impl DerefMut for SolutionPath {
    fn deref_mut(&mut self) -> &mut Self::Target {
        &mut self.0
    }
}

#[derive(Eq, Clone, Copy, Debug)]
pub struct EdgePos {
    pub pos: Pos,
    pub dir: Direction,
}

impl EdgePos {
    pub const fn new(x: i8, y: i8, dir: Direction) -> Self {
        Self {
            pos: Pos { x, y },
            dir,
        }
    }

    /// Return the cells left and right of the edge.
    pub fn get_neighbouring_cells(&self) -> (Pos, Pos) {
        let x = self.pos.x;
        let y = self.pos.y;

        match self.dir {
            Direction::Up => (Pos::new(x - 1, y), Pos::new(x, y)),
            Direction::Down => (Pos::new(x, y - 1), Pos::new(x - 1, y - 1)),
            Direction::Right => (Pos::new(x, y), Pos::new(x, y - 1)),
            Direction::Left => (Pos::new(x - 1, y - 1), Pos::new(x - 1, y)),
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

impl Hash for EdgePos {
    fn hash<H: std::hash::Hasher>(&self, state: &mut H) {
        let norm = self.normalize();
        norm.pos.hash(state);
        norm.dir.hash(state);
    }
}

pub type Color = u8;

/// Represents a polyonmino
#[derive(PartialEq, Eq, Clone, Debug, Hash)]
pub struct Poly {
    pub rotatable: bool,
    pub minos: Vec<Pos>,
}

impl From<u32> for Poly {
    fn from(value: u32) -> Self {
        let rotatable = (value & 1 << 20) != 0;
        let mut minos = vec![];

        for x in 0..4 {
            for y in 0..4 {
                if (value & (1 << (x * 4 + 3 - y))) != 0 {
                    minos.push(Pos::new(x as i8, y as i8))
                }
            }
        }

        Self { rotatable, minos }
    }
}

impl Poly {
    /// Returns the possible rotations for a polyomino
    /// depending on if it is rotatable or
    /// TODO: precompute, cause this gets called a LOT per piece
    pub fn get_rotations(&self) -> Vec<Self> {
        let mut res = vec![self.clone()];
        let mut current = self.clone();

        if self.rotatable {
            current.rotate_clockwise();
            res.push(current.clone());
            current.rotate_clockwise();
            res.push(current.clone());
            current.rotate_clockwise();
            res.push(current.clone());
        }

        res
    }

    fn rotate_clockwise(&mut self) {
        let center = self.minos[0];
        for block in &mut self.minos[1..] {
            let new_pos = Pos {
                x: center.x - (block.y - center.y),
                y: center.y + (block.x - center.x),
            };
            *block = new_pos;
        }
    }
}

#[derive(Clone)]
pub enum CellType {
    /// Empty cell that has no constaints
    Empty,
    /// Squares may not be in the same area as different colored squares
    Square(Color),
    /// Start constraint, exactly 2 elements of that color may be
    /// present in the same area.
    Star(Color),
    /// Triangle constraint, forces the solution path to be adjacent to this cell
    /// as many times as there are triangles in the cell
    Triangle(u8),
    /// Tetris constraint, represented as a list of coordinates
    /// Maybe swtiched out to a bitmask implementation in the future
    Poly(Poly),
    /// Tetris constraint, represented as a list of coordinates
    /// Maybe swtiched out to a bitmask implementation in the future
    Ylop(Poly),
    /// Canceller cancels out one constraint in the same area. If there are two, they may cancel each other.
    Canceller(Color),
}

#[derive(Clone, Debug)]
pub struct Puzzle {
    pub width: i8,
    pub height: i8,

    /// List of start vertices of the puzzle
    pub starts: Vec<Pos>,
    /// List of end vertices of the puzzle
    pub ends: Vec<Pos>,
    // List of blocked edges
    pub blocked_edges: HashSet<EdgePos>,

    // pub symmetry: represented by a transform?

    // Constraints
    pub squares: HashMap<Pos, Color>,
    pub stars: HashMap<Pos, Color>,
    pub triangles: HashMap<Pos, u8>,
    pub polys: HashMap<Pos, Poly>,
    pub ylops: HashMap<Pos, Poly>,
    pub cancels: HashMap<Pos, Color>,
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
            polys: Default::default(),
            ylops: Default::default(),
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
        pos.x >= 0 && pos.x <= self.width && pos.y >= 0 && pos.y <= self.height
    }

    /// Check if the cell is inside the puzzle
    #[inline(always)]
    pub fn contains_cell(&self, pos: &Pos) -> bool {
        pos.x >= 0 && pos.x < self.width && pos.y >= 0 && pos.y < self.height
    }

    /// Check if the position is on the puzzle boundary
    #[inline(always)]
    pub fn is_outer(&self, pos: &Pos) -> bool {
        pos.x == 0 || pos.x == self.width || pos.y == 0 || pos.y == self.height
    }

    /// Check if the given edge is broken
    #[inline(always)]
    pub fn is_blocked(&self, edge: &EdgePos) -> bool {
        self.blocked_edges.contains(&edge.normalize())
    }

    /// Check that the proposed solution is valid
    /// This is quite expensive, and should only be used
    /// for testing, as a ground truth
    pub fn is_solution(&self, path: &SolutionPath) -> bool {
        if path.len() < 2 {
            return false;
        }

        // Check start and end
        let start = self.starts.contains(path.first().unwrap());
        let end = self.ends.contains(path.last().unwrap());

        // Get edges
        let mut path_edges = Vec::with_capacity(path.len() - 1);
        for window in path.windows(2) {
            let dir = window[0].get_direction_to(&window[1]);

            if dir.is_none() {
                return false;
            }

            path_edges.push(EdgePos::new(window[0].x, window[0].y, dir.unwrap()))
        }

        // Check stones
        let vertex_stone = self.vertex_stones.iter().all(|pos| path.contains(pos));
        let edge_stone = self
            .edge_stones
            .iter()
            .all(|edge| path_edges.contains(edge));

        // Check triangles
        let mut triangle = true;
        for (triangle_pos, count) in self.triangles.iter() {
            let cell_edges = &triangle_pos.get_cell_edges();
            let filled_cell_edges_count = cell_edges
                .iter()
                .filter(|edge| path_edges.contains(edge))
                .count();

            if filled_cell_edges_count != *count as usize {
                triangle = false;
                break;
            }
        }

        // Compute areas
        let areas_valid =
            if !self.squares.is_empty() || !self.stars.is_empty() || !self.polys.is_empty() {
                let mut areas: Vec<HashSet<Pos>> = vec![];
                for x in 0..self.width {
                    for y in 0..self.height {
                        let pos = Pos::new(x, y);
                        if !areas.iter().any(|area| area.contains(&pos)) {
                            // Floodfill from this cell
                            let mut area = HashSet::new();
                            self.floodfill(pos, &path_edges, &mut area);
                            areas.push(area);
                        }
                    }
                }
                // Check areas
                areas.iter().all(|area| self.is_valid(area))
            } else {
                true
            };

        start && end && vertex_stone && edge_stone && triangle && areas_valid
    }

    pub fn is_valid(&self, area: &HashSet<Pos>) -> bool {
        // If there are cancels in the area
        if let Some(cancel_pos) = self.cancels.keys().find(|p| area.contains(p)) {
            // Try removing a symbol in the area and recurse
            for pos in area {
                if pos == cancel_pos {
                    continue;
                }

                let mut new_puzzle = self.clone();
                new_puzzle.cancels.remove(&cancel_pos);

                let valid = if self.squares.contains_key(pos) {
                    new_puzzle.squares.remove(&pos);
                    new_puzzle.is_valid(area)
                } else if self.stars.contains_key(pos) {
                    new_puzzle.stars.remove(&pos);
                    new_puzzle.is_valid(area)
                } else if self.triangles.contains_key(pos) {
                    new_puzzle.triangles.remove(&pos);
                    new_puzzle.is_valid(area)
                } else if self.polys.contains_key(pos) {
                    new_puzzle.polys.remove(&pos);
                    new_puzzle.is_valid(area)
                } else if self.ylops.contains_key(pos) {
                    new_puzzle.ylops.remove(&pos);
                    new_puzzle.is_valid(area)
                } else if self.cancels.contains_key(pos) {
                    new_puzzle.cancels.remove(&pos);
                    new_puzzle.is_valid(area)
                } else if self.vertex_stones.contains(pos) {
                    new_puzzle.vertex_stones.remove(&pos);
                    new_puzzle.is_valid(area)
                } else {
                    false
                };
                // TODO: Add cancelling for edge stones

                if valid {
                    return true;
                }
            }

            // This canceller has nothing to cancel, or none of the
            // cancellations lead to a valid solution
            return false;
        }

        // Check squares
        let mut color: Option<Color> = None;
        for cell in area.iter() {
            match self.squares.get(cell) {
                Some(col) if color.get_or_insert(*col) != col => return false,
                _ => {}
            }
        }

        // Check stars
        let mut color_counts: HashMap<Color, u8> = HashMap::new();
        for &star_color in area.iter().filter_map(|pos| self.stars.get(pos)) {
            *color_counts.entry(star_color).or_insert(0) += 1;
            if color_counts[&star_color] > 2 {
                return false;
            }
        }
        // If there is a star in the area, do the other checks, only on the star colors of course
        if !color_counts.is_empty() {
            for square_color in area.iter().filter_map(|pos| self.squares.get(pos)) {
                match color_counts.get(square_color) {
                    Some(1) => *color_counts.get_mut(square_color).unwrap() = 2,
                    Some(_) => return false,
                    None => {}
                }
            }

            for canceller_color in area.iter().filter_map(|pos| self.cancels.get(pos)) {
                match color_counts.get(canceller_color) {
                    Some(1) => *color_counts.get_mut(canceller_color).unwrap() = 2,
                    Some(_) => return false,
                    None => {}
                }
            }

            if color_counts.values().any(|&count| count != 2) {
                return false;
            }
        }

        if !self.check_tetris(area) {
            return false;
        }

        true
    }

    /// Return true if the area is valid according to the tetris rule
    /// false otherwise
    fn check_tetris(&self, area: &HashSet<Pos>) -> bool {
        let polys: Vec<_> = area.iter().filter_map(|pos| self.polys.get(pos)).collect();
        let ylops: Vec<_> = area.iter().filter_map(|pos| self.ylops.get(pos)).collect();

        if polys.is_empty() && ylops.is_empty() {
            return true;
        }

        let tile_count = polys.iter().fold(0, |acc, poly| acc + poly.minos.len());
        let elit_count = ylops.iter().fold(0, |acc, ylop| acc + ylop.minos.len());

        // If the tile counts don't even match the area, don't bother doing anything else
        if tile_count - elit_count != area.len() {
            return false;
        }

        let mut covering = HashMap::new();
        for &pos in area.iter() {
            covering.insert(pos, 0);
        }

        self.can_tile(polys, ylops, &covering)
    }

    /// Try tiling the given polys and ylops in the given area
    /// This implementation is heavily inspired from jbdarkid's solver
    /// This is currently implemented using a perly recursive strategy
    /// TODO: switch to backtracking to improve memory footprint
    fn can_tile(&self, polys: Vec<&Poly>, ylops: Vec<&Poly>, area: &HashMap<Pos, i16>) -> bool {
        // Insert stop condition(s) here
        if polys.is_empty() && ylops.is_empty() {
            return area.values().all(|&cover_count| cover_count == 1);
        }

        // Find a square that is not covered
        let square = area.iter().find(|(_pos, &value)| value < 1).unwrap();

        for (poly_idx, poly) in polys.iter().enumerate() {
            // For every mino, attempt to place it at the selected square
            for rotation in &poly.get_rotations() {
                // For every rotation state of the poly
                for center_mino in rotation.minos.iter() {
                    // Recurse if mino is fully contained within the area and
                    // only covers squares with 0 or less minos on it
                    if rotation.minos.iter().all(|mino|
                        matches!(area.get(&(*square.0 + *mino - *center_mino)), Some(&cover_count) if cover_count < 1)
                    ) {
                        let mut new_area = area.clone();
                        for mino in rotation.minos.iter() {
                            // TODO: increase cover count instead
                            new_area.remove(&(*square.0 + *mino - *center_mino));
                        }
                        let mut new_polys = polys.clone();
                        new_polys.swap_remove(poly_idx);
                        if self.can_tile(new_polys, ylops.clone(), &new_area) {
                            return true;
                        }
                    }
                }
            }
        }

        false
    }

    /// Returns the list of connected cells starting from `pos`, delimited by `edges`
    pub fn floodfill(&self, pos: Pos, edges: &Vec<EdgePos>, area: &mut HashSet<Pos>) {
        // TODO: maybe switch the edge list for Hashset?
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

    /// Creates a Puzzle from jbdarkid's json format
    /// TODO: Implement ends/starts on edges. They are equivalent
    /// to two end/start on each end of the edge, and a blocked
    /// edge separating them
    pub fn from_json(src: &str) -> Result<Self, String> {
        let data = json::parse(src).unwrap();

        let width = data["width"]
            .as_usize()
            .ok_or("Failed to decode puzzle width")?
            / 2;
        let height = data["height"]
            .as_usize()
            .ok_or("Failed to decode puzzle height")?
            / 2;

        let mut puzzle = Puzzle {
            width: width as i8,
            height: height as i8,
            starts: vec![],
            ends: vec![],
            ..Default::default()
        };

        // To read the grid, we need to do some coordinate manipulations
        // because of the format

        // Process corners (starts, ends, stones)
        for x in 0..=width {
            for y in 0..=height {
                let cell = &data["grid"][x * 2][(height - y) * 2];
                if cell["start"] == true {
                    puzzle.starts.push(Pos::new(x as i8, y as i8));
                }

                if cell["end"].is_string() {
                    puzzle.ends.push(Pos::new(x as i8, y as i8));
                }

                if cell["dot"].is_number() {
                    puzzle.vertex_stones.insert(Pos::new(x as i8, y as i8));
                }
            }
        }

        // Process edges (stone, broken edge)
        for x in 0..=width {
            for y in 0..=height {
                let up = &data["grid"][x * 2][(height - y) * 2 - 1];
                let right = &data["grid"][x * 2 + 1][(height - y) * 2];

                if up["dot"].is_number() {
                    puzzle
                        .edge_stones
                        .insert(EdgePos::new(x as i8, y as i8, Direction::Up));
                }
                if right["dot"].is_number() {
                    puzzle
                        .edge_stones
                        .insert(EdgePos::new(x as i8, y as i8, Direction::Right));
                }
                if up["gap"].is_number() {
                    puzzle
                        .blocked_edges
                        .insert(EdgePos::new(x as i8, y as i8, Direction::Up));
                }
                if right["gap"].is_number() {
                    puzzle
                        .blocked_edges
                        .insert(EdgePos::new(x as i8, y as i8, Direction::Right));
                }
            }
        }

        let mut colors = HashMap::new();

        // Process cells
        for x in 0..width {
            for y in 0..height {
                let cell = &data["grid"][x * 2 + 1][(height - y) * 2 - 1];
                let pos = Pos::new(x as i8, y as i8);
                let nb_of_colors = colors.len();

                if cell.is_null() {
                    continue;
                }

                match cell["type"].as_str().ok_or("Failed to decode cell type")? {
                    "triangle" => {
                        puzzle.triangles.insert(
                            pos,
                            cell["count"]
                                .as_u8()
                                .ok_or("Couldn't decode triangle count")?,
                        );
                    }
                    "square" => {
                        puzzle.squares.insert(
                            pos,
                            *colors
                                .entry(cell["color"].as_str().ok_or("Failed to decode color")?)
                                .or_insert(nb_of_colors as u8),
                        );
                    }
                    "star" => {
                        puzzle.stars.insert(
                            pos,
                            *colors
                                .entry(cell["color"].as_str().ok_or("Failed to decode color")?)
                                .or_insert(nb_of_colors as u8),
                        );
                    }
                    "poly" => {
                        puzzle.polys.insert(
                            pos,
                            Poly::from(
                                cell["polyshape"]
                                    .as_u32()
                                    .ok_or("Failed to decode polyshape")?,
                            ),
                        );
                    }
                    "ylop" => {
                        puzzle.ylops.insert(
                            pos,
                            Poly::from(
                                cell["polyshape"]
                                    .as_u32()
                                    .ok_or("Failed to decode polyshape")?,
                            ),
                        );
                    }
                    "nega" => {
                        puzzle.cancels.insert(
                            pos,
                            *colors
                                .entry(cell["color"].as_str().ok_or("Failed to decode color")?)
                                .or_insert(nb_of_colors as u8),
                        );
                    }
                    _ => {}
                };
            }
        }

        Ok(puzzle)
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
        let solution = SolutionPath::new(Pos::new(1, 0), "U".into()).unwrap();

        assert!(puzzle.is_solution(&solution));
    }

    #[test]
    fn test_2x1_stars() {
        // This puzzle is a 2x1 puzzle with the start in the middle
        // at the bottom and the exit above it. Both cells contain a star
        let puzzle = Puzzle {
            width: 2,
            height: 1,
            starts: vec![Pos::new(1, 0)],
            ends: vec![Pos::new(1, 1)],
            stars: [(Pos::new(0, 0), 0), (Pos::new(1, 0), 0)].into(),
            ..Default::default()
        };
        let solutions = [
            SolutionPath::new(Pos::new(1, 0), "LUR".into()).unwrap(),
            SolutionPath::new(Pos::new(1, 0), "RUL".into()).unwrap(),
        ];

        for sol in solutions {
            assert!(puzzle.is_solution(&sol));
        }
    }

    #[test]
    fn test_triangles() {
        let puzzle = Puzzle {
            width: 4,
            height: 4,
            starts: vec![Pos::new(0, 0)],
            ends: vec![Pos::new(4, 4)],
            triangles: [
                (Pos::new(0, 3), 3),
                (Pos::new(1, 3), 1),
                (Pos::new(2, 2), 2),
                (Pos::new(3, 2), 2),
                (Pos::new(3, 1), 3),
                (Pos::new(2, 0), 1),
            ]
            .into(),
            ..Default::default()
        };

        let solution =
            SolutionPath::new(Pos::new(0, 0), "UURULURRRDLDDLDRRRULURUU".into()).unwrap();
        assert!(puzzle.is_solution(&solution));
    }
}
