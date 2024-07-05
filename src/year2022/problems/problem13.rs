use std::io::prelude::*;
use std::{
    collections::{HashMap, VecDeque},
    fs::File,
};

static SPLIT_COL: &str = if cfg!(target_os = "windows") {
    "\r\n"
} else {
    "\n"
};

#[derive(Clone)]
pub struct Puzzle(String);

#[derive(Debug, PartialEq)]
pub enum PuzzleState {
    Failure,
    Continue,
    Success,
}

impl Puzzle {
    pub fn new(p: String) -> Self {
        Puzzle(p)
    }

    pub fn split(&self) -> Vec<String> {
        let mut layer_count = 0;
        let mut layer: HashMap<usize, Vec<usize>> = HashMap::new();

        let mut check_item = VecDeque::new();
        let max_len = self.0.chars().count();

        for (i, s) in self.0.chars().enumerate() {
            if i == 0 || i == max_len - 1 {
                continue;
            }
            if s == '[' {
                if check_item.len() == 0 {
                    let next_layer = layer.entry(layer_count).or_default();
                    next_layer.push(i);
                }

                check_item.push_front('[');
                continue;
            }
            if s == ']' {
                check_item.pop_front();
                if check_item.len() == 0 {
                    let current_layer = layer.entry(layer_count).or_default();
                    current_layer.push(i);
                }
                continue;
            }

            if check_item.len() == 0 && s == ',' {
                layer_count += 1;
                continue;
            }
            if s != ',' && s != '[' && s != ']' {
                if check_item.len() == 0 {
                    let current_layer = layer.entry(layer_count).or_default();
                    current_layer.push(i);
                }
                continue;
            }
        }

        let mut to_split_layer = vec![String::new(); layer.len()];

        for (key, value) in &layer {
            let s = *value.first().unwrap();
            let e = value.last().unwrap().checked_add(1).unwrap();

            let element = self.0[s..e].to_owned();

            to_split_layer[*key] = element;
        }
        to_split_layer
    }
}

fn compare(left: &String, right: &String) -> PuzzleState {
    let puzzle_left = Puzzle::new(left.to_string()).split();
    let puzzle_right = Puzzle::new(right.to_string()).split();

    let mut check = PuzzleState::Continue;

    for (idx, value) in puzzle_left.iter().enumerate() {
        if puzzle_right.get(idx).is_none() {
            return PuzzleState::Failure;
        }

        match (value.parse::<i32>(), puzzle_right[idx].parse::<i32>()) {
            (Ok(left), Ok(right)) => {
                let gap = left - right;
                match gap {
                    n if n > 0 => {
                        check = PuzzleState::Failure;
                    }
                    n if n == 0 => {
                        check = PuzzleState::Continue;
                    }
                    n if n < 0 => {
                        check = PuzzleState::Success;
                    }
                    _ => {
                        println!("invalid comparison");
                    }
                }
            }
            (Err(_), Err(_)) => {
                check = compare(value, &puzzle_right[idx]);
            }
            (Err(_), Ok(right)) => {
                let right = format!("[{}]", right);
                check = compare(value, &right);
            }

            (Ok(left), Err(_)) => {
                let left = format!("[{}]", left);
                check = compare(&left, &puzzle_right[idx]);
            }
        };

        if check == PuzzleState::Failure || check == PuzzleState::Success {
            break;
        }
    }

    // NOTE: [], [[], 3, 4]  => PuzzleState::Success
    // NOTE: [], [] => PuzzleState::Continue
    // NOTE: [6], [6,3,4]  => PuzzleState::Success
    if check == PuzzleState::Continue && puzzle_left.len() < puzzle_right.len() {
        check = PuzzleState::Success;
    }
    check
}

pub fn insert_sort(list: &mut Vec<String>) {
    for i in 1..list.len() {
        let mut j = i;

        while j > 0 && compare(&list[j - 1], &list[j]) == PuzzleState::Failure {
            list.swap(j - 1, j);
            j -= 1;
        }
    }
}

pub fn problem13_1(path: &str) -> usize {
    let mut file = File::open(path)
        .expect("can't read the file, please check whether the file is exist or not");
    let mut data_content = String::new();

    file.read_to_string(&mut data_content)
        .expect("cant parse data to String");

    let mut left_collect: Vec<String> = Vec::new();
    let mut right_collect: Vec<String> = Vec::new();

    let split_dat = data_content.split(SPLIT_COL);
    let mut total_len = 0;
    for (idx, sp) in split_dat.enumerate() {
        if sp.is_empty() {
            continue;
        }
        let dir = idx % 3;

        if dir == 0 {
            total_len += 1;
            left_collect.push(sp.to_string());
        } else {
            right_collect.push(sp.to_string());
        }
    }

    let mut count = 0;

    for i in 0..total_len {
        let left = &left_collect[i];
        let right = &right_collect[i];
        let check = compare(left, right);
        if check != PuzzleState::Failure {
            count += 1;
            println!("{}", i + 1);
            count += i;
        }
    }

    count
}

pub fn problem13_2(path: &str) -> usize {
    let mut file = File::open(path)
        .expect("can't read the file, please check whether the file is exist or not");
    let mut data_content = String::new();

    file.read_to_string(&mut data_content)
        .expect("cant parse data to String");

    let mut list: Vec<String> = Vec::new();

    let split_dat = data_content.split(SPLIT_COL);

    for sp in split_dat {
        if sp.is_empty() {
            continue;
        }
        list.push(sp.to_string());
    }

    list.push("[[2]]".to_string());
    list.push("[[6]]".to_string());

    insert_sort(&mut list);

    let divider_1 = &list.iter().position(|x| x == "[[2]]").unwrap() + 1;
    let divider_2 = &list.iter().position(|x| x == "[[6]]").unwrap() + 1;

    divider_1 * divider_2
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn tests_y2022_d13_1() {
        let file_path = "testdata/y2022_p13_simple.txt";
        let value = problem13_1(file_path);
        assert_eq!(value, 13);

        let file_path = "testdata/y2022_p13.txt";
        let value = problem13_1(file_path);
        assert_eq!(value, 5682);
    }

    #[test]
    fn check_compare() {
        let left = "[6]".to_string();
        let right = "[6,3,4]".to_string();

        let check = compare(&left, &right);
        assert_eq!(check, PuzzleState::Success);

        // BUG: The case I missout
        let left =
            "[[6],[[8,[9],[3,6,0,8,6],7],9,4],[[],[[10,0],[9,3,8,10,1],[10,4]]]]".to_string();
        let right =
            "[[6,[2,[0,0,0,5,5],[7,1,2,9],7],[],0,[8]],[],[6,9,6],[8,4,[[7,3,3],[3],6],0,0],[]]"
                .to_string();

        let check = compare(&left, &right);
        assert_eq!(check, PuzzleState::Success);

        // BUG: The case I missout
        let left = "[[],[1,10,5,[4,[0,9,1,1]],[[9,3,6,1],10,4,3,5]],[10,8,[5]],[1,[6]],[1,[[5,9],[],5,[]]]]".to_string();
        let right = "[[]]".to_string();

        let check = compare(&left, &right);
        assert_eq!(check, PuzzleState::Failure);
    }

    #[test]
    fn tests_y2022_d13_2() {
        let file_path = "testdata/y2022_p13_simple.txt";
        let value = problem13_2(file_path);
        assert_eq!(value, 140);

        let file_path = "testdata/y2022_p13.txt";
        let value = problem13_2(file_path);
        assert_eq!(value, 20304);
    }
}
