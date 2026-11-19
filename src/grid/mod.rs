#![allow(unused)]

use std::ops::{Index, IndexMut};

pub mod single_grid;

/// Universal coordinate representing a position in the puzzle grid.
///
/// (even, even) coords represent corners
/// (odd, odd) coords represent cells
/// the rest represent edges
#[derive(Clone, Copy)]
pub struct UniversalPos {
    x: i8,
    y: i8,
}

impl UniversalPos {
    pub fn new(x: i8, y: i8) -> Self {
        Self { x, y }
    }

    pub fn get_neighbours(&self) -> [UniversalPos; 4] {
        [
            UniversalPos {
                x: self.x + 1,
                y: self.y,
            },
            UniversalPos {
                x: self.x - 1,
                y: self.y,
            },
            UniversalPos {
                x: self.x,
                y: self.y + 1,
            },
            UniversalPos {
                x: self.x,
                y: self.y - 1,
            },
        ]
    }
}

/// Represents a color
pub type Color = u8;

/// Represent a constraint that can be placed on a puzzle grid corner.
///
/// The last should only be present in symmetry puzzles
#[derive(Clone, Copy, Default)]
pub enum CornerConstraint {
    #[default]
    None,
    Stone,
    BlueStone,
    YellowStone,
}

/// Represent a constraint that can be placed on a puzzle grid edge.
///
/// The last should only be present in symmetry puzzles
#[derive(Clone, Copy, Default)]
pub enum EdgeConstraint {
    #[default]
    None,
    Stone,
    Broken,
    BlueStone,
    YellowStone,
}

/// Represent a constraint that can be placed in a puzzle grid cell.
#[derive(Clone, Copy, Default)]
pub enum CellConstraint {
    #[default]
    None,
    Square(Color),
    Star(Color),
    Triangle(u8, Color),
    Poly(u32, Color),
    Ylop(u32, Color),
    Cancel(Color),
}

/// A generic puzzle constraint
#[derive(Clone, Copy)]
pub enum Constraint {
    Corner(CornerConstraint),
    Edge(EdgeConstraint),
    Cell(CellConstraint),
}

/// A generic interface for interacting with a puzzle grid
pub trait PuzzleGrid {
    /// Init a new puzzle grid
    ///
    /// width and height are in conventionnal cell side count
    fn new(width: i8, height: i8, pillar: bool) -> Self
    where
        Self: Sized;
    // fn get_absolute_width/height

    fn universal_width(&self) -> i8;
    fn universal_height(&self) -> i8;

    /// Get a corner constraint at the given coordinate.
    ///
    /// Should panic if the given coordinate is not a corner
    fn get_corner(&self, coords: &UniversalPos) -> CornerConstraint;

    /// Get an edge constraint at the given coordinate.
    ///
    /// Should panic if the given coordinate is not an edge
    fn get_edge(&self, coords: &UniversalPos) -> EdgeConstraint;

    /// Get a cell constraint at the given coordinate.
    ///
    /// Should panic if the given coordinate is not a cell
    fn get_cell(&self, coords: &UniversalPos) -> CellConstraint;

    /// Get whatever element is at the given coordinates.
    #[inline(always)]
    fn get_any(&self, coords: &UniversalPos) -> Constraint {
        let UniversalPos { x, y } = coords;
        match (x % 2, y % 2) {
            (0, 0) => Constraint::Corner(self.get_corner(coords)),
            (1, 1) => Constraint::Cell(self.get_cell(coords)),
            _ => Constraint::Edge(self.get_edge(coords)),
        }
    }
}

/// Specialised grid for floodfill algorithm
struct VisitedGrid {
    width: i8,
    height: i8,
    cells: Vec<bool>, // Maybe move to bitvec in the future
}

impl VisitedGrid {
    fn new(width: i8, height: i8) -> Self {
        Self {
            width,
            height,
            cells: vec![false; (width * height) as usize],
        }
    }

