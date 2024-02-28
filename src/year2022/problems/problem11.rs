#![allow(dead_code)]
#![allow(unused_variables)]
use std::fs::File;
use std::io::Read;

static SPLIT_SECTION: &str = if cfg!(target_os = "windows") {
    "\r\n\r\n"
} else {
    "\n\n"
};

#[derive(Debug)]
struct MonkeyState {
    no: i32,
    start_items: Option<Vec<i32>>,
    divide_value: i32,
    operate_symbol: String,
    operate_value: i32,
    throw_to: [i32; 2],
    count: i32,
}

trait OperationWorry {
    fn operate(&self, item: i32) -> i32;
    fn divided(&self, worry_score: i32) -> i32;
}
impl OperationWorry for MonkeyState {
    fn operate(&self, item: i32) -> i32 {
        if self.operate_symbol == "+".to_string() {
            item + &self.operate_value
        } else if self.operate_symbol == "*".to_string() {
            item * &self.operate_value
        } else if self.operate_symbol == "^".to_string() {
            item * item
        } else {
            0
        }
    }
    fn divided(&self, worry_score: i32) -> i32 {
        match worry_score % &self.divide_value == 0 {
            true => self.throw_to[0],
            false => self.throw_to[1],
        }
    }
}
impl MonkeyState {
    fn new(section: Vec<&str>) -> Self {
        let monkey_key = section[0]
            .split(" ")
            .collect::<Vec<&str>>()
            .get(1)
            .unwrap()
            .to_string();
        let no = monkey_key
            .split(":")
            .collect::<Vec<&str>>()
            .get(0)
            .unwrap()
            .parse::<i32>()
            .unwrap();

        let start_string = section[1]
            .split(": ")
            .collect::<Vec<&str>>()
            .get(1)
            .unwrap()
            .to_string();

        let start_items = Some(
            start_string
                .split(",")
                .map(|x| x.trim().parse::<i32>().unwrap())
                .collect::<Vec<i32>>(),
        );

        let operate_value = section[2]
            .split(" ")
            .collect::<Vec<&str>>()
            .last()
            .unwrap()
            .parse::<i32>()
            .unwrap_or(0);

        let operate_symbol_parse = section[2].split(" ").collect::<Vec<&str>>();

        let operate_symbol = if operate_value == 0 {
            "^".to_string()
        } else {
            operate_symbol_parse
                .get(operate_symbol_parse.len() - 2)
                .unwrap()
                .to_string()
        };

        let divide_value = section[3]
            .split(" ")
            .collect::<Vec<&str>>()
            .last()
            .unwrap()
            .parse::<i32>()
            .unwrap();

        let monkey_true = section[4]
            .split(" ")
            .collect::<Vec<&str>>()
            .last()
            .unwrap()
            .parse::<i32>()
            .unwrap();

        let monkey_false = section[5]
            .split(" ")
            .collect::<Vec<&str>>()
            .last()
            .unwrap()
            .parse::<i32>()
            .unwrap();

        MonkeyState {
            no,
            start_items,
            divide_value,
            operate_symbol,
            operate_value,
            throw_to: [monkey_true, monkey_false],
            count: 0,
        }
    }
}

pub fn problem11_1(path: &str) -> i32 {
    let mut file = File::open(path).expect("can not read the file");
    let mut content = String::new();
    let _ = file.read_to_string(&mut content);
    let data_list: Vec<&str> = content.split_terminator(SPLIT_SECTION).collect();

    let mut monkey_list = data_list
        .iter()
        .map(|section| {
            let paraph: Vec<&str> = section.split_terminator("\n").collect();
            MonkeyState::new(paraph)
        })
        .collect::<Vec<MonkeyState>>();

    let mut monkey_throw_buffer: Vec<Vec<i32>> = vec![Vec::new(); monkey_list.len()];

    for _ in 0..20 {
        let _ = monkey_list.iter_mut().for_each(|monkey| {
            let start_items = monkey.start_items.take();
            let new_items = [
                start_items.unwrap_or(Vec::new()).clone(),
                monkey_throw_buffer.get(monkey.no as usize).unwrap().clone(),
            ]
            .concat();

            monkey_throw_buffer[monkey.no as usize] = Vec::new();

            new_items.iter().for_each(|i| {
                monkey.count += 1;
                let worry_score = monkey.operate(*i) / 3;
                let throw_to_monkey_index = monkey.divided(worry_score) as usize;
                match monkey_throw_buffer.get_mut(throw_to_monkey_index) {
                    Some(s) => {
                        s.push(worry_score);
                    }
                    None => {}
                }
            })
        });
    }

    let mut count_list = monkey_list.iter().map(|c| c.count).collect::<Vec<i32>>();
    count_list.sort_by(|a, b| b.cmp(a));

    count_list[0] * count_list[1]
}
pub fn problem11_2() {}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn tests_y2022_d11_1() {
        let file_path_simple = "testdata/y2022_p11_simple.txt";
        assert_eq!(10605, problem11_1(file_path_simple));

        let file_path = "testdata/y2022_p11.txt";
        assert_eq!(117640, problem11_1(file_path));
    }

    // #[test]
    // fn tests_y2022_d11_2() {}
}
