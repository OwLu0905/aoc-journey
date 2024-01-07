use std::{
    fs::File,
    io::{BufRead, BufReader},
};

#[derive(Debug)]
enum Cycle {
    One,
    Two(i32),
}

#[derive(Debug)]
struct Signal {
    count: i32,
    cycle: Vec<i32>,
}

impl Signal {
    fn new() -> Self {
        Signal {
            count: 0,
            cycle: vec![1],
        }
    }
    fn program(&mut self, cycle: Cycle) {
        match cycle {
            Cycle::One => {
                self.count += 1;
                let last_vaule = *self.cycle.last().unwrap();
                self.cycle.push(last_vaule);
            }
            Cycle::Two(value) => {
                self.count += 2;
                let last_vaule = *self.cycle.last().unwrap();
                self.cycle.push(last_vaule);
                self.cycle.push(value + last_vaule);
            }
        }
    }
    fn strength(&self, cycle_count: i32) -> i32 {
        match self.cycle.get(cycle_count as usize - 1) {
            Some(value) => value * cycle_count,
            _ => 0,
        }
    }
}

pub fn problem10_1(path: &str) -> i32 {
    let file = File::open(path).expect("cant read the file");
    let reader = BufReader::new(file);

    let mut signal_count = Signal::new();
    for i in reader.lines() {
        let line_str = i.unwrap();
        let action: Cycle = match line_str.starts_with("noop") {
            true => Cycle::One,
            false => {
                let line_split = line_str.split(" ").collect::<Vec<&str>>();
                let take_value = line_split.get(1).unwrap().parse::<i32>().unwrap();
                Cycle::Two(take_value)
            }
        };
        signal_count.program(action);
    }

    signal_count.strength(20)
        + signal_count.strength(60)
        + signal_count.strength(100)
        + signal_count.strength(140)
        + signal_count.strength(180)
        + signal_count.strength(220)
}

pub fn problem10_2(path: &str) {
    let file = File::open(path).expect("cant read the file");
    let reader = BufReader::new(file);
    let mut signal_count = Signal::new();
    for i in reader.lines() {
        let line_str = i.unwrap();
        let action: Cycle = match line_str.starts_with("noop") {
            true => Cycle::One,
            false => {
                let line_split = line_str.split(" ").collect::<Vec<&str>>();
                let take_value = line_split.get(1).unwrap().parse::<i32>().unwrap();
                Cycle::Two(take_value)
            }
        };
        signal_count.program(action);
    }

    for (idx, i) in signal_count.cycle.iter().enumerate() {
        let cur_idx = if (idx + 1) % 40 == 0 {
            40
        } else {
            (idx + 1) % 40
        };

        if cur_idx as i32 >= *i && cur_idx as i32 <= (*i + 2) {
            print!("#");
        } else {
            print!(".");
        }

        if cur_idx == 40 {
            println!("");
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn tests_y2022_d10_1() {
        let file_path_simple = "testdata/y2022_p10_simple.txt";
        assert_eq!(13140, problem10_1(file_path_simple));

        let file_path = "testdata/y2022_p10.txt";
        assert_eq!(12540, problem10_1(file_path))
    }

    #[test]
    fn tests_y2022_d10_2() {
        let file_path_simple = "testdata/y2022_p10_simple.txt";
        problem10_2(file_path_simple);

        println!("/////////////////////////////");
        let file_path = "testdata/y2022_p10.txt";
        problem10_2(file_path);
    }

    // #[test]
    // fn test_cycle() {
    //     let mut signal_count = Signal::new();
    //     let noop_cycle = Cycle::One;
    //     let _ = &signal_count.program(noop_cycle);
    //     assert_eq!(1, signal_count.count);
    //     assert_eq!(1, signal_count.cycle[1]);
    //
    //     let two_cycle = Cycle::Two(3);
    //     let _ = &signal_count.program(two_cycle);
    //
    //     assert_eq!(1, signal_count.cycle[2]);
    //     assert_eq!(3, signal_count.cycle[3]);
    // }
}
