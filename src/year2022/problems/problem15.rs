use std::{
    collections::HashMap,
    fs::File,
    io::{BufRead, BufReader},
};

fn manhat_dist(list: &[i32]) -> i32 {
    i32::abs(list[0] - list[2]) + i32::abs(list[1] - list[3])
}

fn merge(mut list: Vec<Vec<i32>>) -> i32 {
    list.sort_unstable_by(|a, b| a[0].cmp(&b[0]));

    let mut merge_list = Vec::new();
    let total_len = list.len();

    let mut min = list[0][0];
    let mut max = list[0][1];

    for i in 0..(total_len - 1) {
        if max >= (list[i + 1][0] - 1) {
            if max < list[i + 1][1] {
                max = list[i + 1][1]
            }
        } else {
            merge_list.push(vec![min, max]);
            min = list[i + 1][0];
            max = list[i + 1][1];
        }
    }
    merge_list.push(vec![min, max]);

    let sum = merge_list
        .iter()
        .map(|x| {
            let gap = x[1] - x[0];
            gap
        })
        .sum::<i32>();

    sum
}

fn merge_list(mut list: Vec<Vec<i32>>) -> Vec<Vec<i32>> {
    list.sort_unstable_by(|a, b| a[0].cmp(&b[0]));

    let mut merge_list = Vec::new();
    let total_len = list.len();

    let mut min = list[0][0];
    let mut max = list[0][1];

    for i in 0..(total_len - 1) {
        if max >= (list[i + 1][0] - 1) {
            if max < list[i + 1][1] {
                max = list[i + 1][1]
            }
        } else {
            merge_list.push(vec![min, max]);
            min = list[i + 1][0];
            max = list[i + 1][1];
        }
    }
    merge_list.push(vec![min, max]);
    merge_list
}

pub fn problem15_1(path: &str, target_row: i32) -> i32 {
    let file = File::open(path).expect("cant open files");
    let reader = BufReader::new(file);

    let mut target_map = HashMap::new();

    let mut check_interval_list = Vec::new();

    let mut sum = 0;
    for line in reader.lines() {
        let line_str = line.unwrap();

        let split_line = line_str
            .split(" ")
            .filter(|x| x.contains("="))
            .map(|x| {
                let x = x.trim_end_matches(&[',', ':'][..]);
                let split_x = x.split("=").collect::<Vec<&str>>();
                let value = split_x[1].parse::<i32>().unwrap();
                value
            })
            .collect::<Vec<i32>>();
        let dist = manhat_dist(&split_line);

        if split_line[3] == target_row {
            target_map.insert(split_line[3], false);
        }

        let sensor_x = split_line[0];
        let sensor_y = split_line[1];

        // STEP0: check  target_row in range
        if (target_row > sensor_y + dist) || (target_row < sensor_y - dist) {
            continue;
        }

        // STEP1: check upper or lower
        let right_bound = sensor_x + (dist - i32::abs(target_row - sensor_y));
        let left_bound = sensor_x - (dist - i32::abs(target_row - sensor_y));

        sum = sum + (right_bound - left_bound);
        check_interval_list.push(vec![left_bound, right_bound]);

        // NOTE: performance (add to hashmap) :  4.98s
        // for i in left_bound..(right_bound + 1) {
        //     if target_map.get(&i).is_none() {
        //         target_map.insert(i, true);
        //     }
        // }
    }
    let sum = merge(check_interval_list);

    sum
}
pub fn problem15_2(path: &str) -> i128 {
    let file = File::open(path).expect("cant open files");
    let reader = BufReader::new(file);

    let position_list: Vec<Vec<i32>> = reader
        .lines()
        .map(|x| {
            let x = x.unwrap();
            let pos_list = x
                .split(" ")
                .filter(|x| x.contains("="))
                .map(|x| {
                    let x = x.trim_end_matches(&[',', ':'][..]);
                    let split_x = x.split("=").collect::<Vec<&str>>();
                    let value = split_x[1].parse::<i32>().unwrap();
                    value
                })
                .collect::<Vec<i32>>();
            pos_list
        })
        .collect();
    let mut signal: i128 = 0;

    for i in 0..4000001 {
        let mut check_interval_list = Vec::new();
        for split_line in &position_list {
            let dist = manhat_dist(&split_line);

            let sensor_x = split_line[0];
            let sensor_y = split_line[1];

            // STEP0: check  target_row in range
            if (i > sensor_y + dist) || (i < sensor_y - dist) {
                continue;
            }

            // STEP1: check upper or lower
            let right_bound = sensor_x + (dist - i32::abs(i - sensor_y));
            let left_bound = sensor_x - (dist - i32::abs(i - sensor_y));

            check_interval_list.push(vec![left_bound, right_bound]);
        }
        let sum_list = merge_list(check_interval_list);
        if sum_list.len() > 1 {
            signal = (i as i128 + 4000000 * (sum_list[0][1] as i128 + 1 as i128)) as i128;
            break;
        }
    }
    signal
}

#[cfg(test)]
mod tests {

    use super::*;

    #[test]
    fn tests_y2022_d15_1() {
        let file_path_simple = "testdata/y2022_p15_simple.txt";
        assert_eq!(problem15_1(file_path_simple, 10), 26);

        let file_path = "testdata/y2022_p15.txt";
        assert_eq!(problem15_1(file_path, 2000000), 5716881);
    }

    #[test]
    fn tests_y2022_d15_2() {
        let file_path_simple = "testdata/y2022_p15_simple.txt";
        assert_eq!(problem15_2(file_path_simple), 56000011);

        let file_path = "testdata/y2022_p15.txt";
        assert_eq!(problem15_2(file_path), 10852583132904)
    }
}
