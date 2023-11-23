use std::{collections::HashMap, fs::File, io::Read};

struct LayerDir {
    dir: String,
}

pub fn problem7_1(path: &str) -> u32 {
    let mut file = File::open(path).expect("Unable to read file");
    let mut content = String::new();
    let _ = file
        .read_to_string(&mut content)
        .expect("Unable to read file content");

    let mut current_dir = LayerDir {
        dir: "root".to_string(),
    };
    let mut check_previous_dir: HashMap<String, String> = HashMap::new();
    let mut check_size: HashMap<String, u32> = HashMap::new();
    check_size.insert("root".to_string(), 0);

    for (idx, i) in content.lines().enumerate() {
        if i.trim().starts_with("$ cd ..") {
            // TODO: 2. $ cd .. => to previouse dir
            let prv = check_previous_dir
                .get(current_dir.dir.as_str())
                .expect("dont have prvious")
                .to_string();

            let mut get_value: u32 = 0;

            if let Some(entry) = check_size.get(prv.as_str()) {
                get_value = entry.clone();
            }

            if let Some(entry) = check_size.get(current_dir.dir.as_str()) {
                get_value += entry.clone();
            }

            if let Some(entry) = check_size.get_mut(prv.as_str()) {
                *entry = get_value;
            }

            current_dir.dir = check_previous_dir
                .get(current_dir.dir.as_str())
                .expect("don't have previous")
                .to_string();
        } else if i.trim().starts_with("$ cd") {
            // TODO: 1. $ cd xx => to next dir
            let current_dir_name = i
                .split_whitespace()
                .nth(2)
                .expect("cant get dir name")
                .to_string()
                + idx.to_string().as_str();
            let c = current_dir_name.clone();
            check_previous_dir.insert(c.clone(), current_dir.dir.to_string());
            current_dir.dir = current_dir_name.to_string();
            check_size.insert(c.clone(), 0);
        } else if i.trim().starts_with("dir") || i.trim().starts_with("& ls") {
            // TODO: 3. $ ls => show all items
            // TODO: 4. dir => show dir name
        } else {
            // TODO: 5. 33xxx file_name => show file name and size
            let current_file_size = i
                .split_whitespace()
                .nth(0)
                .expect("cant get dir name")
                .parse::<u32>()
                .unwrap_or(0);
            if let Some(entry) = check_size.get_mut(current_dir.dir.as_str()) {
                *entry += current_file_size;
            }
        }
    }

    // NOTE: Back to root dir
    loop {
        if current_dir.dir == "root" {
            break;
        }
        let prv = check_previous_dir
            .get(current_dir.dir.as_str())
            .expect("dont have prvious")
            .to_string();

        let mut get_value: u32 = 0;

        if let Some(entry) = check_size.get(prv.as_str()) {
            get_value = entry.clone();
        }

        if let Some(entry) = check_size.get(current_dir.dir.as_str()) {
            get_value += entry.clone();
        }

        if let Some(entry) = check_size.get_mut(prv.as_str()) {
            *entry = get_value;
        }

        current_dir.dir = check_previous_dir
            .get(current_dir.dir.as_str())
            .expect("don't have previous")
            .to_string();
    }

    // TODO: check why use clone before sorting
    let filter_value: Vec<u32> = check_size
        .values()
        .cloned()
        .filter(|&v| v <= 100000)
        .collect();

    let mut sorted_filter_value = filter_value.clone();
    sorted_filter_value.sort();
    sorted_filter_value.iter().sum()
}

pub fn problem7_2(path: &str) -> u32 {
    let mut file = File::open(path).expect("Unable to read file");
    let mut content = String::new();
    let _ = file
        .read_to_string(&mut content)
        .expect("Unable to read file content");

    let mut current_dir = LayerDir {
        dir: "root".to_string(),
    };
    let mut check_previous_dir: HashMap<String, String> = HashMap::new();
    let mut check_size: HashMap<String, u32> = HashMap::new();

    check_size.insert("root".to_string(), 0);

    for (idx, i) in content.lines().enumerate() {
        // TODO: check current dir
        if i.trim().starts_with("$ cd ..") {
            let prv = check_previous_dir
                .get(current_dir.dir.as_str())
                .expect("dont have prvious")
                .to_string();

            let mut get_value: u32 = 0;

            if let Some(entry) = check_size.get(prv.as_str()) {
                get_value = entry.clone();
            }

            if let Some(entry) = check_size.get(current_dir.dir.as_str()) {
                get_value += entry.clone();
            }

            if let Some(entry) = check_size.get_mut(prv.as_str()) {
                *entry = get_value;
            }

            current_dir.dir = check_previous_dir
                .get(current_dir.dir.as_str())
                .expect("don't have previous")
                .to_string();
        } else if i.trim().starts_with("$ cd") {
            let current_dir_name = i
                .split_whitespace()
                .nth(2)
                .expect("cant get dir name")
                .to_string()
                + idx.to_string().as_str();
            let c = current_dir_name.clone();
            check_previous_dir.insert(c.clone(), current_dir.dir.to_string());
            current_dir.dir = current_dir_name.to_string();
            check_size.insert(c.clone(), 0);
        } else if i.trim().starts_with("dir") || i.trim().starts_with("& ls") {
        } else {
            let current_file_size = i
                .split_whitespace()
                .nth(0)
                .expect("cant get dir name")
                .parse::<u32>()
                .unwrap_or(0);
            if let Some(entry) = check_size.get_mut(current_dir.dir.as_str()) {
                *entry += current_file_size;
            }
        }
    }

    // NOTE: back to root
    loop {
        if current_dir.dir == "root" {
            break;
        }

        let prv = check_previous_dir
            .get(current_dir.dir.as_str())
            .expect("dont have prvious")
            .to_string();

        let mut get_value: u32 = 0;

        if let Some(entry) = check_size.get(prv.as_str()) {
            get_value = entry.clone();
        }

        if let Some(entry) = check_size.get(current_dir.dir.as_str()) {
            get_value += entry.clone();
        }

        if let Some(entry) = check_size.get_mut(prv.as_str()) {
            *entry = get_value;
        }

        current_dir.dir = check_previous_dir
            .get(current_dir.dir.as_str())
            .expect("don't have previous")
            .to_string();
    }

    check_size.remove("/0");
    let root_size = *check_size
        .get(&"root".to_string())
        .expect("can't get root size");
    let gap = 70000000 - root_size;

    let mut sort_map: Vec<_> = check_size.into_iter().collect();
    sort_map.sort_by(|a, b| a.1.cmp(&b.1));

    let mut min_delete = 0;
    for (_, v) in sort_map.iter().rev() {
        let unused_space = gap + v;
        if unused_space < 30000000 {
            break;
        } else {
            min_delete = *v;
        }
    }

    min_delete
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn tests_y2022_d7_1() {
        let file_path_simple = "testdata/y2022_p7_simple.txt";
        let file_path = "testdata/y2022_p7.txt";
        assert_eq!(95437, problem7_1(file_path_simple));
        assert_eq!(1182909, problem7_1(file_path));
    }

    #[test]
    fn tests_y2022_d7_2() {
        let file_path_simple = "testdata/y2022_p7_simple.txt";
        assert_eq!(24933642, problem7_2(file_path_simple));
        let file_path = "testdata/y2022_p7.txt";
        assert_eq!(2832508, problem7_2(file_path));
    }
}
