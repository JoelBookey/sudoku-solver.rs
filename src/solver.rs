use crate::{is_in_col, is_in_row, pretty_print, square_need, Grid, Value};
use std::thread::sleep;
use std::time::Duration;

const SLEEP_TIME: Duration = Duration::from_millis(200);

pub struct Solver {
    grid: Grid,
    solved: Grid,
    row: usize,
    col: usize,
    s_row: usize,
    s_col: usize,
    debug: bool,

    // only used when using hard_solve
    working_clone: Option<Grid>,
    solution_found: bool,
}

#[derive(Debug, PartialEq)]
pub enum SolveResult {
    Solved,
    FailedSolve,
    ManySolutions,
}

// results of the recursive functions
#[derive(Debug, PartialEq)]
enum RecResult {
    Back,
    Done,
    Error,
}

impl Solver {
    pub fn new(grid: Grid, debug: bool) -> Solver {
        Solver {
            grid,
            solved: grid,
            row: 0,
            col: 0,
            s_row: 0,
            s_col: 0,
            solution_found: false,
            working_clone: None,
            debug,
        }
    }

    pub fn get_solved(&self) -> Option<Grid> {
        if self.solved == self.grid {
            return None;
        }
        Some(self.solved)
    }

    pub fn solve(&mut self) -> SolveResult {
        match self.rec_solve() {
            RecResult::Back => SolveResult::FailedSolve,
            RecResult::Done => SolveResult::Solved,
            _ => unreachable!(),
        }
    }

    fn rec_solve(&mut self) -> RecResult {
        if self.is_full() {
            return RecResult::Done;
        }
        if let Some(Value::Definite(_)) = self.this_val() {
            self.next();
            let val = self.rec_solve();
            self.back();
            return val;
        }

        for num in self.list_correct().iter() {
            if self.debug {
                println!("DEBUG: ");
                pretty_print(&self.solved);
                sleep(SLEEP_TIME);
            }
            self.change_pos_to(Some(Value::Maybe(*num)));
            self.next();
            match self.rec_solve() {
                RecResult::Back => {
                    self.back();
                    continue;
                }
                RecResult::Done => {
                    self.back();
                    return RecResult::Done;
                }
                _ => unreachable!(),
            }
        }

        self.change_pos_to(None);

        RecResult::Back
    }

    // this function takes significantly longer but it returns an error if there is multiple
    // solutions
    pub fn hard_solve(&mut self) -> SolveResult {
        let res = self.hard_rec_solve();
        if res == RecResult::Back && self.solution_found {
            self.solved = self.working_clone.unwrap();
            self.working_clone = None;
            return SolveResult::Solved;
        }
        self.working_clone = None;
        if res == RecResult::Error {
            self.solved = self.grid;
            return SolveResult::ManySolutions;
        }

        SolveResult::FailedSolve
    }

    fn hard_rec_solve(&mut self) -> RecResult {
        if self.is_full() {
            if self.solution_found {
                return RecResult::Error;
            }
            self.working_clone = Some(self.solved);
            self.solution_found = true;
            self.back();
            self.change_pos_to(None);
            self.next();
            return RecResult::Back;
        }
        if let Some(Value::Definite(_)) = self.this_val() {
            self.next();
            let val = self.hard_rec_solve();
            self.back();
            return val;
        }

        for num in self.list_correct().iter() {
            if self.debug {
                println!("DEBUG: ");
                pretty_print(&self.solved);
                sleep(SLEEP_TIME);
                print!("\x1b[2J");
            }
            self.change_pos_to(Some(Value::Maybe(*num)));
            self.next();
            match self.hard_rec_solve() {
                RecResult::Back => {
                    self.back();
                    continue;
                }
                RecResult::Done => {
                    self.back();
                    return RecResult::Done;
                }
                RecResult::Error => {
                    self.back();
                    return RecResult::Error;
                }
            }
        }

        self.change_pos_to(None);
        RecResult::Back
    }

    fn this_val(&self) -> Option<Value> {
        self.solved[self.row][self.col][self.s_row][self.s_col]
    }

