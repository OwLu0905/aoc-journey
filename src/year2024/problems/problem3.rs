use regex::Regex;
use std::{
    fs::File,
    io::{BufReader, Read},
};

fn split_undo(contents: &str) -> i32 {
    let split_value: Vec<&str> = contents.split("don't()").collect();
    get_multiply_value(split_value[0])
}

fn split_do(contents: &str) -> i32 {
    let value: Vec<i32> = contents.split("do()").map(|v| split_undo(v)).collect();
    value.iter().sum()
}

fn get_multiply_value(contents: &str) -> i32 {
    let re = Regex::new(r"mul\(([0-9]{1,3}),\s*([0-9]{1,3})\)").unwrap();

    let mut sum = 0;
    for caps in re.captures_iter(contents) {
        let first_num = &caps[1].parse::<i32>().unwrap_or(0);
        let second_num = &caps[2].parse::<i32>().unwrap_or(0);
        sum += first_num * second_num
    }

    sum
}

pub fn problem3_1(path: &str) -> i32 {
    let file = File::open(path).expect("Open file failed");
    let mut reader = BufReader::new(file);

    let mut contents = String::new();

    reader
        .read_to_string(&mut contents)
        .expect("Failed to read file");

    let sum = get_multiply_value(&contents);
    sum
}

pub fn problem3_2(path: &str) -> i32 {
    let file = File::open(path).expect("Open file failed");
    let mut reader = BufReader::new(file);

    let mut contents = String::new();

    reader
        .read_to_string(&mut contents)
        .expect("Failed to read file");

    let value = split_do(&contents);
    value
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn tests_y2024_d3_1() {
        let file_path = "testdata/y2024_p3_simple.txt";
        assert_eq!(problem3_1(file_path), 161);

        let file_path = "testdata/y2024_p3.txt";
        assert_eq!(problem3_1(file_path), 159833790);
    }

    #[test]
    fn tests_y2024_d3_2() {
        let file_path = "testdata/y2024_p3_simple2.txt";
        assert_eq!(problem3_2(file_path), 48);

        let file_path = "testdata/y2024_p3.txt";
        assert_eq!(problem3_2(file_path), 89349241);
    }
}
