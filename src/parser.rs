use crate::{Grid, Value};
use std::error::Error;
use std::{fmt, fs};

#[derive(Debug)]
pub enum ParseError {
    PathNotFound,
    InvalidLayout,
    NotANumber,
    Zero,
}

impl fmt::Display for ParseError {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            ParseError::PathNotFound => write!(f, "Path Not Found"),
            ParseError::InvalidLayout => write!(f, "Invalid sudoku layer"),
            ParseError::NotANumber => write!(f, "Came across not a number"),
            ParseError::Zero => write!(f, "Came across Zero"),
        }
    }
}

impl Error for ParseError {}

pub fn parse(path: &String) -> Result<Grid, ParseError> {
    let contents = match fs::read_to_string(path) {
        Ok(cont) => cont,
        Err(_) => {
            return Err(ParseError::PathNotFound);
        }
    };

    let mut out_grid: Grid = [[[[None; 3]; 3]; 3]; 3];
    let mut s_row = 0;
    let mut row = 0;
    for line in contents.lines() {
        if row == 3 {
            return Err(ParseError::InvalidLayout);
        }

        for (n, c) in line.chars().enumerate() {
            if c == '_' {
                continue;
            }
            if n == 9 {
                return Err(ParseError::InvalidLayout);
            }
            let num = match c.to_digit(10) {
                Some(val) => val,
                None => {
                    return Err(ParseError::NotANumber);
                }
            };

            if num == 0 {
                return Err(ParseError::Zero);
            }

            out_grid[row][n / 3][s_row][n % 3] = Some(Value::Definite(num as u8));
        }
        s_row += 1;
        if s_row == 3 {
            s_row = 0;
            row += 1;
        }
    }

    Ok(out_grid)
}
