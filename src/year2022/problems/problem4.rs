use std::{fs::File, io::Read};

static SPLIT_COL: &str = if cfg!(target_os = "windows") {
    "\r\n"
} else {
    "\n"
};

pub fn problem4_1(path: &str) -> i32 {
    let mut file = File::open(path).expect("can't read file");
    let mut content = String::new();
    let _ = file.read_to_string(&mut content);

    let section_list = content.split_terminator(SPLIT_COL).collect::<Vec<&str>>();
    let pair_list = section_list
        .iter()
        .map(|x| check_overlap(x))
        .collect::<Vec<i32>>();

    let count_pair: i32 = pair_list.iter().sum();
    count_pair
}

pub fn check_overlap(str_list: &str) -> i32 {
    let split_section = str_list.split_terminator(",").collect::<Vec<&str>>();
    let first = split_section[0]
        .split_terminator("-")
        .collect::<Vec<&str>>()
        .iter()
        .map(|x| x.parse::<i32>().unwrap_or_default())
        .collect::<Vec<i32>>();

    let second = split_section[1]
        .split_terminator("-")
        .collect::<Vec<&str>>()
        .iter()
        .map(|x| x.parse::<i32>().unwrap_or_default())
        .collect::<Vec<i32>>();

    let min_dis = first[0] - second[0];
    let max_dis = first[1] - second[1];
    let count = if min_dis * max_dis <= 0 { 1 } else { 0 };

    // BUG: miss the case which first[0] is equal to second[0] and first[1] is less than second[11]
    // let count = match first[0] <= second[0] {
    //     true => {
    //         if first[1] >= second[1] {
    //             1
    //         } else {
    //             0
    //         }
    //     }
    //     false => {
    //         if second[1] >= first[1] {
    //             1
    //         } else {
    //             0
    //         }
    //     }
    // };

    count
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn tests_y2022_d4_1() {
        let file_path_simple = "testdata/y2022_p4_simple.txt";
        assert_eq!(2, problem4_1(file_path_simple));

        let file_path = "testdata/y2022_p4.txt";
        assert_eq!(524, problem4_1(file_path));
    }
}
