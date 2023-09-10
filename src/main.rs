use crate::puzzle::*;

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

    let mut solver = solver::BFSSolver::new(&puzzle);
    let solutions = solver.solve();

    println!(
        "Found {} solutions, visited {} states",
        solutions.len(),
        solver.states_visited
    );
    if !solutions.is_empty() {
        println!("Shortest solution: {:?}", solutions[0]);
    }
}
