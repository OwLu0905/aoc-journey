use std::{
    collections::HashMap,
    fs::File,
    io::{BufRead, BufReader},
};

trait Walk {
    fn walk(&mut self, action: &Motions, record: &mut HashMap<[i32; 2], bool>) -> &Self;
}

impl Motions {
    fn get_position(&self) -> [i32; 2] {
        match self.direction.as_str() {
            "U" => [0, 1],
            "R" => [1, 0],
            "D" => [0, -1],
            "L" => [-1, 0],
            _ => [0, 0],
        }
    }
}

#[derive(Debug)]
struct Motions {
    direction: String,
    step: u32,
}

#[derive(Debug)]
struct Rope {
    head: [i32; 2],
    tail: [i32; 2],
}

impl Walk for Rope {
    fn walk(&mut self, action: &Motions, record: &mut HashMap<[i32; 2], bool>) -> &Self {
        for _ in 0..action.step {
            let cal_step = action.get_position();
            self.head[0] += cal_step[0];
            self.head[1] += cal_step[1];

            let x_dist = self.head[0] - self.tail[0];
            let y_dist = self.head[1] - self.tail[1];

            if x_dist == 0 && y_dist.abs() > 1 {
                self.tail[1] += y_dist / 2;
            } else if y_dist == 0 && x_dist.abs() > 1 {
                self.tail[0] += x_dist / 2;
            } else if x_dist.abs() > y_dist.abs() {
                self.tail[0] += x_dist / 2;
                self.tail[1] += y_dist;
            } else if y_dist.abs() > x_dist.abs() {
                self.tail[0] += x_dist;
                self.tail[1] += y_dist / 2;
            }

            let record_arr = [self.tail[0], self.tail[1]];

            if !record.contains_key(&record_arr) {
                record.insert(record_arr, true);
            }
        }
        self
    }
}

pub fn problem9_1(path: &str) -> usize {
    let file = File::open(path).expect("cant read the file");
    let reader = BufReader::new(file);

    let matrix: Vec<Motions> = reader
        .lines()
        .map(|line_result| {
            let split_result = line_result.unwrap();
            let split_result = split_result.split(" ").collect::<Vec<&str>>();

            // BUG: use this directly would face the lifetime problem => unwrap store the data in a
            // temporary value
            // let a = line_result.unwrap().split(" ").collect::<Vec<&str>>();

            let step = split_result[1].parse::<u32>().unwrap();
            Motions {
                direction: String::from(split_result[0]),
                step,
            }
        })
        .collect();

    let mut record_map: HashMap<[i32; 2], bool> = HashMap::new();
    let mut rope = Rope {
        head: [0, 0],
        tail: [0, 0],
    };
    for i in matrix {
        rope.walk(&i, &mut record_map);
    }

    record_map.len()
}
pub fn problem9_2() {}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn tests_y2022_d9_1() {
        let file_path_simple = "testdata/y2022_p9_simple.txt";
        assert_eq!(13, problem9_1(file_path_simple));

        let file_path = "testdata/y2022_p9.txt";
        assert_eq!(5858, problem9_1(file_path));
    }

    // #[test]
    // fn tests_y2022_d9_2() {}
}
