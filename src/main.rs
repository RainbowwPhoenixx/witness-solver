use crate::puzzle::*;

mod puzzle;
mod solver;

fn main() {
    let puzzle = Puzzle {
        width: 5,
        height: 5,
        ends: vec![Pos::new(5, 5)],
        edge_stones: [EdgePos::new(2, 2, Direction::Up)].into(),
        ..Default::default()
    };

    let mut solver = solver::BFSSolver::new(&puzzle);
    let solutions = solver.solve();

    println!("Found {} solutions, visited {} states", solutions.len(), solver.states_visited);
    if !solutions.is_empty() {
        println!("Shortest solution: {:?}", solutions[0]);
    }
}
