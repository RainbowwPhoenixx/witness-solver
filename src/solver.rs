#![allow(unused)]

use std::collections::{HashSet, VecDeque};

use crate::puzzle::*;

#[derive(Clone)]
pub struct PartialSolution {
    /// Candidate solution path
    path: SolutionPath,
    partial_area_left: HashSet<Pos>,
    partial_area_right: HashSet<Pos>,
    // /// Completed areas that have already been checked and are correct
    // full_areas: Vec<HashSet<Pos>>,
    // /// Cancels that are not yet taken by partial or full areas
    // cancels_left: u8,
    /// Number of reachable ends left
    reachable_ends: u8,
}

impl PartialSolution {
    pub fn new(start: Pos, cancels: u8, ends: u8) -> Self {
        Self {
            path: vec![start].into(),
            partial_area_left: HashSet::new(),
            partial_area_right: HashSet::new(),
            // full_areas: vec![],
            // cancels_left: cancels,
            reachable_ends: ends,
        }
    }
}

pub struct BFSSolverConfig {
    pub simple_end_reachability_check: bool,
    pub edge_stones: bool,
    pub partial_area_check: bool,
    pub closed_area_check: bool,
    pub max_solutions: u32, // if 0, get all solutions
}

impl Default for BFSSolverConfig {
    fn default() -> Self {
        Self {
            simple_end_reachability_check: true,
            edge_stones: true,
            partial_area_check: true,
            closed_area_check: false, // TODO: change this when it gets implemented
            max_solutions: 0,
        }
    }
}

pub struct BFSSolver {
    /// Initial puzzle
    puzzle: Puzzle,
    /// Queue of potential solutions, sorted from shortest to longest
    queue: VecDeque<PartialSolution>,
    /// List of solutions found so far
    solutions: Vec<SolutionPath>,

    // Statistics
    pub states_visited: u64,

    // Config
    pub config: BFSSolverConfig,
}

impl BFSSolver {
    pub fn new(puzzle: &Puzzle) -> Self {
        Self {
            puzzle: puzzle.clone(),
            queue: VecDeque::new(),
            solutions: vec![],
            states_visited: 0,
            config: Default::default(),
        }
    }

    pub fn solve(&mut self) -> Vec<SolutionPath> {
        if !self.solutions.is_empty() {
            return self.solutions.clone();
        }

        // Init the search with the start nodes
        for &start in self.puzzle.starts.iter() {
            self.queue.push_back(PartialSolution::new(
                start,
                self.puzzle.cancels.len() as u8,
                self.puzzle.ends.len() as u8,
            ))
        }

        // So long as there are states to be visited, keep processing them
        while !self.queue.is_empty() {
            if self.queue.len() > 70_000_000 {
                println!("Exiting here for fear of OOM");
                return self.solutions.clone();
            }
            let partial_sol = self.queue.pop_front().unwrap();
            self.process_partial_solution(partial_sol);
            self.states_visited += 1;

            if self.config.max_solutions > 0
                && self.solutions.len() >= self.config.max_solutions as usize
            {
                return self.solutions.clone();
            }
        }

        self.solutions.clone()
    }

