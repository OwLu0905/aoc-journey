use std::fs::File;
use std::io::prelude::*;

// NOTE: windows
// const SPLIT_ELF: &str = "\r\n\r\n";
// const SPLIT_COL: &str = "\r\n";

// NOTE: macos
// const SPLIT_ELF: &str = "\n\n";
// const SPLIT_COL: &str = "\n";

static SPLIT_ELF: &str = if cfg!(target_os = "windows") {
    "\r\n\r\n"
} else {
    "\n\n"
};

static SPLIT_COL: &str = if cfg!(target_os = "windows") {
    "\r\n"
} else {
    "\n"
};

#[derive(Debug)]
struct MaxCal {
    elf: i32,
    cal: i32,
}

pub fn solve_problem1(path: &str) -> i32 {
    let mut file = File::open(path)
        .expect("can't read the file, please check whether the file is exist or not");
    let mut data_content = String::new();

    file.read_to_string(&mut data_content)
        .expect("cant parse data to String");

    dbg!(&data_content);
    // TODO: collect to Vec
    let mut clear_data: Vec<Vec<&str>> = Vec::new();

    let split_dat = data_content.split(SPLIT_ELF);
    for sp in split_dat {
        clear_data.push(sp.split(SPLIT_COL).collect());
    }

    let mut max_elf_info = MaxCal { elf: 0, cal: 0 };

    for (i, el) in clear_data.iter().enumerate() {
        let a: i32 = el.iter().map(|x| x.parse().unwrap_or(0)).sum();
        match a > max_elf_info.cal {
            true => {
                max_elf_info.cal = a;
                max_elf_info.elf = i as i32;
            }
            _ => {}
        }
    }
    dbg!(&max_elf_info);
    max_elf_info.cal
}

pub fn solve_problem2(path: &str) -> i32 {
    let mut file = File::open(path)
        .expect("can't read the file, please check whether the file is exist or not");

    let mut data_content = String::new();
    file.read_to_string(&mut data_content)
        .expect("cant parse data to String");

    let mut top_three_calorie: [i32; 3] = [0, 0, 0];
    let split_elf = data_content.split(SPLIT_ELF).collect::<Vec<&str>>();
    let split_val: Vec<Vec<&str>> = split_elf
        .iter()
        .map(|&i| -> Vec<&str> { i.split(SPLIT_COL).collect() })
        .collect();

    for (_, ele) in split_val.iter().enumerate() {
        let a: i32 = ele.iter().map(|x| x.parse().unwrap_or(0)).sum();
        if a > top_three_calorie[0] {
            top_three_calorie[2] = top_three_calorie[1];
            top_three_calorie[1] = top_three_calorie[0];
            top_three_calorie[0] = a;
        } else if a > top_three_calorie[1] {
            top_three_calorie[2] = top_three_calorie[1];
            top_three_calorie[1] = a;
        } else if a > top_three_calorie[2] {
            top_three_calorie[2] = a;
        }
    }
    let sum = top_three_calorie.iter().sum();
    sum
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn tests_y2022_p1() {
        let file_path = "testdata/y2022_p1.txt";
        let max_calorie = solve_problem1(file_path);
        assert_eq!(max_calorie, 67016);

        let file_path = "testdata/y2022_example.txt";
        let max_calorie = solve_problem2(file_path);
        assert_eq!(max_calorie, 45000);
    }
    #[test]
    fn tests_y2022_p2() {
        let file_path = "testdata/y2022_p1.txt";
        let max_calorie = solve_problem2(file_path);
        assert_eq!(max_calorie, 200116);
    }
}
