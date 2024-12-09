use std::{
    fs::File,
    io::{BufRead, BufReader},
};

type List = Vec<i128>;

enum Operator {
    Add,
    Multiply,
    Concate,
}

fn calculation(list: &List, test_value: i128) -> bool {
    let multiply = list.iter().fold(1, |acc, &x| acc * x);
    let sum = list.iter().fold(0, |acc, &x| acc + x);

    if multiply < test_value && sum > test_value {
        return false;
    }
    let result_add = next(list, 1, Operator::Add, list[0], test_value);

    let result_mul = next(list, 1, Operator::Multiply, list[0], test_value);
    result_add || result_mul
}

fn next(list: &List, index: usize, operators: Operator, acc: i128, test_value: i128) -> bool {
    if index == list.len() {
        if acc == test_value {
            return true;
        }
        return false;
    }

    let mut next_acc = acc;
    match operators {
        Operator::Add => next_acc += list[index],
        Operator::Multiply => next_acc = next_acc * list[index],
        _ => {}
    }

    let result_add = next(list, index + 1, Operator::Add, next_acc, test_value);

    let result_mul = next(list, index + 1, Operator::Multiply, next_acc, test_value);

    result_add || result_mul
}

fn calculation_concate(list: &List, test_value: i128) -> bool {
    // let multiply = list.iter().fold(1, |acc, &x| acc * x);
    // let sum = list.iter().fold(0, |acc, &x| acc + x);
    // // let concate = list.iter().fold(0, |acc, &x| {
    // //     let pow: u32 = (x as u32 / 10 as u32) + 1;
    // //     acc * 10_i128.pow(pow) + x
    // // });

    let result_add = next_concate(list, 1, Operator::Add, list[0], test_value);

    let result_mul = next_concate(list, 1, Operator::Multiply, list[0], test_value);

    let result_concate = next_concate(list, 1, Operator::Concate, list[0], test_value);
    result_add || result_mul || result_concate
}

fn next_concate(
    list: &List,
    index: usize,
    operators: Operator,
    acc: i128,
    test_value: i128,
) -> bool {
    if index == list.len() {
        if acc == test_value {
            return true;
        }
        return false;
    }

    if acc > test_value {
        return false;
    }

    let mut next_acc = acc;

    match operators {
        Operator::Add => next_acc += list[index],
        Operator::Multiply => next_acc = next_acc * list[index],
        Operator::Concate => {
            let digits: u32 = (list[index] as f64).log10().floor() as u32 + 1;
            next_acc = 10_i128.pow(digits) * next_acc + list[index];
        }
    }

    let result_add = next_concate(list, index + 1, Operator::Add, next_acc, test_value);

    let result_mul = next_concate(list, index + 1, Operator::Multiply, next_acc, test_value);

    let result_concate = next_concate(list, index + 1, Operator::Concate, next_acc, test_value);
    result_add || result_mul || result_concate
}

pub fn problem7_1(path: &str) -> i128 {
    let file = File::open(path).expect("Open file fialed");

    let reader = BufReader::new(file);

    let mut sum = 0;

    for line in reader.lines() {
        let line_result = line.unwrap();
        let line_split: Vec<_> = line_result.split(": ").collect();

        let test_value = line_split[0].parse::<i128>().unwrap_or(0);
        let test_list: Vec<_> = line_split[1]
            .split_terminator(" ")
            .map(|t| t.parse::<i128>().unwrap_or(0))
            .collect();

        let check = calculation(&test_list, test_value);
        if check {
            sum += test_value
        }
    }
    sum
}

pub fn problem7_2(path: &str) -> i128 {
    let file = File::open(path).expect("Open file fialed");

    let reader = BufReader::new(file);

    let mut sum = 0;

    for line in reader.lines() {
        let line_result = line.unwrap();
        let line_split: Vec<_> = line_result.split(": ").collect();

        let test_value = line_split[0].parse::<i128>().unwrap_or(0);
        let test_list: Vec<_> = line_split[1]
            .split_terminator(" ")
            .map(|t| t.parse::<i128>().unwrap_or(0))
            .collect();

        let check = calculation_concate(&test_list, test_value);
        if check {
            sum += test_value
        }
    }
    sum
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn tests_y2024_d7_1() {
        let file_path = "testdata/y2024_p7_simple.txt";
        assert_eq!(problem7_1(file_path), 3749);

        // 5511276766591
        let file_path = "testdata/y2024_p7.txt";
        assert_eq!(problem7_1(file_path), 5512534574980);
    }

    #[test]
    fn tests_y2024_d7_2() {
        let file_path = "testdata/y2024_p7_simple.txt";
        assert_eq!(problem7_2(file_path), 11387);

        // 5511276766591
        let file_path = "testdata/y2024_p7.txt";
        assert_eq!(problem7_2(file_path), 328790210468594);
    }
}
