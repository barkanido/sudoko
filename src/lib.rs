use std::collections::HashSet;

use lazy_static::lazy_static;

pub struct Sudoku {
    pub board: Vec<Option<u32>>,
}

fn new_board() -> Vec<Option<u32>> {
    let mut board: Vec<Option<u32>> = Vec::new();
    for _ in 0..=(9 * 9) {
        board.push(None)
    }
    board
}

impl Sudoku {
    pub fn new() -> Self {
        let board = new_board();
        Self { board }
    }
}

impl From<Vec<u32>> for Sudoku {
    fn from(v: Vec<u32>) -> Self {
        let board: Vec<Option<u32>> = v
            .into_iter()
            .map(|x| if x == 0 { None } else { Some(x) })
            .collect();
        assert!(board.len() == 81);
        Self { board }
    }
}

impl std::fmt::Display for Sudoku {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        writeln!(
            f,
            "{}",
            &self
                .board
                .chunks(9)
                .map(|row| row
                    .to_vec()
                    .chunks(3)
                    .map(|block| block
                        .to_vec()
                        .iter()
                        .map(|item| item.unwrap_or_default().to_string())
                        .collect::<Vec<_>>()
                        .join(" "))
                    .collect::<Vec<_>>()
                    .join(" | "))
                .collect::<Vec<_>>()
                .chunks(3)
                .map(|group_of_rows| group_of_rows.to_vec().join("\n"))
                .collect::<Vec<_>>()
                .join("\n---------------------\n")
        )
    }
}

pub fn solve_sudoku(input: &mut Sudoku) -> bool {
    match find_first_empty(&input.board) {
        None => true,
        Some((row, col)) => {
            for option in options_for(&input.board, row, col) {
                let idx = (row * 9 + col) as usize;
                input.board[idx] = Some(option);
                if solve_sudoku(input) {
                    return true;
                }
                input.board[idx] = None;
            }
            false
        }
    }
}

fn find_first_empty(input: &[Option<u32>]) -> Option<(usize, usize)> {
    input
        .iter()
        .position(|x| x.is_none())
        .map(|idx| ((idx / 9), (idx % 9)))
}

lazy_static! {
    static ref ALL_OPTIONS: HashSet<u32> = vec![1, 2, 3, 4, 5, 6, 7, 8, 9].into_iter().collect();
}

fn options_for(board: &[Option<u32>], row: usize, col: usize) -> HashSet<u32> {
    let row_vals = row_vals(board, row);
    let col_vals = col_vals(board, col);
    let cube_vals = cube_vals(board, row, col);
    ALL_OPTIONS
        .difference(&row_vals)
        .copied()
        .collect::<HashSet<_>>()
        .difference(&col_vals)
        .copied()
        .collect::<HashSet<_>>()
        .difference(&cube_vals)
        .copied()
        .collect::<HashSet<_>>()
}

fn row_vals(board: &[Option<u32>], row: usize) -> HashSet<u32> {
    (0..9)
        .into_iter()
        .filter_map(|col| board[row * 9 + col])
        .collect()
}

fn col_vals(board: &[Option<u32>], col: usize) -> HashSet<u32> {
    (0..9)
        .into_iter()
        .filter_map(|row| board[row * 9 + col])
        .collect()
}

fn cube_vals(board: &[Option<u32>], row: usize, col: usize) -> HashSet<u32> {
    // round down to the cube boundry
    let left_col = col / 3 * 3;
    let upper_row = row / 3 * 3;
    let mut res = HashSet::new();
    for cube_col in 0..3 {
        let col = left_col + cube_col;
        for cube_row in 0..3 {
            let row = upper_row + cube_row;
            if let Some(val) = board[row * 9 + col] {
                res.insert(val);
            }
        }
    }
    res
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn find_firt_empty_test() {
        let mut board = new_board();
        board[3] = Some(5);
        assert_eq!(find_first_empty(&board), Some((0, 0)));
    }
    #[test]
    fn row_vals_test() {
        let mut board = new_board();
        let row4 = &mut board[27..=35];
        row4[3] = Some(9);
        row4[5] = Some(3);
        row4[6] = Some(2);
        let expected = vec![2, 3, 9].into_iter().collect::<HashSet<u32>>();
        assert_eq!(expected, row_vals(&board, 3));
    }

    #[test]
    fn col_vals_test() {
        let mut board = new_board();
        board[3] = Some(9);
        board[21] = Some(5);
        board[30] = Some(1);
        let expected = vec![9, 5, 1].into_iter().collect::<HashSet<u32>>();
        assert_eq!(expected, col_vals(&board, 3));
    }

    #[test]
    fn cube_vals_test() {
        let mut board = new_board();
        board[40] = Some(9);
        board[50] = Some(5);
        board[32] = Some(1);
        let expected = vec![9, 5, 1].into_iter().collect::<HashSet<u32>>();
        assert_eq!(expected, cube_vals(&board, 5, 5));
    }

    #[test]
    fn options_for_test() {
        let mut board = new_board();
        board[4] = Some(1); // 4th col
        board[13] = Some(2); // 4th col
        board[37] = Some(3); // 4th row
        board[38] = Some(4); // 4th row
        board[30] = Some(5); // middle cube
        board[50] = Some(6); // middle cube
        let expected = vec![9, 8, 7].into_iter().collect::<HashSet<u32>>();
        assert_eq!(expected, options_for(&board, 4, 4)); // a spot in the middle cubem 4th row 4th col
    }

    // fn bench_solve() {

    // let expert_board = vec![
    //     0, 0, 5, 0, 0, 0, 0, 6, 2, 0, 6, 3, 0, 0, 9, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 4, 0, 0, 0,
    //     0, 0, 6, 7, 0, 3, 0, 0, 6, 7, 0, 5, 0, 0, 0, 1, 0, 0, 8, 0, 0, 0, 0, 0, 8, 0, 1, 2, 0, 0,
    //     6, 0, 0, 0, 0, 0, 0, 0, 0, 5, 3, 0, 0, 4, 0, 0, 0, 0, 8, 0, 0,
    // ];
    // let easy_board = vec![
    //     0, 0, 0, 5, 9, 0, 0, 3, 7, 0, 7, 9, 0, 3, 2, 0, 8, 0, 0, 0, 8, 7, 0, 0, 0, 0, 0, 3, 0, 0,
    //     0, 0, 1, 6, 2, 0, 0, 9, 0, 0, 0, 6, 3, 7, 0, 7, 2, 0, 3, 0, 0, 1, 5, 8, 0, 0, 0, 0, 0, 7,
    //     8, 6, 5, 0, 8, 7, 0, 1, 0, 2, 4, 0, 0, 5, 4, 6, 0, 0, 7, 0, 0,
    // ];
    // // let mut game = Sudoku::from(expert_board);
    // let mut game = Sudoku::from(expert_board);
    // println!("{}", game);
    // solve_sudoku(&mut game);
    // println!("{}", game);
    // }
}
