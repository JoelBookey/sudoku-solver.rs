use clap::Parser;
use sudoku_solver::parser::parse;
use sudoku_solver::pretty_print;
use sudoku_solver::solver::{SolveResult, Solver};

#[derive(Parser, Debug)]
struct Args {
    file_path: String,
    arg_1: Option<String>,
    arg_2: Option<String>,
}

fn match_arg(s: Option<String>, debug: &mut bool, hard_solve: &mut bool) -> Result<(), String> {
    match s {
        None => {}
        Some(s) => {
            if s == *"--debug" {
                *debug = true;
            } else if s == *"--hard" {
                *hard_solve = true;
            } else {
                return Err(format!("unexpected argument: {s}"));
            }
        }
    };
    Ok(())
}

fn main() -> Result<(), String> {
    let args = Args::parse();
    let file_path = args.file_path;
    let mut debug = false;
    let mut hard_solve = false;
    match_arg(args.arg_1, &mut debug, &mut hard_solve)?;
    match_arg(args.arg_2, &mut debug, &mut hard_solve)?;

    let grid = match parse(&file_path) {
        Ok(val) => val,
        Err(e) => return Err(e.to_string()),
    };

    let mut solver = Solver::new(grid, debug);
    if hard_solve {
        match solver.hard_solve() {
            SolveResult::FailedSolve => return Err("could not solve".to_string()),
            SolveResult::ManySolutions => return Err("multiple solutions found".to_string()),
            _ => {}
        }
    } else if solver.solve() == SolveResult::FailedSolve {
        return Err("could not solve".to_string());
    }
    println!("SOLVED: ");
    pretty_print(&solver.get_solved().unwrap());

    Ok(())
}
