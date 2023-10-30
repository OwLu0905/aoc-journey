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

pub fn problem3_2() {}

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
}
