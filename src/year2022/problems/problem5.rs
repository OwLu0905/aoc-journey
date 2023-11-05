use std::{fs::File, io::Read};

static SPLIT_RULE: &str = if cfg!(target_os = "windows") {
    "\r\n\r\n"
} else {
    "\n\n"
};

static SPLIT_COL: &str = if cfg!(target_os = "windows") {
    "\r\n"
} else {
    "\n"
};

pub fn problem5_1(path: &str) -> String {
    let mut file = File::open(path).expect("can't read file");
    let mut content = String::new();
    let _ = file.read_to_string(&mut content);
    let section_list = content.split_terminator(SPLIT_RULE).collect::<Vec<&str>>();
    let stack_list = section_list[0]
        .split_terminator(SPLIT_COL)
        .collect::<Vec<&str>>();
    let rule_list = section_list[1]
        .split_terminator(SPLIT_COL)
        .collect::<Vec<&str>>();

    let stack_length = (stack_list[0].len() + 1) / 4;
    let mut stacks: Vec<Vec<char>> = vec![Vec::new(); stack_length];
    for (j, val) in stack_list.iter().rev().enumerate() {
        if j == 0 {
            continue;
        }
        let chars: Vec<char> = val.chars().collect();
        // NOTE: 2, 6, 10 ..
        for (i, x) in chars.iter().enumerate() {
            if i > 0 {
                let idx = (i + 1 - 2) % 4;
                let num = (i + 1 - 2) / 4;
                if idx == 0 && *x != ' ' {
                    stacks[num].push(x.clone())
                }
            }
        }
    }

    let stack_moves = rule_list
        .iter()
        .map(|x| {
            let parts: Vec<&str> = x.split(|c| c == ' ' || c == ' ').collect();
            let mut result: Vec<i32> = Vec::new();

            for part in parts {
                if let Ok(num) = part.parse::<i32>() {
                    result.push(num);
                }
            }
            result
        })
        .collect::<Vec<Vec<i32>>>();

    for c in stack_moves {
        for _ in 0..c[0] {
            let a = (c[1] - 1) as usize;
            let b = (c[2] - 1) as usize;
            let rg = stacks[a].pop().expect("error pop value from stack");
            stacks[b].push(rg);
        }
    }

    let mut answer = String::new();
    for s in stacks.iter() {
        let pop_value = &s.last().expect("error get last value from vec");
        answer = answer + &pop_value.to_string();
    }

    answer
}

pub fn problem5_2(path: &str) -> String {
    let mut file = File::open(path).expect("can't read file");
    let mut content = String::new();
    let _ = file.read_to_string(&mut content);
    let section_list = content.split_terminator(SPLIT_RULE).collect::<Vec<&str>>();
    let stack_list = section_list[0]
        .split_terminator(SPLIT_COL)
        .collect::<Vec<&str>>();
    let rule_list = section_list[1]
        .split_terminator(SPLIT_COL)
        .collect::<Vec<&str>>();

    let stack_length = (stack_list[0].len() + 1) / 4;
    let mut stacks: Vec<Vec<char>> = vec![Vec::new(); stack_length];
    for (j, val) in stack_list.iter().rev().enumerate() {
        if j == 0 {
            continue;
        }
        let chars: Vec<char> = val.chars().collect();
        // NOTE: 2, 6, 10 ..
        for (i, x) in chars.iter().enumerate() {
            if i > 0 {
                let idx = (i + 1 - 2) % 4;
                let num = (i + 1 - 2) / 4;
                if idx == 0 && *x != ' ' {
                    stacks[num].push(x.clone())
                }
            }
        }
    }

    let stack_moves = rule_list
        .iter()
        .map(|x| {
            let parts: Vec<&str> = x.split(|c| c == ' ' || c == ' ').collect();
            let mut result: Vec<i32> = Vec::new();

            for part in parts {
                if let Ok(num) = part.parse::<i32>() {
                    result.push(num);
                }
            }
            result
        })
        .collect::<Vec<Vec<i32>>>();

    for c in stack_moves {
        let mut keep_order: Vec<char> = Vec::new();
        for _ in 0..c[0] {
            let a = (c[1] - 1) as usize;
            let rg = stacks[a].pop().expect("error pop value from stack");
            keep_order.insert(0, rg);
        }
        let b = (c[2] - 1) as usize;
        let _ = &stacks[b].extend(keep_order);
    }

    let mut answer = String::new();
    for s in stacks.iter() {
        let pop_value = &s.last().expect("error get last value from vec");
        answer = answer + &pop_value.to_string();
    }

    answer.to_string()
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn tests_y2022_d5_1() {
        let file_path_simple = "testdata/y2022_p5_simple.txt";
        assert_eq!("CMZ", problem5_1(file_path_simple));

        let file_path = "testdata/y2022_p5.txt";
        assert_eq!("BZLVHBWQF", problem5_1(file_path));
    }
    #[test]
    fn tests_y2022_d5_2() {
        let file_path_simple = "testdata/y2022_p5_simple.txt";
        assert_eq!("MCD", problem5_2(file_path_simple));

        let file_path = "testdata/y2022_p5.txt";
        assert_eq!("TDGJQTZSL", problem5_2(file_path));
    }
}
