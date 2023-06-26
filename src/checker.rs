use crate::grid::{is_full, is_in_col, is_in_row, square_need, Grid};

pub fn check_grid(grid: &Grid) -> bool {
    if !is_full(grid) {
        return false;
    }
    for (row_index, row) in grid.iter().enumerate() {
        for (col_index, square) in row.iter().enumerate() {
            for (s_row_i, s_row) in square.iter().enumerate() {
                for (i, wrapped) in s_row.iter().enumerate() {
                    if let Some(val) = wrapped {
                        let mut check_grid = grid.clone();
                        check_grid[row_index][col_index][s_row_i][i] = None;
                        if !is_val_correct(
                            check_grid,
                            val.unwrap(),
                            row_index,
                            col_index,
                            s_row_i,
                            i,
                        ) {
                            return false;
                        }
                    }
                }
            }
        }
    }

    true
}

fn is_val_correct(grid: Grid, val: u8, row: usize, col: usize, s_row: usize, s_col: usize) -> bool {
    if !square_need(&grid[row][col], &val) {
        return false;
    }
    if grid[row]
        .iter()
        .any(|square| is_in_row(square, s_row, &val))
    {
        return false;
    }
    if grid.iter().any(|r| is_in_col(&r[col], s_col, &val)) {
        return false;
    }

    true
}
#[cfg(test)]
mod tests {

    use super::*;
    use crate::Value;

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

    #[test]
    fn incorrect_check() {
        let mut test = TEST.clone();
        test[0][0][1][2] = Some(Value::Definite(1));
        assert!(!check_grid(&test));
    }
}
