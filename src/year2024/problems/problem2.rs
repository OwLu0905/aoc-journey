use std::{
    fs::File,
    io::{BufRead, BufReader},
};

fn valid_loss(value: i32) -> bool {
    value.abs() <= 3 && value.abs() >= 1
}

fn valid_direction(loss: i32, direction: i32) -> bool {
    loss > 0 && direction > 0 || loss < 0 && direction < 0
}

fn examine_quota(list: &mut Vec<i32>) -> i32 {
    let mut valid = true;
    let last = list.last().unwrap();
    let first = list.first().unwrap();

    let gradient = *last - *first;

    let mut error_pos = -1;
    for i in 1..list.len() {
        let loss = list[i] - list[i - 1];

        if !valid_loss(loss) && !valid_direction(loss, gradient) {
            valid = false;
            error_pos = i as i32;
            break;
        }

        if valid_loss(loss) && !valid_direction(loss, gradient) {
            valid = false;
            error_pos = i as i32;
            break;
        }
        if !valid_loss(loss) && valid_direction(loss, gradient) {
            valid = false;
            error_pos = i as i32;
            break;
        }
    }
    if !valid && error_pos != -1 {
        let mut list_1 = list.to_vec();
        list_1.remove(error_pos as usize);
        let check_1 = examine(list_1);

        let mut list_2 = list.to_vec();
        list_2.remove((error_pos - 1) as usize);
        let check_2 = examine(list_2);
        if check_1 == 1 || check_2 == 1 {
            return 1;
        }
    }
    if valid {
        return 1;
    }
    0
}

fn examine(list: Vec<i32>) -> i32 {
    let mut valid = true;
    let last = list.last().unwrap();
    let first = list.first().unwrap();

    let gradient = *last - *first;

    for i in 1..list.len() {
        let loss = list[i] - list[i - 1];
        if !valid_loss(loss) {
            valid = false;
            break;
        }

        if !valid_direction(loss, gradient) {
            valid = false;
            break;
        }
    }
    if valid {
        return 1;
    }
    0
}

pub fn problem2_1(path: &str) -> i32 {
    let file = File::open(path).expect("cant open files");

    let reader = BufReader::new(file);

    let mut sum = 0;
    for line in reader.lines() {
        let line_str = line.unwrap();
        let split_col: Vec<i32> = line_str
            .split(" ")
            .map(|x| x.parse::<i32>().unwrap_or(0))
            .collect();
        let count = examine(split_col);

        sum += count;
    }
    sum
}
pub fn problem2_2(path: &str) -> i32 {
    let file = File::open(path).expect("cant open files");

    let reader = BufReader::new(file);

    let mut sum = 0;
    for line in reader.lines() {
        let line_str = line.unwrap();
        let mut split_col: Vec<i32> = line_str
            .split(" ")
            .map(|x| x.parse::<i32>().unwrap_or(0))
            .collect();
        let count = examine_quota(&mut split_col);

        sum += count;
    }
    sum
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn tests_y2024_d2_1() {
        let file_path_simple = "testdata/y2024_p2_simple.txt";
        assert_eq!(problem2_1(file_path_simple), 2);

        let file_path = "testdata/y2024_p2.txt";
        assert_eq!(problem2_1(file_path), 631);
    }

    #[test]
    fn tests_y2024_d2_2() {
        let file_path_simple = "testdata/y2024_p2_simple.txt";
        assert_eq!(problem2_2(file_path_simple), 4);

        let file_path = "testdata/y2024_p2.txt";
        assert_eq!(problem2_2(file_path), 665);
    }
}