    fn process_partial_solution(&mut self, sol: PartialSolution) {
        let pos = sol.path.last().unwrap();

        for dir in Direction::VARIANTS {
            let next = pos.move_direction(dir);

            // Check illegal moves:
            // - Check that the target vertex is in the puzzle
            // - Check that the edge we want to take is not blocked
            // - Check that the target vertex is not already part of the path
            //   (in reverse because we are more likely to hit edges that closer
            //   to the end of the candidate solution)
            if !self.puzzle.contains_vertex(&next)
                || self.puzzle.is_blocked(&EdgePos { pos: *pos, dir })
                || sol.path.iter().rev().any(|p| p == &next)
            {
                continue;
            };

            let mut new_sol = sol.clone();
            new_sol.path.push(next);

            if self.config.edge_stones
                && self.puzzle.cancels.is_empty()
                && self.stones_invalid(&new_sol)
            {
                continue;
            }

            let (left, right) = EdgePos { pos: *pos, dir }.get_neighbouring_cells();

            // Check if we enclosed an area
            if self.config.closed_area_check
                && sol.path.len() > 2
                && (!self.puzzle.contains_cell(&left) || !self.puzzle.contains_cell(&right))
            {
                todo!("Enclosed area checking is not yet implemented")
            }

            // TODO: adapt this condition to cancels
            if self.config.partial_area_check && self.puzzle.cancels.is_empty() {
                // Compute & check partial areas
                new_sol.partial_area_left.insert(left);
                new_sol.partial_area_right.insert(right);

                // If left is outside, we are going along an edge and need to clear that area
                if !self.puzzle.contains_cell(&left) {
                    new_sol.partial_area_left.clear();
                }
                // Same with right
                if !self.puzzle.contains_cell(&right) {
                    new_sol.partial_area_right.clear();
                }

                // Discard the solution if one of the areas is invalid
                if self.area_invalid(&new_sol.partial_area_left) {
                    continue;
                }
                if self.area_invalid(&new_sol.partial_area_right) {
                    continue;
                }
            }

            if self.puzzle.ends.contains(&next) {
                // With enough work on early pruning, this
                // call to is_solution could be replaced with much less
                // expensive functions for final checks only
                if self.puzzle.is_solution(&new_sol.path) {
                    self.solutions.push(new_sol.path.clone());
                }

                new_sol.reachable_ends -= 1;
                if self.config.simple_end_reachability_check && new_sol.reachable_ends == 0 {
                    continue;
                }
            }

            self.queue.push_back(new_sol);
        }
    }

    /// Returns true if a superset of the given area
    /// would for sure be invalid
    fn area_invalid(&self, area: &HashSet<Pos>) -> bool {
        // Check squares
        let mut color: Option<Color> = None;
        for cell in area.iter() {
            match self.puzzle.squares.get(cell) {
                Some(col) if color.get_or_insert(*col) != col => return true,
                _ => {}
            }
        }

        false
    }

    /// Return true if it is impossible for the partial solution
    /// to result in a correct solution with regards to the stones
    fn stones_invalid(&self, sol: &PartialSolution) -> bool {
        // There is a stone on an edge perpandicular to the current path
        // We only need to check the last one, since previous ones were checked in other iterations
        for dir in Direction::VARIANTS {
            // SAFETY: this function is never called with a path of one element, since that is just the start node
            let pos = sol.path[sol.path.len() - 2];
            if self
                .puzzle
                .edge_stones
                .contains(&EdgePos::new(pos.x, pos.y, dir))
                && pos.move_direction(dir) != sol.path[sol.path.len() - 1]
                && pos.move_direction(dir) != *sol.path.get(sol.path.len() - 3).unwrap_or(&pos)
            {
                return true;
            };
        }
        false
    }
}

impl Iterator for BFSSolver {
    type Item = Vec<Pos>;

    fn next(&mut self) -> Option<Self::Item> {
        todo!()
    }
}

#[cfg(test)]
mod bfs_tests {
    use super::*;
    use crate::puzzle::*;
    use std::collections::{HashMap, HashSet};

    fn test_solutions(puzzle: &Puzzle, expected_solutions: Vec<SolutionPath>) {
        let mut solver = BFSSolver::new(puzzle);
        let solutions = solver.solve();

        assert_eq!(
            solutions, expected_solutions,
            "Expected: {:?}\nGot: {:?}",
            expected_solutions, solutions
        )
    }

    fn test_solution_count(puzzle: &Puzzle, expected_count: usize) {
        let mut solver = BFSSolver::new(puzzle);
        let solutions = solver.solve();

        assert_eq!(
            solutions.len(),
            expected_count,
            "Expected: {:?}\nGot: {:?}",
            expected_count,
            solutions.len()
        )
    }

