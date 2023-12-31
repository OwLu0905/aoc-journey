use std::{collections::HashMap, fs::File, io::Read};

const CONTINUE_NUMBER_1: i32 = 4;
pub fn problem6_1(puzzle_path: &str) -> i32 {
    // NOTE: read file
    let mut puzzle = String::new();
    let mut file = File::open(puzzle_path).expect("cant open the file");
    let _ = file.read_to_string(&mut puzzle);
    puzzle = puzzle
        .trim_end_matches(|c| c == '\n' || c == '\r')
        .to_string();

    // NOTE: check data
    // let mut check_is_duplicate: Vec<bool> = vec![false; puzzle.chars().count()];
    let mut check_is_duplicate: Vec<usize> = vec![0; puzzle.chars().count()];
    let mut check_record: Vec<Option<usize>> = vec![None; 26];

    for (i, val) in puzzle.chars().enumerate() {
        let char_to_idx = val as usize - 'a' as usize;
        if let Some(prev_index) = check_record[char_to_idx] {
            check_is_duplicate[prev_index] = i - prev_index;
        }
        check_record[char_to_idx] = Some(i);
    }

    // NOTE: count
    let mut count: i32 = 0;
    let mut result_index: i32 = 0;
    for (i, &val) in check_is_duplicate.iter().enumerate() {
        if (val as i32) > (CONTINUE_NUMBER_1 - 1 - count) {
            count += 1
        } else if val == 0 {
            count += 1
        } else {
            count = 0;
        }

        if count == CONTINUE_NUMBER_1 {
            result_index = i as i32;
            break;
        }
    }
    result_index + 1
}

pub fn problem6_1_another(puzzle_path: &str) -> i32 {
    let mut file = File::open(puzzle_path).expect("can't read file");
    let mut puzzle = String::new();
    let _ = file.read_to_string(&mut puzzle);
    puzzle = puzzle
        .trim_end_matches(|c| c == '\n' || c == '\r')
        .to_string();

    let mut count: i32 = 0;
    let mut point: i32 = 0;

    let mut count_map: HashMap<char, i32> = HashMap::new();

    let puzzle_len = puzzle.chars().count() as i32;
    let puzzle_char = puzzle.chars().collect::<Vec<char>>();

    let mut ans = 0;
    let _ = loop {
        for i in point..puzzle_len {
            let current_char = puzzle_char[i as usize];

            match count_map.get(&current_char) {
                Some(value) => {
                    count = 1;
                    point = *value + 1;
                    count_map.clear();
                    break;
                }
                None => {
                    count_map.insert(current_char, i);
                    count += 1;
                }
            };
            if count == 5 {
                ans = i;
                break;
            }
        }

        if count == 5 {
            break;
        }
    };
    ans + 1
}

const CONTINUE_NUMBER_2: i32 = 14;
pub fn problem6_2(puzzle_path: &str) -> i32 {
    // NOTE: read file
    let mut puzzle = String::new();
    let mut file = File::open(puzzle_path).expect("cant open the file");
    let _ = file.read_to_string(&mut puzzle);
    puzzle = puzzle
        .trim_end_matches(|c| c == '\n' || c == '\r')
        .to_string();

    // NOTE: check data
    // let mut check_is_duplicate: Vec<bool> = vec![false; puzzle.chars().count()];
    let mut check_is_duplicate: Vec<usize> = vec![0; puzzle.chars().count()];
    let mut check_record: Vec<Option<usize>> = vec![None; 26];

    for (i, val) in puzzle.chars().enumerate() {
        let char_to_idx = val as usize - 'a' as usize;
        if let Some(prev_index) = check_record[char_to_idx] {
            check_is_duplicate[prev_index] = i - prev_index;
        }
        check_record[char_to_idx] = Some(i);
    }

    // NOTE: count
    let mut count: i32 = 0;
    let mut result_index: i32 = 0;
    for (i, &val) in check_is_duplicate.iter().enumerate() {
        if (val as i32) > (CONTINUE_NUMBER_2 - 1 - count) {
            count += 1
        } else if val == 0 {
            count += 1
        } else {
            count = 0;
        }

        if count == CONTINUE_NUMBER_2 {
            result_index = i as i32;
            break;
        }
    }
    result_index + 1
}

pub fn problem6_2_another(puzzle_path: &str) -> i32 {
    let mut file = File::open(puzzle_path).expect("can't read file");
    let mut puzzle = String::new();
    let _ = file.read_to_string(&mut puzzle);
    puzzle = puzzle
        .trim_end_matches(|c| c == '\n' || c == '\r')
        .to_string();

    let mut count: i32 = 0;
    let mut point: i32 = 0;

    let mut count_map: HashMap<char, i32> = HashMap::new();

    let puzzle_len = puzzle.chars().count() as i32;
    let puzzle_char = puzzle.chars().collect::<Vec<char>>();

    let mut ans = 0;
    let _ = loop {
        for i in point..puzzle_len {
            let current_char = puzzle_char[i as usize];

            match count_map.get(&current_char) {
                Some(value) => {
                    count = 1;
                    point = *value + 1;
                    count_map.clear();
                    break;
                }
                None => {
                    count_map.insert(current_char, i);
                    count += 1;
                }
            };
            if count == 15 {
                ans = i;
                break;
            }
        }

        if count == 15 {
            break;
        }
    };
    ans + 1
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn tests_y2022_d6_1() {
        let puzzle_simple = "testdata/y2022_p6_simple.txt";
        assert_eq!(problem6_1_another(puzzle_simple), 10);

        let puzzle_simple = "testdata/y2022_p6_simple.txt";
        assert_eq!(problem6_1(puzzle_simple), 10);

        let puzzle_simple = "testdata/y2022_p6.txt";
        assert_eq!(problem6_1_another(puzzle_simple), 1262);

        let puzzle_simple = "testdata/y2022_p6.txt";
        assert_eq!(problem6_1(puzzle_simple), 1262);
    }

    #[test]
    fn tests_y2022_d6_2() {
        //  case 1 : mjqjpqmgbljsphdztnvjfqwrcgsmlb => 19
        //  case 2 : dqqqqqqqqaabcadaaaaa => 15

        let puzzle_path = "testdata/y2022_p6.txt";
        assert_eq!(problem6_2(puzzle_path), 3444);
        assert_eq!(problem6_2_another(puzzle_path), 3444);
    }
}
