use std::env;
use sudoku_solver::parser::parse;
use sudoku_solver::pretty_print;
use sudoku_solver::solver::{SolveResult, Solver};

fn main() -> Result<(), String> {
    // println!("hello");
    // println!("\x1b[93mGoodbye\x1b[0m");

    let args = env::args().collect::<Vec<String>>();
    let path = args.get(1);
    if path.is_none() {
        return Err("please provide a file!".to_string());
    }

    let mut debug = false;
    if let Some(arg) = args.get(2) {
        if arg == "debug" {
            debug = true;
        } else {
            return Err(format!("unexpected argument: {arg}").to_string());
        }
    }

    let mut g = match parse(path.unwrap()) {
        Ok(val) => val,
        Err(e) => return Err(e.to_string()),
    };

    let mut solver = Solver::new(&mut g, debug);
    if solver.rec_solve() == SolveResult::Back {
        return Err("could not solve".to_string());
    }
    pretty_print(solver.grid);

    Ok(())
}
