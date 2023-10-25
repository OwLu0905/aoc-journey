use std::fs::File;
use std::io::prelude::*;

const SPLIT_ELF: &str = "\r\n\r\n";
const SPLIT_COL: &str = "\r\n";

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

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn tests_y2022_p1() {
        let file_path = "testdata/y2022-p1.txt";
        let max_calorie = solve_problem1(file_path);
        dbg!(max_calorie);
    }
}
