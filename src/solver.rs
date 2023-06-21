type Square = [[Option<Value>; 3]; 3];
type Grid = [[Square; 3]; 3];

pub struct Solver<'a> {
    grid: &'a mut Grid,
    row: usize,
    col: usize,
    s_row: usize,
    s_col: usize,
}

#[derive(Debug, PartialEq)]
pub enum SolveResult {
    Back,
    Done,
}

impl<'a> Solver<'a> {
    pub fn new(grid: &'a mut Grid) -> Solver<'a> {
        Solver {
            grid,
            row: 0,
            col: 0,
            s_row: 0,
            s_col: 0,
        }
    }

    pub fn rec_solve(&mut self) -> SolveResult {
        if self.is_full() {
            return SolveResult::Done;
        }
        if let Some(Value::Definite(_)) = self.this_val() {
            self.next();
            let val = self.rec_solve();
            self.back();
            return val;
        }

        for num in self.list_correct().iter() {
            self.change_pos_to(Some(Value::Maybe(*num)));
            self.next();
            match self.rec_solve() {
                SolveResult::Back => {
                    self.back();
                    continue;
                }
                SolveResult::Done => {
                    self.back();
                    return SolveResult::Done;
                }
            }
        }

        self.change_pos_to(None);

        SolveResult::Back
    }

    fn this_val(&self) -> Option<Value> {
        self.grid[self.row][self.col][self.s_row][self.s_col]
    }

    fn is_full(&self) -> bool {
        !self
            .grid
            .iter()
            .flatten()
            .flatten()
            .flatten()
            .any(|val| val == &None)
    }

    fn change_pos_to(&mut self, val: Option<Value>) {
        self.grid[self.row][self.col][self.s_row][self.s_col] = val;
    }

    fn next(&mut self) {
        if let Err(()) = special_add(&mut self.s_col) {
            if let Err(()) = special_add(&mut self.s_row) {
                if let Err(()) = special_add(&mut self.col) {
                    if let Err(()) = special_add(&mut self.row) {
                        //                       println!("at end");
                    }
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
        if !square_need(&self.grid[self.row][self.col], n) {
            return false;
        }
        if is_in_grid_row(&self.grid, self.row * 3 + self.s_row, n) {
            return false;
        }
        if is_in_grid_col(&self.grid, self.col * 3 + self.s_col, n) {
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

fn is_in_row(s: &Square, i: usize, val: &u8) -> bool {
    s[i].iter()
        .filter(|v| v.is_some())
        .any(|v| &unwrap_val(&v.unwrap()) == val)
}

fn is_in_col(s: &Square, i: usize, val: &u8) -> bool {
    s.iter()
        .filter(|col| col[i].is_some())
        .find(|col| &unwrap_val(&col[i].unwrap()) == val)
        .is_some()
}

fn is_in_grid_row(g: &Grid, i: usize, val: &u8) -> bool {
    let r_index = i / 3;
    let s_index = i % 3;
    g[r_index]
        .iter()
        .find(|s| is_in_row(s, s_index, val))
        .is_some()
}

fn is_in_grid_col(g: &Grid, i: usize, val: &u8) -> bool {
    g.iter()
        .find(|row| is_in_col(&row[i / 3], i % 3, val))
        .is_some()
}

fn pretty_print(g: &Grid) {
    for row in g.iter() {
        println!("{}", "-".repeat(27));
        for i in 0..3 {
            row.iter().for_each(|l| print!("{:?}", l[i]));
            println!();
        }
    }
}

fn square_need(s: &Square, n: &u8) -> bool {
    for row in s.iter() {
        for cell in row.iter() {
            if let Some(val) = cell {
                if &unwrap_val(val) == n {
                    return false;
                }
            }
        }
    }

    true
}

#[derive(PartialEq, Debug, Copy, Clone)]
pub enum Value {
    Maybe(u8),
    Definite(u8),
}

fn unwrap_val(val: &Value) -> u8 {
    match val {
        Value::Maybe(v) => *v,
        Value::Definite(v) => *v,
    }
}

const NUMS: [u8; 9] = [1, 2, 3, 4, 5, 6, 7, 8, 9];

#[cfg(test)]
mod tests {

    use super::*;

    const SQ2: Square = [
        [
            Some(Value::Maybe(9)),
            Some(Value::Maybe(8)),
            Some(Value::Definite(7)),
        ],
        [None, None, None],
        [None, None, None],
    ];
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
    fn test_col() {
        assert!(is_in_col(&SQ2, 2, &7));
        assert_eq!(is_in_col(&SQ2, 0, &3), false);
    }

    #[test]
    fn test_row() {
        assert!(is_in_row(&SQ2, 0, &8));
        assert_eq!(is_in_row(&SQ2, 1, &4), false);
    }

    #[test]
    fn test_square_need() {
        assert!(square_need(&SQ2, &5));
        assert_eq!(square_need(&SQ2, &7), false);
    }

    #[test]
    fn test_grid_row_check() {
        let g = [[SQ2; 3]; 3];
        assert!(is_in_grid_row(&TEST, 1, &7));
        assert!(is_in_grid_row(&g, 3, &8));
        assert!(is_in_grid_row(&g, 6, &7));
        assert!(!is_in_grid_row(&g, 2, &8));
    }
    #[test]
    fn test_grid_col_check() {
        let g = [[SQ2; 3]; 3];
        assert!(is_in_grid_col(&g, 2, &7));
        assert!(is_in_grid_col(&g, 6, &9));
        assert!(!is_in_grid_col(&g, 2, &8));
    }

    #[test]
    fn test_is_full() {
        let mut test2 = TEST.clone();
        let s = Solver::new(&mut test2);
        assert!(!s.is_full());
    }

    #[test]
    fn test_is_correct() {
        let mut test2 = TEST.clone();
        let mut s = Solver::new(&mut test2);
        assert!(!s.is_correct(&1));
        drop(s.next());
        drop(s.next());
        drop(s.next());
        //println!("this val: {:?}", s.this_val());
        //println!("{:?}", (s.s_col, s.s_row));
        assert!(!s.is_correct(&7));
    }

    #[test]
    fn test_this_val() {
        let mut test2 = TEST.clone();
        let mut s = Solver::new(&mut test2);
        drop(s.next());
        drop(s.next());
        assert_eq!(s.this_val(), Some(Value::Definite(1)));
    }

    #[test]
    fn test_solve() {
        let mut test2 = TEST.clone();
        let mut solve = Solver::new(&mut test2);
        solve.rec_solve();
        assert_eq!(solve.grid, &SOLVED);
    }
}