    fn is_full(&self) -> bool {
        !self
            .solved
            .iter()
            .flatten()
            .flatten()
            .flatten()
            .any(|val| val.is_none())
    }

    fn change_pos_to(&mut self, val: Option<Value>) {
        self.solved[self.row][self.col][self.s_row][self.s_col] = val;
    }

    fn next(&mut self) {
        if let Err(()) = special_add(&mut self.s_col) {
            if let Err(()) = special_add(&mut self.s_row) {
                if let Err(()) = special_add(&mut self.col) {
                    let _ = special_add(&mut self.row);
                }
            }
        }
    }

    fn back(&mut self) {
        if let Err(()) = special_sub(&mut self.s_col) {
            if let Err(()) = special_sub(&mut self.s_row) {
                if let Err(()) = special_sub(&mut self.col) {
                    if let Err(()) = special_sub(&mut self.row) {
                        //println!("at beginning");
                    }
                }
            }
        }
    }
    fn is_correct(&self, n: &u8) -> bool {
        if !square_need(&self.solved[self.row][self.col], n) {
            return false;
        }
        if self.is_in_grid_row(n) {
            return false;
        }
        if self.is_in_grid_col(n) {
            return false;
        }
        true
    }

    fn list_correct(&self) -> Vec<u8> {
        let mut out: Vec<u8> = Vec::new();
        for num in NUMS.iter() {
            if self.is_correct(num) {
                out.push(*num);
            }
        }

        //println!("possible nums: {out:?}");
        out
    }
    fn is_in_grid_row(&self, val: &u8) -> bool {
        self.solved[self.row]
            .iter()
            .any(|s| is_in_row(s, self.s_row, val))
    }
    fn is_in_grid_col(&self, val: &u8) -> bool {
        self.solved
            .iter()
            .any(|row| is_in_col(&row[self.col], self.s_col, val))
    }
}

fn special_add(n: &mut usize) -> Result<(), ()> {
    if *n == 2 {
        *n = 0;
        return Err(());
    }

    *n += 1;
    Ok(())
}

fn special_sub(n: &mut usize) -> Result<(), ()> {
    if *n as isize - 1 < 0 {
        *n = 2;
        return Err(());
    }

    *n -= 1;
    Ok(())
}

const NUMS: [u8; 9] = [1, 2, 3, 4, 5, 6, 7, 8, 9];

#[cfg(test)]
mod tests {

    use super::*;

    const TEST: Grid = [
        [
            [
                [None, None, Some(Value::Definite(1))],
                [None, Some(Value::Definite(4)), Some(Value::Definite(9))],
                [Some(Value::Definite(8)), None, None],
            ],
            [
                [Some(Value::Definite(8)), Some(Value::Definite(4)), None],
                [Some(Value::Definite(5)), Some(Value::Definite(7)), None],
                [Some(Value::Definite(1)), None, None],
            ],
            [
                [Some(Value::Definite(6)), None, None],
                [Some(Value::Definite(8)), None, Some(Value::Definite(1))],
                [Some(Value::Definite(5)), None, None],
            ],
        ],
        [
            [
                [None, None, None],
                [
                    Some(Value::Definite(2)),
                    Some(Value::Definite(1)),
                    Some(Value::Definite(8)),
                ],
                [Some(Value::Definite(4)), None, None],
            ],
            [
                [None, Some(Value::Definite(6)), Some(Value::Definite(1))],
                [
                    Some(Value::Definite(4)),
                    Some(Value::Definite(5)),
                    Some(Value::Definite(7)),
                ],
                [None, None, None],
            ],
            [
                [Some(Value::Definite(2)), None, None],
                [
                    Some(Value::Definite(3)),
                    Some(Value::Definite(9)),
                    Some(Value::Definite(6)),
                ],
                [Some(Value::Definite(1)), None, Some(Value::Definite(7))],
            ],
        ],
        [
            [
                [Some(Value::Definite(3)), None, None],
                [Some(Value::Definite(1)), None, None],
                [None, None, Some(Value::Definite(2))],
            ],
            [
                [None, Some(Value::Definite(1)), Some(Value::Definite(5))],
                [None, None, None],
                [Some(Value::Definite(7)), None, None],
            ],
            [
                [None, Some(Value::Definite(2)), Some(Value::Definite(8))],
                [None, Some(Value::Definite(6)), None],
                [Some(Value::Definite(4)), Some(Value::Definite(1)), None],
            ],
        ],
    ];

