pub mod checker;
pub mod creator;
pub mod grid;
pub mod parser;
pub mod solver;

use grid::{Grid, Value};
use std::io::{stdout, Write};

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

pub(crate) fn special_add(n: &mut usize) -> Result<(), ()> {
    if *n == 2 {
        *n = 0;
        return Err(());
    }

    *n += 1;
    Ok(())
}

pub(crate) fn special_sub(n: &mut usize) -> Result<(), ()> {
    if *n as isize - 1 < 0 {
        *n = 2;
        return Err(());
    }

    *n -= 1;
    Ok(())
}
