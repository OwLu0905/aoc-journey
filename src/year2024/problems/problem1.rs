use std::{
    fs::File,
    io::{BufRead, BufReader},
};

pub fn problem1_1(path: &str) -> i32 {
    let file = File::open(path).expect("cant open files");
    let reader = BufReader::new(file);

    let mut index = 0;
    let mut array_left = vec![];
    let mut array_right = vec![];

    for line in reader.lines() {
        index += 1;
        let line_str = line.unwrap();
        let split_value = line_str
            .split("   ")
            .map(|x| {
                let value = x.parse::<i32>().unwrap();
                value
            })
            .collect::<Vec<i32>>();

        priority_queue(
            &mut array_left,
            &mut array_right,
            &split_value[0],
            &split_value[1],
        );
    }

    let mut sum = 0;
    for i in 0..index {
        sum += (array_right[i] - array_left[i]).abs();
    }

    sum
}
pub fn problem1_2() {}

fn add_to_sorted_list(queue: &mut Vec<i32>, check: &i32) {
    let mut pos = 0;
    let mut swap = false;

    for (index, value) in queue.iter().enumerate() {
        if *check < *value {
            pos = index;
            swap = true;
            break;
        }
    }
    if (!swap) {
        pos = queue.len();
    }

    queue.insert(pos, *check);
}
fn priority_queue(queue_left: &mut Vec<i32>, queue_right: &mut Vec<i32>, left: &i32, right: &i32) {
    add_to_sorted_list(queue_left, left);
    add_to_sorted_list(queue_right, right);
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn tests_y2024_d1_1() {
        let file_path_simple = "testdata/y2024_p1_simple.txt";
        assert_eq!(problem1_1(file_path_simple), 11);

        let file_path = "testdata/y2024_p1.txt";
        assert_eq!(problem1_1(file_path), 3246517);
    }

    #[test]
    fn tests_y2024_d1_2() {}
}