    #[test]
    fn test_1x1() {
        let puzzle = Puzzle::default();
        let solutions = vec![
            SolutionPath::new(Pos::new(0, 0), "UR".into()).unwrap(),
            SolutionPath::new(Pos::new(0, 0), "RU".into()).unwrap(),
        ];

        test_solutions(&puzzle, solutions);
    }

    #[test]
    fn test_1x1_broken() {
        let puzzle = Puzzle {
            blocked_edges: [EdgePos::new(0, 0, Direction::Up)].into(),
            ..Default::default()
        };
        let solutions = vec![SolutionPath::new(Pos::new(0, 0), "RU".into()).unwrap()];

        test_solutions(&puzzle, solutions);
    }

    #[test]
    fn test_1x1_multiple_start() {
        let puzzle = Puzzle {
            starts: vec![Pos::new(0, 0), Pos::new(0, 1)],
            ..Default::default()
        };
        let solutions = vec![
            SolutionPath::new(Pos::new(0, 1), "R".into()).unwrap(),
            SolutionPath::new(Pos::new(0, 0), "UR".into()).unwrap(),
            SolutionPath::new(Pos::new(0, 0), "RU".into()).unwrap(),
            SolutionPath::new(Pos::new(0, 1), "DRU".into()).unwrap(),
        ];

        test_solutions(&puzzle, solutions);
    }

    #[test]
    fn test_1x1_multiple_end() {
        let puzzle = Puzzle {
            ends: vec![Pos::new(0, 1), Pos::new(1, 1)],
            ..Default::default()
        };
        let solutions = vec![
            SolutionPath::new(Pos::new(0, 0), "U".into()).unwrap(),
            SolutionPath::new(Pos::new(0, 0), "UR".into()).unwrap(),
            SolutionPath::new(Pos::new(0, 0), "RU".into()).unwrap(),
            SolutionPath::new(Pos::new(0, 0), "RUL".into()).unwrap(),
        ];

        test_solutions(&puzzle, solutions);
    }

    #[test]
    fn test_1x1_stones() {
        let puzzle = Puzzle {
            vertex_stones: [Pos::new(1, 0)].into(),
            edge_stones: [EdgePos::new(0, 0, Direction::Right)].into(),
            ..Default::default()
        };
        let solutions = vec![SolutionPath::new(Pos::new(0, 0), "RU".into()).unwrap()];

        test_solutions(&puzzle, solutions);
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
        let solutions = vec![SolutionPath::new(Pos::new(1, 0), "U".into()).unwrap()];

        test_solutions(&puzzle, solutions);
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
        let solutions = vec![
            SolutionPath::new(Pos::new(1, 0), "RUL".into()).unwrap(),
            SolutionPath::new(Pos::new(1, 0), "LUR".into()).unwrap(),
        ];

        test_solutions(&puzzle, solutions);
    }

    #[test]
    fn test_broken_star_square() {
        // This puzzle is an actual puzzle example from the game that
        // contains broken paths, stars, and squares
        let puzzle = Puzzle {
            blocked_edges: [
                EdgePos::new(0, 4, Direction::Right),
                EdgePos::new(3, 3, Direction::Right),
                EdgePos::new(3, 1, Direction::Up),
                EdgePos::new(2, 0, Direction::Up),
            ]
            .into(),
            squares: [
                (Pos::new(0, 0), 0),
                (Pos::new(1, 0), 0),
                (Pos::new(2, 2), 0),
                (Pos::new(1, 1), 1),
                (Pos::new(2, 3), 1),
                (Pos::new(3, 3), 1),
            ]
            .into(),
            stars: [(Pos::new(3, 0), 5), (Pos::new(0, 3), 5)].into(),
            ..Puzzle::default_with_size(4, 4)
        };
        let solutions = vec![
            SolutionPath::new(Pos::new(0, 0), "RRRRULLLURRULURR".into()).unwrap(),
            SolutionPath::new(Pos::new(0, 0), "UUURRRDLLDRRDRUUUU".into()).unwrap(),
            SolutionPath::new(Pos::new(0, 0), "RRRRUULULDDLUUURRR".into()).unwrap(),
            SolutionPath::new(Pos::new(0, 0), "RRRRULLLURRULLURRR".into()).unwrap(),
        ];

        test_solutions(&puzzle, solutions);
    }