    use Value::*;
    const SOLVED: Grid = [
        [
            [
                [Some(Maybe(5)), Some(Maybe(3)), Some(Definite(1))],
                [Some(Maybe(6)), Some(Definite(4)), Some(Definite(9))],
                [Some(Definite(8)), Some(Maybe(2)), Some(Maybe(7))],
            ],
            [
                [Some(Definite(8)), Some(Definite(4)), Some(Maybe(9))],
                [Some(Definite(5)), Some(Definite(7)), Some(Maybe(2))],
                [Some(Definite(1)), Some(Maybe(3)), Some(Maybe(6))],
            ],
            [
                [Some(Definite(6)), Some(Maybe(7)), Some(Maybe(2))],
                [Some(Definite(8)), Some(Maybe(3)), Some(Definite(1))],
                [Some(Definite(5)), Some(Maybe(4)), Some(Maybe(9))],
            ],
        ],
        [
            [
                [Some(Maybe(7)), Some(Maybe(5)), Some(Maybe(3))],
                [Some(Definite(2)), Some(Definite(1)), Some(Definite(8))],
                [Some(Definite(4)), Some(Maybe(9)), Some(Maybe(6))],
            ],
            [
                [Some(Maybe(9)), Some(Definite(6)), Some(Definite(1))],
                [Some(Definite(4)), Some(Definite(5)), Some(Definite(7))],
                [Some(Maybe(3)), Some(Maybe(2)), Some(Maybe(8))],
            ],
            [
                [Some(Definite(2)), Some(Maybe(8)), Some(Maybe(4))],
                [Some(Definite(3)), Some(Definite(9)), Some(Definite(6))],
                [Some(Definite(1)), Some(Maybe(5)), Some(Definite(7))],
            ],
        ],
        [
            [
                [Some(Definite(3)), Some(Maybe(7)), Some(Maybe(4))],
                [Some(Definite(1)), Some(Maybe(8)), Some(Maybe(5))],
                [Some(Maybe(9)), Some(Maybe(6)), Some(Definite(2))],
            ],
            [
                [Some(Maybe(6)), Some(Definite(1)), Some(Definite(5))],
                [Some(Maybe(2)), Some(Maybe(9)), Some(Maybe(4))],
                [Some(Definite(7)), Some(Maybe(8)), Some(Maybe(3))],
            ],
            [
                [Some(Maybe(9)), Some(Definite(2)), Some(Definite(8))],
                [Some(Maybe(7)), Some(Definite(6)), Some(Maybe(3))],
                [Some(Definite(4)), Some(Definite(1)), Some(Maybe(5))],
            ],
        ],
    ];
    #[test]
    fn test_grid_row_check() {
        let s = Solver::new(TEST, false);
        assert!(s.is_in_grid_row(&6));
    }
    #[test]
    fn test_grid_col_check() {
        let s = Solver::new(TEST, false);
        assert!(s.is_in_grid_col(&8));
    }

    #[test]
    fn test_is_full() {
        let s = Solver::new(TEST, false);
        assert!(!s.is_full());
    }

    #[test]
    fn test_is_correct() {
        let mut s = Solver::new(TEST, false);
        assert!(!s.is_correct(&1));
        s.next();
        s.next();
        s.next();
        //println!("this val: {:?}", s.this_val());
        //println!("{:?}", (s.s_col, s.s_row));
        assert!(!s.is_correct(&7));
    }

    #[test]
    fn test_this_val() {
        let mut s = Solver::new(TEST, false);
        s.next();
        s.next();
        assert_eq!(s.this_val(), Some(Value::Definite(1)));
    }

    #[test]
    fn test_solve() {
        let mut solve = Solver::new(TEST, false);
        solve.rec_solve();
        assert_eq!(&solve.get_solved().unwrap(), &SOLVED);
    }
}
