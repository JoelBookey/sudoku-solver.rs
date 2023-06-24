pub mod checker;
pub mod parser;
pub mod solver;

use std::io::{stdout, Write};

pub type Square = [[Option<Value>; 3]; 3];
pub type Grid = [[Square; 3]; 3];

pub fn pretty_print(g: &Grid) {
    for row in g.iter() {
        println!("{}", "-".repeat(15));
        for i in 0..3 {
            row.iter().for_each(|l| print_row(&l[i]));
            println!();
        }
        println!("{}", "-".repeat(15));
    }
    stdout().flush().unwrap();
}

fn print_row(row: &[Option<Value>; 3]) {
    print!("|");
    for val in row.iter() {
        match val {
            Some(Value::Maybe(val)) => print!("{}", val),
            Some(Value::Definite(val)) => print!("\x1b[92m{val}\x1b[0m"),
            None => print!("_"),
        }
    }
    print!("|");
}

#[derive(PartialEq, Debug, Copy, Clone)]
pub enum Value {
    Maybe(u8),
    Definite(u8),
}

impl Value {
    pub fn unwrap(&self) -> u8 {
        match self {
            Value::Maybe(v) => *v,
            Value::Definite(v) => *v,
        }
    }
}

pub fn is_in_row(s: &Square, i: usize, val: &u8) -> bool {
    s[i].iter()
        .filter(|v| v.is_some())
        .any(|v| &v.unwrap().unwrap() == val)
}

pub fn is_in_col(s: &Square, i: usize, val: &u8) -> bool {
    s.iter()
        .filter(|col| col[i].is_some())
        .any(|col| &col[i].unwrap().unwrap() == val)
}

pub fn square_need(s: &Square, n: &u8) -> bool {
    for cell in s.iter().flatten().flatten() {
        if &cell.unwrap() == n {
            return false;
        }
    }

    true
}

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
}
