use crate::puzzle::*;

mod puzzle;
mod solver;

fn main() {
    let puzzle = Puzzle {
        ends: vec![Pos::new(0, 1), Pos::new(1, 1)],
        ..Default::default()
    };

    let mut solver = solver::BFSSolver::new(&puzzle);
    let solutions = solver.solve();

    println!("Solutions: {:?}", solutions)
}
