// NOTE: find X -> find M -> find A -> find S

use std::{
    fs::File,
    io::{BufRead, BufReader},
};

fn check_bound(row: i32, col: i32, max_row: usize, max_col: usize) -> bool {
    if row < 0 || col < 0 || row >= max_row as i32 || col >= max_col as i32 {
        return false;
    }
    true
}

fn start_pos(
    data: &Vec<Vec<String>>,
    row: usize,
    col: usize,
    max_row: usize,
    max_col: usize,
) -> i32 {
    let dr: [i32; 8] = [-1, -1, -1, 0, 0, 1, 1, 1];
    let dc: [i32; 8] = [-1, 0, 1, -1, 1, -1, 0, 1];

    let mut count = 0;
    for i in 0..8 {
        let valid = get_pos(data, "M", row, col, dr[i], dc[i], max_row, max_col);
        if valid {
            count += 1;
        }
    }
    count
}

fn get_pos(
    data: &Vec<Vec<String>>,
    find: &str,
    row: usize,
    col: usize,
    dir_row: i32,
    dir_col: i32,
    max_row: usize,
    max_col: usize,
) -> bool {
    let new_row = row as i32 + dir_row;
    let new_col = col as i32 + dir_col;

    if !check_bound(new_row, new_col, max_row, max_col) {
        return false;
    }

    let new_row = new_row as usize;
    let new_col = new_col as usize;
    let value = data[new_row][new_col].as_str();

    if value == find {
        if value == "M" {
            return get_pos(
                data, "A", new_row, new_col, dir_row, dir_col, max_row, max_col,
            );
        }

        if value == "A" {
            return get_pos(
                data, "S", new_row, new_col, dir_row, dir_col, max_row, max_col,
            );
        }

        if value == "S" {
            return true;
        }
    };

    false
}

pub fn problem4_1(path: &str) -> i32 {
    let file = File::open(path).expect("Open file fialed");
    let reader = BufReader::new(file);

    let matrix: Vec<Vec<String>> = reader
        .lines()
        .map(|x| x.unwrap().chars().map(|c| c.to_string()).collect())
        .collect();

    let max_row = matrix.len();
    let max_col = matrix[0].len();

    let mut sum = 0;

    for i in 0..max_row {
        for j in 0..max_col {
            if matrix[i][j] == "X" {
                let check = start_pos(&matrix, i, j, max_row, max_col);
                if check >= 1 {
                    sum += check;
                }
            }
        }
    }
    sum
}
pub fn problem4_2() {}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn tests_y2024_d4_1() {
        let file_path = "testdata/y2024_p4_simple.txt";
        assert_eq!(problem4_1(file_path), 18);

        let file_path = "testdata/y2024_p4.txt";
        assert_eq!(problem4_1(file_path), 2401);
    }

    #[test]
    fn tests_y2024_d4_2() {}
}
