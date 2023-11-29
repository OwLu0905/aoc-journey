use std::{
    fs::File,
    io::{BufRead, BufReader},
};

pub fn problem8_1(path: &str) -> i32 {
    let file = File::open(path).expect("cant read file");
    let reader = BufReader::new(file);

    let matrix: Vec<Vec<u8>> = reader
        .lines()
        .map(|line_result| {
            line_result
                .unwrap()
                .chars()
                .filter_map(|c| c.to_digit(10).map(|digit| digit as u8))
                .collect()
        })
        .collect();
    let row_len = &matrix.len();
    let col_len = &matrix[0].len();

    let mut check_matrix: Vec<Vec<i32>> = vec![vec![0; *col_len]; *row_len];

    let mut row_max_vec = vec![0; *row_len];
    let mut col_max_vec = vec![0; *col_len];

    for i in 0..*row_len {
        let max_row_value = row_max_vec.get_mut(i).unwrap();
        for j in 0..*col_len {
            let max_col_value = col_max_vec.get_mut(j).unwrap();
            let cuurent_value = matrix[i][j];
            if cuurent_value > *max_row_value || i == 0 || i == *row_len - 1 {
                *max_row_value = cuurent_value;
                if let Some(row) = check_matrix.get_mut(i) {
                    if let Some(value) = row.get_mut(j) {
                        *value = 1;
                    }
                }
            }
            if cuurent_value > *max_col_value || j == 0 || j == *col_len - 1 {
                *max_col_value = cuurent_value;
                if let Some(row) = check_matrix.get_mut(i) {
                    if let Some(value) = row.get_mut(j) {
                        *value = 1;
                    }
                }
            }
        }
    }

    let mut row_max_vec = vec![0; *row_len];
    let mut col_max_vec = vec![0; *col_len];

    let mut count = 0;
    for i in (0..*row_len).rev() {
        let max_row_value = row_max_vec.get_mut(i).expect("cant");
        for j in (0..*col_len).rev() {
            let max_col_value = col_max_vec.get_mut(j).expect("can't");
            let cuurent_value = matrix[i][j];
            if cuurent_value > *max_row_value || i == 0 || i == *row_len - 1 {
                *max_row_value = cuurent_value;
                if let Some(row) = check_matrix.get_mut(i) {
                    if let Some(value) = row.get_mut(j) {
                        *value = 1;
                    }
                }
            }
            if cuurent_value > *max_col_value || j == 0 || j == *col_len - 1 {
                *max_col_value = cuurent_value;
                if let Some(row) = check_matrix.get_mut(i) {
                    if let Some(value) = row.get_mut(j) {
                        *value = 1;
                    }
                }
            }
            if check_matrix[i][j] == 1 {
                count += 1;
            }
        }
    }

    count
}
pub fn problem8_2() {}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn tests_y2022_d8_1() {
        let file_path_simple = "testdata/y2022_p8_simple.txt";
        assert_eq!(21, problem8_1(file_path_simple));

        let file_path = "testdata/y2022_p8.txt";
        assert_eq!(1736, problem8_1(file_path));
    }

    #[test]
    fn tests_y2022_d8_2() {}
}
