use std::time::Instant;

use crate::{puzzle::*, solver::BFSSolverConfig};

mod puzzle;
mod solver;

fn main() {
    let puzzle = Puzzle::from_json(r#"{"grid":[[{"type":"line","line":0,"dir":null},{"type":"line","line":0,"dir":null},{"type":"line","line":0,"dir":null},{"type":"line","line":0,"dir":null,"gap":2,"dot":null,"start":null,"end":null},{"type":"line","line":0,"dir":null,"start":false,"end":null},{"type":"line","line":0,"dir":null,"gap":2,"dot":null,"start":null,"end":null},{"type":"line","line":1,"dir":"right"},{"type":"line","line":1,"dir":"top"},{"type":"line","line":1,"dir":"top"},{"type":"line","line":0,"dir":null},{"type":"line","line":1,"dir":"right"},{"type":"line","line":1,"dir":"top"},{"type":"line","line":1,"dir":"top"},{"type":"line","line":0,"dir":null},{"type":"line","line":0,"dir":null},{"type":"line","line":0,"dir":null},{"type":"line","line":1,"start":true,"dir":"right"}],[{"type":"line","line":0,"dir":null},{"type":"square","color":"red","line":0,"dir":null},{"type":"line","line":0,"dir":null},null,{"type":"line","line":0,"dir":null,"gap":2,"dot":null,"start":null,"end":null},null,{"type":"line","line":1,"dir":"right"},{"type":"square","color":"white","line":0,"dir":null},{"type":"line","line":1,"dir":"left"},{"type":"square","color":"purple","line":0,"dir":null},{"type":"line","line":1,"dir":"right"},{"type":"square","color":"purple","line":0,"dir":null},{"type":"line","line":1,"dir":"left"},{"type":"square","color":"purple","line":0,"dir":null},{"type":"line","line":0,"dir":null},{"type":"square","color":"purple","line":0,"dir":null},{"type":"line","line":1,"dir":"right"}],[{"type":"line","line":1,"dir":"right"},{"type":"line","line":1,"dir":"top"},{"type":"line","line":1,"dir":"top"},{"type":"line","line":0,"dir":null},{"type":"line","line":1,"dir":"right"},{"type":"line","line":1,"dir":"top"},{"type":"line","line":1,"dir":"top"},{"type":"line","line":0,"dir":null},{"type":"line","line":1,"dir":"left"},{"type":"line","line":0,"dir":null},{"type":"line","line":1,"dir":"right"},{"type":"line","line":0,"dir":null},{"type":"line","line":1,"dir":"left"},{"type":"line","line":0,"dir":null},{"type":"line","line":1,"dir":"right"},{"type":"line","line":1,"dir":"top"},{"type":"line","line":1,"dir":"top"}],[{"type":"line","line":1,"dir":"right"},{"type":"square","color":"white","line":0,"dir":null},{"type":"line","line":1,"dir":"left"},{"type":"square","color":"red","line":0,"dir":null},{"type":"line","line":1,"dir":"right"},{"type":"square","color":"white","line":0,"dir":null},{"type":"line","line":0,"dir":null},{"type":"square","color":"white","line":0,"dir":null},{"type":"line","line":1,"dir":"left"},{"type":"square","color":"purple","line":0,"dir":null},{"type":"line","line":1,"dir":"right"},{"type":"square","color":"purple","line":0,"dir":null},{"type":"line","line":1,"dir":"left"},{"type":"square","color":"purple","line":0,"dir":null},{"type":"line","line":1,"dir":"right"},{"type":"square","color":"red","line":0,"dir":null},{"type":"line","line":0,"dir":null}],[{"type":"line","line":1,"dir":null,"end":"top"},{"type":"line","line":0,"dir":null},{"type":"line","line":1,"dir":"left"},{"type":"line","line":1,"dir":"top"},{"type":"line","line":1,"dir":"top"},{"type":"line","line":0,"dir":null},{"type":"line","line":0,"dir":null},{"type":"line","line":0,"dir":null},{"type":"line","line":1,"dir":"left"},{"type":"line","line":0,"dir":null},{"type":"line","line":1,"dir":"right"},{"type":"line","line":0,"dir":null},{"type":"line","line":1,"dir":"left"},{"type":"line","line":0,"dir":null},{"type":"line","line":1,"dir":"right"},{"type":"line","line":0,"dir":null},{"type":"line","line":0,"dir":null}],[{"type":"line","line":0,"dir":null,"gap":2,"dot":null,"start":null,"end":null},null,{"type":"line","line":0,"dir":null,"gap":2,"dot":null,"start":null,"end":null},null,{"type":"line","line":0,"dir":null},{"type":"square","color":"white","line":0,"dir":null},{"type":"line","line":0,"dir":null},{"type":"square","color":"white","line":0,"dir":null},{"type":"line","line":1,"dir":"left"},{"type":"square","color":"purple","line":0,"dir":null},{"type":"line","line":1,"dir":"right"},{"type":"square","color":"purple","line":0,"dir":null},{"type":"line","line":1,"dir":"left"},{"type":"square","color":"purple","line":0,"dir":null},{"type":"line","line":1,"dir":"right"},{"type":"square","color":"red","line":0,"dir":null},{"type":"line","line":0,"dir":null}],[{"type":"line","line":0,"dir":null,"start":false,"end":null},{"type":"line","line":0,"dir":null,"gap":2,"dot":null,"start":null,"end":null},{"type":"line","line":0,"dir":null,"start":false,"end":null},{"type":"line","line":0,"dir":null,"gap":2,"dot":null,"start":null,"end":null},{"type":"line","line":0,"dir":null},{"type":"line","line":0,"dir":null},{"type":"line","line":0,"dir":null},{"type":"line","line":0,"dir":null},{"type":"line","line":1,"dir":"left"},{"type":"line","line":0,"dir":null},{"type":"line","line":1,"dir":"right"},{"type":"line","line":0,"dir":null},{"type":"line","line":1,"dir":"left"},{"type":"line","line":0,"dir":null},{"type":"line","line":1,"dir":"down"},{"type":"line","line":1,"dir":"down"},{"type":"line","line":1,"dir":"right"}],[{"type":"line","line":0,"dir":null,"gap":2,"dot":null,"start":null,"end":null},null,{"type":"line","line":0,"dir":null,"gap":2,"dot":null,"start":null,"end":null},null,{"type":"line","line":0,"dir":null,"gap":2,"dot":null,"start":null,"end":null},null,{"type":"line","line":0,"dir":null},{"type":"square","color":"white","line":0,"dir":null},{"type":"line","line":1,"dir":"left"},{"type":"square","color":"purple","line":0,"dir":null},{"type":"line","line":1,"dir":"right"},{"type":"square","color":"purple","line":0,"dir":null},{"type":"line","line":1,"dir":"left"},{"type":"square","color":"purple","line":0,"dir":null},{"type":"line","line":0,"dir":null},{"type":"square","color":"purple","line":0,"dir":null},{"type":"line","line":1,"dir":"right"}],[{"type":"line","line":0,"dir":null,"start":false,"end":null},{"type":"line","line":0,"dir":null,"gap":2,"dot":null,"start":null,"end":null},{"type":"line","line":0,"dir":null,"start":false,"end":null},{"type":"line","line":0,"dir":null,"gap":2,"dot":null,"start":null,"end":null},{"type":"line","line":0,"dir":null,"start":false,"end":null},{"type":"line","line":0,"dir":null,"gap":2,"dot":null,"start":null,"end":null},{"type":"line","line":0,"dir":null},{"type":"line","line":0,"dir":null},{"type":"line","line":1,"dir":"left"},{"type":"line","line":1,"dir":"top"},{"type":"line","line":1,"dir":"top"},{"type":"line","line":0,"dir":null},{"type":"line","line":1,"dir":"left"},{"type":"line","line":1,"dir":"top"},{"type":"line","line":1,"dir":"top"},{"type":"line","line":1,"dir":"top"},{"type":"line","line":1,"dir":"top"}]],"largezero":306,"width":9,"height":17,"pillar":false,"settings":{"NEGATIONS_CANCEL_NEGATIONS":true,"SHAPELESS_ZERO_POLY":false,"PRECISE_POLYOMINOS":true,"FLASH_FOR_ERRORS":true,"FAT_STARTPOINTS":false,"CUSTOM_MECHANICS":false},"name":"Unnamed Puzzle","hasNegations":false,"hasPolyominos":false,"startPoint":{"x":0,"y":16},"endPoint":{"x":4,"y":0},"autoSolved":true,"path":[{"x":0,"y":16},2,2,3,3,2,2,2,2,4,4,2,2,3,3,3,3,1,1,1,1,1,1,1,1,3,3,2,2,2,2,2,2,2,2,3,3,1,1,1,1,1,1,1,1,3,3,2,2,3,3,2,2,3,3,1,1,3,3,2,2,0]}"#).unwrap();

    let bfssolver_config = BFSSolverConfig {
        simple_end_reachability_check: true,
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
        println!(
            "Shortest solution ({} moves): {}",
            solutions[0].len() - 1,
            solutions[0]
        );
    }

    for solution in &solutions[..10] {
        println!("{} moves: {}", solution.len() - 1, solution);
    }
}