    #[test]
    fn test_triangles() {
        let puzzle = Puzzle {
            triangles: [
                (Pos::new(0, 3), 3),
                (Pos::new(1, 3), 1),
                (Pos::new(2, 2), 2),
                (Pos::new(3, 2), 2),
                (Pos::new(3, 1), 3),
                (Pos::new(2, 0), 1),
            ]
            .into(),
            ..Puzzle::default_with_size(4, 4)
        };
        let solution =
            SolutionPath::new(Pos::new(0, 0), "UURULURRRDLDDLDRRRULURUU".into()).unwrap();
        test_solutions(&puzzle, vec![solution]);
    }

    #[test]
    fn test_poly() {
        let puzzle = Puzzle {
            polys: [(
                Pos::new(1, 0),
                Poly {
                    rotatable: false,
                    minos: [Pos::new(0, 0), Pos::new(1, 0)].into(),
                },
            )]
            .into(),
            ..Puzzle::default_with_size(3, 1)
        };
        let solutions = vec![
            SolutionPath::new(Pos::new(0, 0), "RURR".into()).unwrap(),
            SolutionPath::new(Pos::new(0, 0), "RRUR".into()).unwrap(),
            SolutionPath::new(Pos::new(0, 0), "URDRRU".into()).unwrap(),
            SolutionPath::new(Pos::new(0, 0), "URRDRU".into()).unwrap(),
        ];

        test_solutions(&puzzle, solutions)
    }

    #[test]
    fn test_polys() {
        let puzzle = Puzzle {
            polys: [
                (
                    Pos::new(1, 1),
                    Poly {
                        rotatable: false,
                        minos: [
                            Pos::new(0, 0),
                            Pos::new(1, 0),
                            Pos::new(0, 1),
                            Pos::new(1, 1),
                        ]
                        .into(),
                    },
                ),
                (
                    Pos::new(2, 1),
                    Poly {
                        rotatable: false,
                        minos: [Pos::new(0, 0)].into(),
                    },
                ),
            ]
            .into(),
            ..Puzzle::default_with_size(3, 3)
        };

        test_solution_count(&puzzle, 17)
    }

    #[test]
    fn test_poly_rotation() {
        let puzzle = Puzzle {
            polys: [(
                Pos::new(1, 1),
                Poly {
                    rotatable: true,
                    minos: [
                        Pos::new(0, 0),
                        Pos::new(1, 0),
                        Pos::new(0, 1),
                        Pos::new(-1, 0),
                    ]
                    .into(),
                },
            )]
            .into(),
            ..Puzzle::default_with_size(3, 3)
        };

        test_solution_count(&puzzle, 12);
    }

    #[test]
    fn test_cancel() {
        let puzzle = Puzzle {
            cancels: [(Pos::new(0, 0), 0)].into(),
            stars: [(Pos::new(1, 0), 1)].into(),
            ..Puzzle::default_with_size(4, 1)
        };

        test_solution_count(&puzzle, 8)
    }

    #[test]
    fn test_cancel_and_hexagons() {
        let puzzle = Puzzle {
            cancels: [(Pos::new(0, 1), 0)].into(),
            edge_stones: [EdgePos::new(1, 1, Direction::Up)].into(),
            vertex_stones: [Pos::new(1, 1)].into(),
            ..Puzzle::default_with_size(4, 4)
        };

        test_solution_count(&puzzle, 3678)
    }
}
