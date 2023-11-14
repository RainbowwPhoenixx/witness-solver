use std::time::Instant;

use crate::{puzzle::*, solver::BFSSolverConfig};

mod puzzle;
mod solver;

fn main() {
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

    let config = BFSSolverConfig {
        simple_end_reachability_check: true,
        edge_stones: true,
    };

    let mut solver = solver::BFSSolver::new(&puzzle);
    solver.config = config;

    let before = Instant::now();
    let solutions = solver.solve();
    let solve_time = before.elapsed();

    println!(
        "Found {} solutions and visited {} states in {:?}",
        solutions.len(),
        solver.states_visited,
        solve_time,
    );

    if !solutions.is_empty() {
        print!("Shortest solution: ");
        print_solution(&solutions[0]);
    }
}
