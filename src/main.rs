// ACCESS using square/grid [y][x]

type Square = [[u8; 3]; 3];
type Grid = [[Square; 3]; 3];

fn is_in_row(s: &Square, i: usize, val: &u8) -> bool {
    s[i].contains(val)
}

fn is_in_col(s: &Square, i: usize, val: &u8) -> bool {
    s.iter().find(|col| &col[i] == val).is_some()
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

// [[_, _, _]
// [_ ,_, _]
// [_, _, _]
// [[_, _, _]
// [_ ,_, _]
// [_, _, _]
// [[_, _, _]
// [_ ,_, _]
// [_, _, _]

const sqr: Square = [[1, 1, 1], [2, 2, 2], [3, 3, 3]];
const sq2: Square = [[9, 8, 7], [0, 0, 0], [0, 0, 0]];
const nums: [u8; 9] = [1, 2, 3, 4, 5, 6, 7, 8, 9];

fn main() {
    let mut grid = [[sqr; 3]; 3];
    grid[2][1] = sq2;
    pretty_print(&grid);
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

fn nums_needed(s: &Square) -> Vec<u8> {
    let mut out: Vec<u8> = Vec::new();
    for num in nums.iter() {
        if square_need(s, num) {
            out.push(*num);
        }
    }
    out
}

fn square_need(s: &Square, n: &u8) -> bool {
    for row in s.iter() {
        for cell in row.iter() {
            if cell == n {
                return false;
            }
        }
    }

    true
}

// for square in grid ...
//      if not empty then
//          for cell in square
//
//
//

#[cfg(test)]
mod tests {

    use super::{
        is_in_col, is_in_grid_col, is_in_grid_row, is_in_row, nums_needed, sq2, square_need,
    };

    #[test]
    fn test_col() {
        let s = [[0, 0, 2], [0, 3, 0], [0, 0, 0]];
        assert!(is_in_col(&s, 2, &2));
        assert_eq!(is_in_col(&s, 0, &3), false);
    }

    #[test]
    fn test_row() {
        let s = [[0, 0, 0], [0, 6, 0], [0, 0, 0]];
        assert!(is_in_row(&s, 1, &6));
        assert_eq!(is_in_row(&s, 0, &4), false);
    }

    #[test]
    fn test_square_need() {
        let s = [[1, 3, 4], [0, 0, 0], [2, 7, 9]];
        assert!(square_need(&s, &5));
        assert_eq!(square_need(&s, &7), false);
    }

    #[test]
    fn test_nums_needed() {
        let s = [[1, 3, 4], [0, 0, 0], [2, 7, 9]];
        assert_eq!(nums_needed(&s), vec![5, 6, 8]);
        assert_ne!(nums_needed(&s), vec![5, 6, 9]);
    }

    #[test]
    fn test_grid_row_check() {
        let g = [[sq2; 3]; 3];
        assert!(is_in_grid_row(&g, 3, &8));
        assert!(is_in_grid_row(&g, 6, &7));
        assert!(!is_in_grid_row(&g, 2, &8));
    }
    #[test]
    fn test_grid_col_check() {
        let g = [[sq2; 3]; 3];
        assert!(is_in_grid_col(&g, 2, &7));
        assert!(is_in_grid_col(&g, 6, &9));
        assert!(!is_in_grid_col(&g, 2, &8));
    }
}