    fn contains(&self, &UniversalPos { x, y }: &UniversalPos) -> bool {
        0 < x && x < self.width && 0 < y && y < self.height
    }
}

impl Index<&UniversalPos> for VisitedGrid {
    type Output = bool;

    fn index(&self, index: &UniversalPos) -> &Self::Output {
        &self.cells[(index.y * self.width + index.x) as usize]
    }
}

impl IndexMut<&UniversalPos> for VisitedGrid {
    fn index_mut(&mut self, index: &UniversalPos) -> &mut Self::Output {
        &mut self.cells[(index.y * self.width + index.x) as usize]
    }
}

/// Iterator that enumerates all the constraints in the puzzle grid area starting at
/// a specific coordinate
pub struct FloodfillIterator<'a, P: PuzzleGrid + ?Sized> {
    /// Puzzle from which the constraints are read
    puzzle: &'a P,

    /// Grid of visited coordinates
    visited: VisitedGrid,

    /// Queue of positions left to explore
    queue: Vec<UniversalPos>,

    /// List of cells in the floodfilled area, kept track of for the purposes of
    /// tetris constraint verification (should only contain cell coords)
    cells: Vec<UniversalPos>,
}

impl<'a, P: PuzzleGrid> FloodfillIterator<'a, P> {
    pub fn new(puzzle: &'a P, flood_start: Vec<UniversalPos>, path: Vec<UniversalPos>) -> Self {
        let mut visited = VisitedGrid::new(puzzle.universal_width(), puzzle.universal_height());
        // Block the area with the path
        for pos in path {
            visited[&pos] = true;
        }

        Self {
            puzzle,
            queue: flood_start,
            visited,
            cells: vec![],
        }
    }

    /// Destroy the iterator and get the cells visisted so far
    ///
    /// Recommended to only once the iterator is exhausted. Used to get the area
    /// for poly constraint check
    pub fn get_area(self) -> Vec<UniversalPos> {
        self.cells
    }
}

impl<P: PuzzleGrid> Iterator for FloodfillIterator<'_, P> {
    type Item = Constraint;

    fn next(&mut self) -> Option<Self::Item> {
        while let Some(pos) = self.queue.pop() {
            // If already visited, move to next in the queue
            if self.visited[&pos] {
                continue;
            }

            self.visited[&pos] = true;
            for next_pos in pos.get_neighbours() {
                if self.visited.contains(&next_pos) && !self.visited[&next_pos] {
                    self.queue.push(next_pos);
                }
            }

            let constraint = self.puzzle.get_any(&pos);
            if let Constraint::Cell(_) = constraint {
                self.cells.push(pos);
            }

            return Some(constraint);
        }

        None
    }
}

#[cfg(test)]
mod test {
    use benchkit::prelude::*;

    use crate::grid::{
        FloodfillIterator, PuzzleGrid, UniversalPos,
        single_grid::{SingleEnumGrid, SingleUnionGrid},
    };

    /// Test that benches the floodfill implementations
    #[test]
    fn bench_floodfill() {
        let mut suite = BenchmarkSuite::new("Floodfill benches").with_config(MeasurementConfig {
            iterations: 10_000_000,
            warmup_iterations: 150,
            ..Default::default()
        });

        // Bench params
        let start_point = UniversalPos::new(0, 0);
        let (width, height) = (5, 5);
        let pillar = false;

        suite.benchmark("SingleEnumGrid", move || {
            let puzzle = SingleEnumGrid::new(width, height, pillar);
            let mut iter = FloodfillIterator::new(&puzzle, vec![start_point], vec![]);
            iter.count();
        });
        suite.benchmark("SingleUnionGrid", move || {
            let puzzle = SingleUnionGrid::new(width, height, pillar);
            let mut iter = FloodfillIterator::new(&puzzle, vec![start_point], vec![]);
            iter.count();
        });

        let results = suite.run_all();
        results.print_summary();
    }
}
