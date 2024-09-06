use std::time::Instant;

use crate::{puzzle::*, solver::BFSSolverConfig};

mod puzzle;
mod solver;

fn main() {
    // let puzzle = Puzzle {
    //     width: 5,
    //     height: 5,
    //     starts: vec![Pos::new(0, 0)],
    //     ends: vec![Pos::new(5, 5)],
    //     blocked_edges: [
    //         EdgePos::new(0, 2, Direction::Up),
    //         EdgePos::new(3, 2, Direction::Right),
    //     ]
    //     .into(),
    //     polys: [
    //         (
    //             Pos::new(2, 0),
    //             Poly {
    //                 rotatable: true,
    //                 minos: [
    //                     Pos::new(0, 0),
    //                     Pos::new(0, -1),
    //                     Pos::new(1, 0),
    //                     Pos::new(2, 0),
    //                 ]
    //                 .into(),
    //             },
    //         ),
    //         (
    //             Pos::new(4, 1),
    //             Poly {
    //                 rotatable: true,
    //                 minos: [
    //                     Pos::new(0, 0),
    //                     Pos::new(0, -1),
    //                     Pos::new(1, 0),
    //                     Pos::new(2, 0),
    //                 ]
    //                 .into(),
    //             },
    //         ),
    //         (
    //             Pos::new(4, 4),
    //             Poly {
    //                 rotatable: false,
    //                 minos: [
    //                     Pos::new(0, 0),
    //                     Pos::new(1, 0),
    //                     Pos::new(2, 0),
    //                     Pos::new(3, 0),
    //                 ]
    //                 .into(),
    //             },
    //         ),
    //     ]
    //     .into(),
    //     ..Default::default()
    // };

    let puzzle = Puzzle::from_json(r#"{"grid":[[{"type":"line","line":0,"dir":null},{"type":"line","line":0,"dir":null},{"type":"line","line":0,"dir":null},{"type":"line","line":0,"dir":null},{"type":"line","line":0,"dir":null},{"type":"line","line":0,"dir":null},{"type":"line","line":0,"dir":null},{"type":"line","line":0,"dir":null},{"type":"line","line":0,"dir":null},{"type":"line","line":0,"dir":null},{"type":"line","line":0,"start":true,"dir":null}],[{"type":"line","line":0,"dir":null},{"type":"square","color":"black","line":0,"dir":null},{"type":"line","line":0,"dir":null},{"type":"square","color":"black","line":0,"dir":null},{"type":"line","line":0,"dir":null},{"type":"square","color":"black","line":0,"dir":null},{"type":"line","line":0,"dir":null},{"type":"square","color":"white","line":0,"dir":null},{"type":"line","line":0,"dir":null},{"type":"square","color":"black","line":0,"dir":null},{"type":"line","line":0,"dir":null}],[{"type":"line","line":0,"dir":null},{"type":"line","line":0,"dir":null},{"type":"line","line":0,"dir":null},{"type":"line","line":0,"dir":null},{"type":"line","line":0,"dir":null},{"type":"line","line":0,"dir":null},{"type":"line","line":0,"dir":null},{"type":"line","line":0,"dir":null},{"type":"line","line":0,"dir":null},{"type":"line","line":0,"dir":null},{"type":"line","line":0,"dir":null}],[{"type":"line","line":0,"dir":null},{"type":"square","color":"black","line":0,"dir":null},{"type":"line","line":0,"dir":null},{"type":"square","color":"black","line":0,"dir":null},{"type":"line","line":0,"dir":null},{"type":"square","color":"white","line":0,"dir":null},{"type":"line","line":0,"dir":null},{"type":"square","color":"white","line":0,"dir":null},{"type":"line","line":0,"dir":null},{"type":"square","color":"white","line":0,"dir":null},{"type":"line","line":0,"dir":null}],[{"type":"line","line":0,"dir":null},{"type":"line","line":0,"dir":null},{"type":"line","line":0,"dir":null},{"type":"line","line":0,"dir":null},{"type":"line","line":0,"dir":null},{"type":"line","line":0,"dir":null},{"type":"line","line":0,"dir":null},{"type":"line","line":0,"dir":null},{"type":"line","line":0,"dir":null},{"type":"line","line":0,"dir":null},{"type":"line","line":0,"dir":null}],[{"type":"line","line":0,"dir":null},{"type":"square","color":"black","line":0,"dir":null},{"type":"line","line":0,"dir":null},{"type":"square","color":"white","line":0,"dir":null},{"type":"line","line":0,"dir":null},{"type":"square","color":"white","line":0,"dir":null},{"type":"line","line":0,"dir":null},{"type":"square","color":"white","line":0,"dir":null},{"type":"line","line":0,"dir":null},{"type":"square","color":"white","line":0,"dir":null},{"type":"line","line":0,"dir":null}],[{"type":"line","line":0,"dir":null},{"type":"line","line":0,"dir":null},{"type":"line","line":0,"dir":null},{"type":"line","line":0,"dir":null},{"type":"line","line":0,"dir":null},{"type":"line","line":0,"dir":null},{"type":"line","line":0,"dir":null},{"type":"line","line":0,"dir":null},{"type":"line","line":0,"dir":null},{"type":"line","line":0,"dir":null},{"type":"line","line":0,"dir":null}],[{"type":"line","line":0,"dir":null},{"type":"square","color":"black","line":0,"dir":null},{"type":"line","line":0,"dir":null},{"type":"square","color":"black","line":0,"dir":null},{"type":"line","line":0,"dir":null},{"type":"square","color":"white","line":0,"dir":null},{"type":"line","line":0,"dir":null},{"type":"square","color":"white","line":0,"dir":null},{"type":"line","line":0,"dir":null},{"type":"square","color":"white","line":0,"dir":null},{"type":"line","line":0,"dir":null}],[{"type":"line","line":0,"dir":null},{"type":"line","line":0,"dir":null},{"type":"line","line":0,"end":null,"dir":null},{"type":"line","line":0,"dir":null},{"type":"line","line":0,"dir":null},{"type":"line","line":0,"dir":null},{"type":"line","line":0,"dir":null},{"type":"line","line":0,"dir":null},{"type":"line","line":0,"dir":null},{"type":"line","line":0,"dir":null},{"type":"line","line":0,"dir":null}],[{"type":"line","line":0,"dir":null},{"type":"square","color":"black","line":0,"dir":null},{"type":"line","line":0,"dir":null},{"type":"square","color":"black","line":0,"dir":null},{"type":"line","line":0,"dir":null},{"type":"square","color":"black","line":0,"dir":null},{"type":"line","line":0,"dir":null},{"type":"square","color":"white","line":0,"dir":null},{"type":"line","line":0,"dir":null},{"type":"square","color":"black","line":0,"dir":null},{"type":"line","line":0,"dir":null}],[{"type":"line","line":0,"dir":null,"end":"top"},{"type":"line","line":0,"dir":null},{"type":"line","line":0,"dir":null},{"type":"line","line":0,"dir":null},{"type":"line","line":0,"dir":null},{"type":"line","line":0,"dir":null},{"type":"line","line":0,"dir":null},{"type":"line","line":0,"dir":null},{"type":"line","line":0,"dir":null},{"type":"line","line":0,"dir":null},{"type":"line","line":0,"dir":null}]],"largezero":242,"width":11,"height":11,"pillar":false,"settings":{"NEGATIONS_CANCEL_NEGATIONS":true,"SHAPELESS_ZERO_POLY":false,"PRECISE_POLYOMINOS":true,"FLASH_FOR_ERRORS":true,"FAT_STARTPOINTS":false,"CUSTOM_MECHANICS":false},"name":"Unnamed Puzzle","autoSolved":true,"hasNegations":false,"hasPolyominos":false,"startPoint":{"x":0,"y":10},"endPoint":{"x":12,"y":0},"path":[{"x":0,"y":10},2,2,3,3,1,1,3,3,2,2,3,3,2,2,3,3,2,2,4,4,2,2,4,4,2,2,4,4,1,1,4,4,2,2,2,2,3,3,3,3,3,3,1,1,3,3,1,1,3,3,2,2,2,2,0]}"#).unwrap();

    let bfssolver_config = BFSSolverConfig {
        simple_end_reachability_check: false,
        edge_stones: true,
        partial_area_check: true,
        closed_area_check: false,
        max_solutions: 0,
    };

    let mut solver = solver::BFSSolver::new(&puzzle);
    solver.config = bfssolver_config;

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
        print!("Shortest solution ({} moves): ", solutions[0].len() - 1);
        print_solution(&solutions[0]);
    }

    if solutions.len() < 10 {
        for solution in solutions {
            print_solution(&solution);
        }
    }
}
