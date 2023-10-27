use std::ops::Add;
use std::{fs, io::Read};

static SPLIT_COL: &str = if cfg!(target_os = "windows") {
    "\r\n"
} else {
    "\n"
};

// NOTE: the rule
//
/* enum OpStra {
    A = Rock,
    B = Paper,
    C = Scissors,
}
enum MyStra {
    X = Rock,
    Y = Paper,
    Z = Scissors,
} */

// NOTE: Rock > Scissors > Paper > Rock

#[derive(Debug, PartialEq)]
enum OpStra {
    A,
    B,
    C,
}

// NOTE: problem 1 : X, Y, Z indicate Rock, Paper, and Scissors
// NOTE: problem  2 : X, Y, Z indicate Lost, Draw, and Win
#[derive(Debug, PartialEq)]
enum MyStra {
    X,
    Y,
    Z,
}

enum Score {
    Rock = 1,
    Paper = 2,
    Scissors = 3,
}

enum Outcome {
    Lost = 0,
    Draw = 3,
    Win = 6,
}

// WARNING: !! this is from GPT
impl Add<i32> for Outcome {
    type Output = i32;

    fn add(self, rhs: i32) -> i32 {
        match self {
            Outcome::Lost => 0 + rhs,
            Outcome::Draw => 3 + rhs,
            Outcome::Win => 6 + rhs,
        }
    }
}

impl Add<i32> for Score {
    type Output = i32;

    fn add(self, rhs: i32) -> i32 {
        match self {
            Score::Rock => 1 + rhs,
            Score::Paper => 2 + rhs,
            Score::Scissors => 3 + rhs,
        }
    }
}
trait GetScore {
    fn score(&self) -> i32;
}

impl GetScore for OpStra {
    fn score(&self) -> i32 {
        match self {
            OpStra::A => Score::Rock + 0,
            OpStra::B => Score::Paper + 0,
            OpStra::C => Score::Scissors + 0,
        }
    }
}

impl GetScore for MyStra {
    fn score(&self) -> i32 {
        match self {
            MyStra::X => Score::Rock + 0,
            MyStra::Y => Score::Paper + 0,
            MyStra::Z => Score::Scissors + 0,
        }
    }
}

impl MyStra {
    fn my_lose(&self) -> OpStra {
        match self {
            MyStra::X => OpStra::B,
            MyStra::Y => OpStra::C,
            MyStra::Z => OpStra::A,
        }
    }
}

impl OpStra {
    fn op_win_my_score(&self) -> i32 {
        match self {
            OpStra::A => Score::Scissors + 0,
            OpStra::B => Score::Rock + 0,
            OpStra::C => Score::Paper + 0,
        }
    }

    fn op_lose_my_score(&self) -> i32 {
        match self {
            OpStra::A => Score::Paper + 0,
            OpStra::B => Score::Scissors + 0,
            OpStra::C => Score::Rock + 0,
        }
    }
}

fn match_op_char(s: &str) -> OpStra {
    match s {
        "A" => OpStra::A,
        "B" => OpStra::B,
        "C" => OpStra::C,
        _ => panic!("get wrong rule"),
    }
}
fn match_my_char(s: &str) -> MyStra {
    match s {
        "X" => MyStra::X,
        "Y" => MyStra::Y,
        "Z" => MyStra::Z,
        _ => panic!("get wrong rule"),
    }
}

fn compare_result(a: OpStra, b: MyStra) -> i32 {
    if a.score() == b.score() {
        Outcome::Draw + a.score()
    } else {
        match b.my_lose() == a {
            true => Outcome::Lost + b.score(),
            false => Outcome::Win + b.score(),
        }
    }
}

fn compare_result2(a: OpStra, b: MyStra) -> i32 {
    match b {
        MyStra::X => Outcome::Lost + a.op_win_my_score(),
        MyStra::Y => Outcome::Draw + a.score(),
        MyStra::Z => Outcome::Win + a.op_lose_my_score(),
    }
}

pub fn question1(path: &str) -> i32 {
    let mut file = fs::File::open(path).expect("can't open this file");
    let mut data: String = String::new();
    let _ = file.read_to_string(&mut data);
    let a: Vec<Vec<&str>> = data
        .split_terminator(SPLIT_COL)
        .map(|x: &str| x.split(" ").collect())
        .collect();

    let mut score = 0;
    for ele in a {
        let op_round = match_op_char(ele[0]);
        let my_round = match_my_char(ele[1]);
        score += compare_result(op_round, my_round)
    }
    score
}

pub fn question2(path: &str) -> i32 {
    let mut file = fs::File::open(path).expect("can't open this file");
    let mut data: String = String::new();
    let _ = file.read_to_string(&mut data);
    let a: Vec<Vec<&str>> = data
        .split_terminator(SPLIT_COL)
        .map(|x: &str| x.split(" ").collect())
        .collect();

    let mut score = 0;
    for ele in a {
        let op_round = match_op_char(ele[0]);
        let my_round = match_my_char(ele[1]);
        score += compare_result2(op_round, my_round)
    }
    score
}
#[cfg(test)]
mod tests {
    use super::question1;
    use super::question2;

    #[test]
    fn tests_y2022_d2_1() {
        let file_path = "testdata/y2022_p2.txt";
        let score = question1(file_path);
        assert_eq!(score, 14264);
    }
    #[test]
    fn tests_y2022_d2_2() {
        let file_path_simple = "testdata/y2022_p2_simple.txt";
        let score = question2(file_path_simple);
        assert_eq!(score, 12);

        let file_path = "testdata/y2022_p2.txt";
        let score = question2(file_path);
        assert_eq!(score, 12382);
    }
}
