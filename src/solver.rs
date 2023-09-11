#![allow(unused)]

use std::collections::{HashSet, VecDeque};

use crate::puzzle::*;

#[derive(Clone)]
pub struct PartialSolution {
    /// Candidate solution path
    path: Vec<Pos>,
    // partial_area_left: HashSet<Pos>,
    // partial_area_right: HashSet<Pos>,
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
            path: vec![start],
            // partial_area_left: HashSet::new(),
            // partial_area_right: HashSet::new(),
            // full_areas: vec![],
            // cancels_left: cancels,
            reachable_ends: ends,
        }
    }
}

pub struct BFSSolver {
    /// Initial puzzle
    puzzle: Puzzle,
    /// Queue of potential solutions, sorted from shortest to longest
    queue: VecDeque<PartialSolution>,
    /// List of solutions found so far
    solutions: Vec<Vec<Pos>>,

    // Statistics
    pub states_visited: u64,
}

impl BFSSolver {
    pub fn new(puzzle: &Puzzle) -> Self {
        Self {
            puzzle: puzzle.clone(),
            queue: VecDeque::new(),
            solutions: vec![],
            states_visited: 0,
        }
    }
    pub fn solve(&mut self) -> Vec<Vec<Pos>> {
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
            if self.queue.len() > 5_000_000 {
                println!("Exiting here for fear of OOM");
                return self.solutions.clone();
            }
            let partial_sol = self.queue.pop_front().unwrap();
            self.process_partial_solution(partial_sol);
            self.states_visited += 1;
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

            if self.stones_invalid(&new_sol) {
                continue;
            }

            // Check if we enclosed an area
            if self.puzzle.is_outer(&next) {
                // Clear the corresponding partial area
                // Floodfill area and check it
            }
            // Compute partial areas

            if self.puzzle.ends.contains(&next) {
                // With enough work on early pruning, this
                // call to is_solution could be replaced with much less
                // expensive functions for final checks only
                if self.puzzle.is_solution(&new_sol.path) {
                    self.solutions.push(new_sol.path.clone());
                }

                new_sol.reachable_ends -= 1;
                if new_sol.reachable_ends == 0 {
                    continue;
                }
            }

            self.queue.push_back(new_sol);
        }
    }

    fn validate_area(&self, cells: Vec<Pos>) -> bool {
        todo!()
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

    fn run_test(puzzle: &Puzzle, expected_solutions: Vec<Vec<Pos>>) {
        let mut solver = BFSSolver::new(puzzle);
        let solutions = solver.solve();

        assert!(solutions == expected_solutions)
    }

    #[test]
    fn test_1x1() {
        let puzzle = Puzzle::default();
        let solutions = vec![
            vec![Pos { x: 0, y: 0 }, Pos { x: 0, y: 1 }, Pos { x: 1, y: 1 }],
            vec![Pos { x: 0, y: 0 }, Pos { x: 1, y: 0 }, Pos { x: 1, y: 1 }],
        ];

        run_test(&puzzle, solutions);
    }

    #[test]
    fn test_1x1_broken() {
        let puzzle = Puzzle {
            blocked_edges: [EdgePos::new(0, 0, Direction::Up)].into(),
            ..Default::default()
        };
        let solutions = vec![vec![
            Pos { x: 0, y: 0 },
            Pos { x: 1, y: 0 },
            Pos { x: 1, y: 1 },
        ]];

        run_test(&puzzle, solutions);
    }

    #[test]
    fn test_1x1_multiple_start() {
        let puzzle = Puzzle {
            starts: vec![Pos::new(0, 0), Pos::new(0, 1)],
            ..Default::default()
        };
        let solutions = vec![
            vec![Pos { x: 0, y: 1 }, Pos { x: 1, y: 1 }],
            vec![Pos { x: 0, y: 0 }, Pos { x: 0, y: 1 }, Pos { x: 1, y: 1 }],
            vec![Pos { x: 0, y: 0 }, Pos { x: 1, y: 0 }, Pos { x: 1, y: 1 }],
            vec![
                Pos { x: 0, y: 1 },
                Pos { x: 0, y: 0 },
                Pos { x: 1, y: 0 },
                Pos { x: 1, y: 1 },
            ],
        ];

        run_test(&puzzle, solutions);
    }

    #[test]
    fn test_1x1_multiple_end() {
        let puzzle = Puzzle {
            ends: vec![Pos::new(0, 1), Pos::new(1, 1)],
            ..Default::default()
        };
        let solutions = vec![
            vec![Pos { x: 0, y: 0 }, Pos { x: 0, y: 1 }],
            vec![Pos { x: 0, y: 0 }, Pos { x: 0, y: 1 }, Pos { x: 1, y: 1 }],
            vec![Pos { x: 0, y: 0 }, Pos { x: 1, y: 0 }, Pos { x: 1, y: 1 }],
            vec![
                Pos { x: 0, y: 0 },
                Pos { x: 1, y: 0 },
                Pos { x: 1, y: 1 },
                Pos { x: 0, y: 1 },
            ],
        ];

        run_test(&puzzle, solutions);
    }

    #[test]
    fn test_1x1_stones() {
        let puzzle = Puzzle {
            vertex_stones: [Pos::new(1, 0)].into(),
            edge_stones: [EdgePos::new(0, 0, Direction::Right)].into(),
            ..Default::default()
        };
        let solutions = vec![vec![
            Pos { x: 0, y: 0 },
            Pos { x: 1, y: 0 },
            Pos { x: 1, y: 1 },
        ]];

        run_test(&puzzle, solutions);
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
        let solutions = vec![vec![Pos { x: 1, y: 0 }, Pos { x: 1, y: 1 }]];

        run_test(&puzzle, solutions);
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
            vec![
                Pos::new(1, 0),
                Pos::new(2, 0),
                Pos::new(2, 1),
                Pos::new(1, 1),
            ],
            vec![
                Pos::new(1, 0),
                Pos::new(0, 0),
                Pos::new(0, 1),
                Pos::new(1, 1),
            ],
        ];

        run_test(&puzzle, solutions);
    }

    #[test]
    fn test_broken_star_square() {
        // This puzzle is an actual puzzle example from the game that
        // contains broken paths, stars, and squares
        let puzzle = Puzzle {
            width: 4,
            height: 4,
            starts: vec![Pos::new(0, 0)],
            ends: vec![Pos::new(4, 4)],
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
            ..Default::default()
        };
        let solutions = vec![
            vec![
                Pos::new(0, 0),
                Pos::new(1, 0),
                Pos::new(2, 0),
                Pos::new(3, 0),
                Pos::new(4, 0),
                Pos::new(4, 1),
                Pos::new(3, 1),
                Pos::new(2, 1),
                Pos::new(1, 1),
                Pos::new(1, 2),
                Pos::new(2, 2),
                Pos::new(3, 2),
                Pos::new(3, 3),
                Pos::new(2, 3),
                Pos::new(2, 4),
                Pos::new(3, 4),
                Pos::new(4, 4),
            ],
            vec![
                Pos::new(0, 0),
                Pos::new(0, 1),
                Pos::new(0, 2),
                Pos::new(0, 3),
                Pos::new(1, 3),
                Pos::new(2, 3),
                Pos::new(3, 3),
                Pos::new(3, 2),
                Pos::new(2, 2),
                Pos::new(1, 2),
                Pos::new(1, 1),
                Pos::new(2, 1),
                Pos::new(3, 1),
                Pos::new(3, 0),
                Pos::new(4, 0),
                Pos::new(4, 1),
                Pos::new(4, 2),
                Pos::new(4, 3),
                Pos::new(4, 4),
            ],
            vec![
                Pos::new(0, 0),
                Pos::new(1, 0),
                Pos::new(2, 0),
                Pos::new(3, 0),
                Pos::new(4, 0),
                Pos::new(4, 1),
                Pos::new(4, 2),
                Pos::new(3, 2),
                Pos::new(3, 3),
                Pos::new(2, 3),
                Pos::new(2, 2),
                Pos::new(2, 1),
                Pos::new(1, 1),
                Pos::new(1, 2),
                Pos::new(1, 3),
                Pos::new(1, 4),
                Pos::new(2, 4),
                Pos::new(3, 4),
                Pos::new(4, 4),
            ],
            vec![
                Pos::new(0, 0),
                Pos::new(1, 0),
                Pos::new(2, 0),
                Pos::new(3, 0),
                Pos::new(4, 0),
                Pos::new(4, 1),
                Pos::new(3, 1),
                Pos::new(2, 1),
                Pos::new(1, 1),
                Pos::new(1, 2),
                Pos::new(2, 2),
                Pos::new(3, 2),
                Pos::new(3, 3),
                Pos::new(2, 3),
                Pos::new(1, 3),
                Pos::new(1, 4),
                Pos::new(2, 4),
                Pos::new(3, 4),
                Pos::new(4, 4),
            ],
        ];

        run_test(&puzzle, solutions);
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

        let solution = vec![
            Pos { x: 0, y: 0 },
            Pos { x: 0, y: 1 },
            Pos { x: 0, y: 2 },
            Pos { x: 1, y: 2 },
            Pos { x: 1, y: 3 },
            Pos { x: 0, y: 3 },
            Pos { x: 0, y: 4 },
            Pos { x: 1, y: 4 },
            Pos { x: 2, y: 4 },
            Pos { x: 3, y: 4 },
            Pos { x: 3, y: 3 },
            Pos { x: 2, y: 3 },
            Pos { x: 2, y: 2 },
            Pos { x: 2, y: 1 },
            Pos { x: 1, y: 1 },
            Pos { x: 1, y: 0 },
            Pos { x: 2, y: 0 },
            Pos { x: 3, y: 0 },
            Pos { x: 4, y: 0 },
            Pos { x: 4, y: 1 },
            Pos { x: 3, y: 1 },
            Pos { x: 3, y: 2 },
            Pos { x: 4, y: 2 },
            Pos { x: 4, y: 3 },
            Pos { x: 4, y: 4 },
        ];

        run_test(&puzzle, vec![solution]);
    }
}
