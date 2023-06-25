use crate::{
    grid::{is_in_col, is_in_row, square_need, Grid, Value},
    solver::{SolveResult, Solver},
};
use rand::{seq::SliceRandom, Rng};

fn full_grid() -> Grid {
    let mut thread = rand::thread_rng();
    let mut grid = [[[[None; 3]; 3]; 3]; 3];
    loop {
        let indexes = indexes_of_none(&grid);
        if indexes.len() == 0 {
            break;
        }
        let random = thread.gen_range(0..indexes.len());
        let num: u8 = thread.gen_range(1..=9);

        let (row, col, s_row, s_col) = *indexes.get(random).unwrap();

        if is_correct(num, &grid, row, col, s_row, s_col) {
            let mut grid_to_test = grid.clone();
            grid_to_test[row][col][s_row][s_col] = Some(Value::Definite(num));
            let mut solver = Solver::new(grid_to_test, false);
            println!("starting solve");
            match solver.solve() {
                SolveResult::Solved => grid = grid_to_test,
                SolveResult::FailedSolve => (),
                SolveResult::ManySolutions => unreachable!(),
            }
            println!("ended solve");
        }
    }

    grid
}

pub fn create_grid() -> Grid {
    let mut grid = full_grid();
    println!("full grid made");
    let mut rng = rand::thread_rng();
    let mut positions = POSSIBLE_POS.iter().map(|val| *val).collect::<IndexVec>();
    positions.shuffle(&mut rng);

    while let Some(val) = positions.pop() {
        let mut working_grid = grid.clone();
        working_grid[val.0][val.1][val.2][val.3] = None;
        let mut solver = Solver::new(working_grid, false);
        match solver.hard_solve() {
            SolveResult::Solved => grid = working_grid,
            SolveResult::ManySolutions => (),
            SolveResult::FailedSolve => unreachable!(),
        }
    }

    grid
}

const POSSIBLE_POS: [(usize, usize, usize, usize); 81] = [
    (0, 0, 0, 0),
    (0, 0, 0, 1),
    (0, 0, 0, 2),
    (0, 0, 1, 0),
    (0, 0, 1, 1),
    (0, 0, 1, 2),
    (0, 0, 2, 0),
    (0, 0, 2, 1),
    (0, 0, 2, 2),
    (0, 1, 0, 0),
    (0, 1, 0, 1),
    (0, 1, 0, 2),
    (0, 1, 1, 0),
    (0, 1, 1, 1),
    (0, 1, 1, 2),
    (0, 1, 2, 0),
    (0, 1, 2, 1),
    (0, 1, 2, 2),
    (0, 2, 0, 0),
    (0, 2, 0, 1),
    (0, 2, 0, 2),
    (0, 2, 1, 0),
    (0, 2, 1, 1),
    (0, 2, 1, 2),
    (0, 2, 2, 0),
    (0, 2, 2, 1),
    (0, 2, 2, 2),
    (1, 0, 0, 0),
    (1, 0, 0, 1),
    (1, 0, 0, 2),
    (1, 0, 1, 0),
    (1, 0, 1, 1),
    (1, 0, 1, 2),
    (1, 0, 2, 0),
    (1, 0, 2, 1),
    (1, 0, 2, 2),
    (1, 1, 0, 0),
    (1, 1, 0, 1),
    (1, 1, 0, 2),
    (1, 1, 1, 0),
    (1, 1, 1, 1),
    (1, 1, 1, 2),
    (1, 1, 2, 0),
    (1, 1, 2, 1),
    (1, 1, 2, 2),
    (1, 2, 0, 0),
    (1, 2, 0, 1),
    (1, 2, 0, 2),
    (1, 2, 1, 0),
    (1, 2, 1, 1),
    (1, 2, 1, 2),
    (1, 2, 2, 0),
    (1, 2, 2, 1),
    (1, 2, 2, 2),
    (2, 0, 0, 0),
    (2, 0, 0, 1),
    (2, 0, 0, 2),
    (2, 0, 1, 0),
    (2, 0, 1, 1),
    (2, 0, 1, 2),
    (2, 0, 2, 0),
    (2, 0, 2, 1),
    (2, 0, 2, 2),
    (2, 1, 0, 0),
    (2, 1, 0, 1),
    (2, 1, 0, 2),
    (2, 1, 1, 0),
    (2, 1, 1, 1),
    (2, 1, 1, 2),
    (2, 1, 2, 0),
    (2, 1, 2, 1),
    (2, 1, 2, 2),
    (2, 2, 0, 0),
    (2, 2, 0, 1),
    (2, 2, 0, 2),
    (2, 2, 1, 0),
    (2, 2, 1, 1),
    (2, 2, 1, 2),
    (2, 2, 2, 0),
    (2, 2, 2, 1),
    (2, 2, 2, 2),
];
type IndexVec = Vec<(usize, usize, usize, usize)>;
fn indexes_of_none(grid: &Grid) -> IndexVec {
    let mut out: IndexVec = Vec::new();

    for (r, row) in grid.iter().enumerate() {
        for (c, square) in row.iter().enumerate() {
            for (s_r, s_row) in square.iter().enumerate() {
                for (s_c, val) in s_row.iter().enumerate() {
                    if val == &None {
                        out.push((r, c, s_r, s_c));
                    }
                }
            }
        }
    }

    out
}

fn is_correct(val: u8, grid: &Grid, row: usize, col: usize, s_row: usize, s_col: usize) -> bool {
    if !square_need(&grid[row][col], &val) {
        return false;
    }
    if grid[row].iter().any(|s| is_in_row(s, s_row, &val)) {
        return false;
    }
    if grid.iter().any(|row| is_in_col(&row[col], s_col, &val)) {
        return false;
    }

    true
}

#[cfg(test)]
mod tests {

    use super::*;
    use crate::checker::check_grid;
    use crate::pretty_print;

    #[test]
    fn test_full_grid() {
        let grid = full_grid();
        pretty_print(&grid);
        assert!(check_grid(&grid))
    }
}
