use std::{collections::HashMap, fs::File, io::Read};

pub fn problem6_1(puzzle_path: &str) -> i32 {
    // NOTE: read file
    let mut puzzle = String::new();
    let mut file = File::open(puzzle_path).expect("cant open the file");
    let _ = file.read_to_string(&mut puzzle);
    puzzle = puzzle
        .trim_end_matches(|c| c == '\n' || c == '\r')
        .to_string();

    // NOTE: check data
    let mut check_is_duplicate: Vec<bool> = vec![false; puzzle.chars().count()];

    // TODO: use vec to minimize memory size
    let mut check_record: HashMap<char, usize> = HashMap::new();

    for (i, val) in puzzle.chars().enumerate() {
        match check_record.get(&val) {
            Some(&prev_index) => {
                if (i - prev_index) >= 4 {
                    check_is_duplicate[prev_index] = false;
                } else {
                    check_is_duplicate[prev_index] = true;
                }
            }
            None => {
                check_record.insert(val, i);
            }
        }

        if let Some(v) = check_record.get_mut(&val) {
            *v = i
        }
    }

    // NOTE: count
    let mut count = 0;
    let mut result_index: i32 = 0;
    for (i, &val) in check_is_duplicate.iter().enumerate() {
        match val == false {
            true => {
                count += 1;
            }
            false => {
                count = 0;
            }
        }
        if count == 4 {
            result_index = i as i32;
            break;
        }
    }
    result_index + 1
}
pub fn problem6_2() {}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn tests_y2022_d6_1() {
        let puzzle_simple = "testdata/y2022_p6_simple.txt";
        assert_eq!(problem6_1(puzzle_simple), 10);

        let puzzle_simple = "testdata/y2022_p6.txt";
        assert_eq!(problem6_1(puzzle_simple), 1262);
    }

    #[test]
    fn tests_y2022_d6_2() {}
}
