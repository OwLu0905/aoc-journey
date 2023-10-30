use std::{fs::File, io::Read};

static SPLIT_COL: &str = if cfg!(target_os = "windows") {
    "\r\n"
} else {
    "\n"
};

pub fn problem3_1(path: &str) -> i32 {
    let mut file = File::open(path).expect("cant read file");
    let mut content = String::new();
    let _ = file.read_to_string(&mut content);
    let rucksack_list = content.split_terminator(SPLIT_COL).collect::<Vec<&str>>();
    let split_char: Vec<i32> = rucksack_list
        .iter()
        .map(|x| convert_to_int(split_to_find(x)))
        .collect();
    let sum_rucksack: i32 = split_char.iter().sum();
    sum_rucksack
}

pub fn problem3_2(path: &str) -> i32 {
    let mut file = File::open(path).expect("cant read this file");
    let mut content = String::new();
    let _ = file.read_to_string(&mut content);
    let rucksack_list = content.split_terminator(SPLIT_COL).collect::<Vec<&str>>();
    let total = rucksack_list.len() / 3;
    let mut sum = 0;
    for n in 0..total {
        dbg!(n);
        sum += group_to_find(n * 3, &rucksack_list);
    }
    sum
}

fn convert_to_int(item: char) -> i32 {
    match item.is_lowercase() {
        true => item as i32 - 'a' as i32 + 1,
        false => item as i32 - 'A' as i32 + 26 + 1,
    }
}

fn split_to_find(rucksack: &str) -> char {
    let mut alpha_arr: [i32; 52] = [0; 52];
    let rucksack_char = rucksack.chars().collect::<Vec<char>>();
    let rucksack_len = rucksack_char.len();
    let middle = rucksack_len / 2;
    let mut find_char = 'a';
    for n in 0..middle {
        let left = convert_to_int(rucksack_char[n]) as usize - 1;
        match alpha_arr[left] {
            0 => {
                alpha_arr[left] = alpha_arr[left] + 1;
            }
            _ => {}
        };
    }

    for n in middle..(rucksack_len) {
        let right = convert_to_int(rucksack_char[n]) as usize - 1;
        match alpha_arr[right] {
            1 => {
                find_char = rucksack_char[n];
            }
            _ => {}
        };
    }
    find_char
}

fn group_to_find(current: usize, data: &Vec<&str>) -> i32 {
    let mut alpha_arr: [i32; 52] = [0; 52];
    // NOTE: first
    let char_lsit = data[current].chars();
    for c in char_lsit {
        alpha_arr[convert_to_int(c) as usize - 1] = alpha_arr[convert_to_int(c) as usize - 1] + 1;
    }
    // NOTE: second
    let mut alpha_arr_sec: [i32; 52] = [0; 52];
    let char_lsit = data[current + 1].chars();
    for c in char_lsit {
        match alpha_arr[convert_to_int(c) as usize - 1] {
            0 => {}
            _ => {
                alpha_arr_sec[convert_to_int(c) as usize - 1] =
                    alpha_arr[convert_to_int(c) as usize - 1] + 1;
            }
        }
    }
    // NOTE: third => find Group Item
    let char_lsit = data[current + 2].chars();
    let mut group_item = 0;
    let mut large_index = 0;
    for c in char_lsit {
        match alpha_arr_sec[convert_to_int(c) as usize - 1] {
            0 => {}
            _ => {
                alpha_arr_sec[convert_to_int(c) as usize - 1] =
                    alpha_arr_sec[convert_to_int(c) as usize - 1] + 1;
                if group_item < alpha_arr_sec[convert_to_int(c) as usize - 1] {
                    group_item = alpha_arr_sec[convert_to_int(c) as usize - 1];
                    large_index = convert_to_int(c);
                }
            }
        }
    }
    dbg!(large_index);
    large_index
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn tests_y2022_d3_1() {
        let file_path_simple = "testdata/y2022_p3_simple.txt";
        assert_eq!(split_to_find("vJrwpWtwJgWrhcsFMMfFFhFp"), 'p');
        assert_eq!(split_to_find("jqHRNqRjqzjGDLGLrsFMfFZSrLrFZsSL"), 'L');
        assert_eq!(split_to_find("PmmdzqPrVvPwwTWBwg"), 'P');
        assert_eq!(split_to_find("wMqvLMZHhHMvwLHjbvcjnnSBnvTQFn"), 'v');
        assert_eq!(split_to_find("ttgJtRGJQctTZtZT"), 't');
        assert_eq!(split_to_find("CrZsJsPPZsGzwwsLwLmpwMDw"), 's');
        assert_eq!(157, problem3_1(file_path_simple));
        let file_path = "testdata/y2022_p3.txt";
        assert_eq!(8233, problem3_1(file_path));
    }
    #[test]
    fn tests_y2022_d3_2() {
        let file_path_simple = "testdata/y2022_p3_simple.txt";
        assert_eq!(70, problem3_2(file_path_simple));
        let file_path = "testdata/y2022_p3.txt";
        assert_eq!(2821, problem3_2(file_path));
    }
}
