use std::{
    collections::{HashMap, HashSet},
    fs::File,
    io::{BufReader, Read},
};

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

pub fn problem5_1(path: &str) -> i32 {
    let file = File::open(path).expect("Open file failed");

    let mut reader = BufReader::new(file);
    let mut contents = String::new();

    reader
        .read_to_string(&mut contents)
        .expect("Failed to read file");

    let split_block: Vec<&str> = contents.split(SPLIT_RULE).collect();

    let mut order_map: HashMap<&str, HashSet<&str>> = HashMap::new();
    let _right_order = split_block[0].split(SPLIT_COL).for_each(|x| {
        let order_check = x.split("|").collect::<Vec<&str>>();
        let key = order_check[0];
        let value = order_check[1];

        order_map
            .entry(key)
            .or_insert_with(HashSet::new)
            .insert(value);
    });

    let update_order: Vec<Vec<&str>> = split_block[1]
        .split_terminator(SPLIT_COL)
        .map(|x| x.split(",").collect())
        .collect();

    let mut check = false;
    let mut sum = 0;
    for i in update_order.iter() {
        for j in 1..i.len() {
            if let Some(v) = order_map.get(i[j - 1]) {
                if v.contains(i[j]) {
                    check = true;
                }
            } else {
                check = true
            }

            if let Some(v) = order_map.get(i[j]) {
                if v.contains(i[j - 1]) {
                    check = false;
                    break;
                }
            } else {
                check = true
            }
        }

        if check {
            let middle_value = i[i.len() / 2].parse::<i32>().unwrap();
            sum += middle_value;
        }
    }

    sum
}

pub fn problem5_2(path: &str) -> i32 {
    let file = File::open(path).expect("Open file failed");

    let mut reader = BufReader::new(file);
    let mut contents = String::new();

    reader
        .read_to_string(&mut contents)
        .expect("Failed to read file");

    let split_block: Vec<&str> = contents.split(SPLIT_RULE).collect();

    let mut order_map: HashMap<&str, HashSet<&str>> = HashMap::new();
    let _right_order = split_block[0].split(SPLIT_COL).for_each(|x| {
        let order_check = x.split("|").collect::<Vec<&str>>();
        let key = order_check[0];
        let value = order_check[1];

        order_map
            .entry(key)
            .or_insert_with(HashSet::new)
            .insert(value);
    });

    let mut update_order: Vec<Vec<&str>> = split_block[1]
        .split_terminator(SPLIT_COL)
        .map(|x| x.split(",").collect())
        .collect();

    let mut sum = 0;
    for row in update_order.iter_mut() {
        let mut c = 0;
        let mut fail = false;
        while c < row.len() {
            let mut s = c;
            let mut p = s + 1;
            let mut swap = false;
            while p < row.len() {
                if let Some(v) = order_map.get(row[p]) {
                    if v.contains(row[s]) {
                        // NOTE: need to swap
                        row.swap(s, p);
                        s = p;
                        p = s + 1;
                        swap = true;
                        fail = true;
                    } else {
                        p += 1;
                    }
                } else {
                    p += 1;
                }
            }
            if !swap {
                c += 1;
            }
        }
        if fail {
            let middle_value = row[row.len() / 2].parse::<i32>().unwrap();
            sum += middle_value;
        }
    }

    sum
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn tests_y2024_d5_1() {
        let file_path = "testdata/y2024_p5_simple.txt";
        assert_eq!(problem5_1(file_path), 143);

        let file_path = "testdata/y2024_p5.txt";
        assert_eq!(problem5_1(file_path), 4872);
    }

    #[test]
    fn tests_y2024_d5_2() {
        let file_path = "testdata/y2024_p5_simple.txt";
        assert_eq!(problem5_2(file_path), 123);

        let file_path = "testdata/y2024_p5.txt";
        assert_eq!(problem5_2(file_path), 5564);
    }
}
