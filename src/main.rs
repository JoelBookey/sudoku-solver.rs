use clap::{Parser, Subcommand};
use std::time::Instant;
use sudoku::checker::check_grid;
use sudoku::creator::create_grid;
use sudoku::parser::parse;
use sudoku::pretty_print;
use sudoku::solver::{SolveResult, Solver};

#[derive(Parser, Debug)]
struct Cli {
    #[command(subcommand)]
    command: Commands,
}

#[derive(Subcommand, Debug)]
enum Commands {
    // solve a sudoku puzzle
    Solve { path: String },
    // check if a sudoku puzzle is correct
    Check { path: String },
    // create a sudoku puzzle
    Create,
}

fn main() -> Result<(), String> {
    let args = Cli::parse();
    if let Commands::Solve { path: file_path } = args.command {
        //if true {
        let grid = match parse(&file_path) {
            Ok(val) => val,
            Err(e) => return Err(e.to_string()),
        };
        let mut solver = Solver::new(grid);
        match solver.hard_solve() {
            SolveResult::FailedSolve => return Err("could not solve".to_string()),
            SolveResult::ManySolutions => return Err("multiple solutions found".to_string()),
            _ => {}
        }

        println!("SOLVED: ");
        pretty_print(&solver.get_solved().unwrap());
    } else if let Commands::Check { path: file_path } = args.command {
        let grid = match parse(&file_path) {
            Ok(val) => val,
            Err(e) => return Err(e.to_string()),
        };
        match check_grid(&grid) {
            true => println!("The given grid is correct."),
            false => println!("The given grid is incorrect"),
        }
    } else if let Commands::Create = args.command {
        let now = Instant::now();
        let grid = create_grid();
        pretty_print(&grid);
        println!("Took: {:?}", now.elapsed());
    } else {
        return Err("NOT A COMMMAND!!".to_string());
    }

    Ok(())
}
