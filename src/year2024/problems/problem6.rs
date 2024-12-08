use std::{
    collections::{HashMap, HashSet},
    fs::File,
    io::{BufRead, BufReader},
};

#[derive(Debug, Copy, Clone, Hash, Eq, PartialEq)]
pub enum Cursor {
    Top,
    Right,
    Down,
    Left,
}

#[derive(Debug)]
pub struct Guard {
    cursor: Cursor,
    row: i32,
    col: i32,
    step: HashSet<(usize, usize)>,
    max_row: usize,
    max_col: usize,
}

impl Guard {
    fn new(
        row: i32,
        col: i32,
        cursor: Cursor,
        step: HashSet<(usize, usize)>,
        max_row: usize,
        max_col: usize,
    ) -> Self {
        Self {
            cursor,
            row,
            col,
            step,
            max_row,
            max_col,
        }
    }

    pub fn next(
        &mut self,
        obstacle_set: &HashSet<(usize, usize)>,
        records: Option<&mut HashSet<(usize, usize)>>,
    ) -> bool {
        let mut next_row = self.row;
        let mut next_col = self.col;

        match self.cursor {
            Cursor::Top => next_row -= 1,
            Cursor::Right => next_col += 1,
            Cursor::Down => next_row += 1,
            Cursor::Left => next_col -= 1,
        }

        if next_row < 0
            || next_row >= (self.max_row as i32)
            || next_col < 0
            || next_col >= (self.max_col as i32)
        {
            return false;
        }

        let obstacle = obstacle_set.contains(&(next_row as usize, next_col as usize));
        if obstacle {
            self.turn();
            return true;
        }

        match self.cursor {
            Cursor::Top => self.row = next_row,
            Cursor::Right => self.col = next_col,
            Cursor::Down => self.row = next_row,
            Cursor::Left => self.col = next_col,
        }

        self.step.insert((self.row as usize, self.col as usize));

        if let Some(rc) = records {
            rc.insert((self.row as usize, self.col as usize));
        }

        return true;
    }
    pub fn next_with_new_obstacle(
        &mut self,
        obstacle_map: &mut HashMap<(usize, usize), HashSet<Cursor>>,
        new_obstacle: (usize, usize),
        repeat_count: &mut i32,
    ) -> bool {
        let mut next_row = self.row;
        let mut next_col = self.col;

        match self.cursor {
            Cursor::Top => next_row -= 1,
            Cursor::Right => next_col += 1,
            Cursor::Down => next_row += 1,
            Cursor::Left => next_col -= 1,
        }

        if next_row < 0
            || next_row >= (self.max_row as i32)
            || next_col < 0
            || next_col >= (self.max_col as i32)
        {
            return false;
        }

        let obstacle = obstacle_map.contains_key(&(next_row as usize, next_col as usize));
        if !obstacle && (next_row as usize, next_col as usize) == new_obstacle {
            self.turn();
            return true;
        }

        if obstacle {
            obstacle_map
                .entry((next_row as usize, next_col as usize))
                .and_modify(|f| {
                    if !f.contains(&self.cursor) {
                        f.insert(self.cursor);
                    } else {
                        *repeat_count = *repeat_count + 1;
                    }
                });

            self.turn();
            return true;
        }

        match self.cursor {
            Cursor::Top => self.row = next_row,
            Cursor::Right => self.col = next_col,
            Cursor::Down => self.row = next_row,
            Cursor::Left => self.col = next_col,
        }

        self.step.insert((self.row as usize, self.col as usize));

        return true;
    }
    fn turn(&mut self) {
        match self.cursor {
            Cursor::Top => self.cursor = Cursor::Right,
            Cursor::Right => self.cursor = Cursor::Down,
            Cursor::Down => self.cursor = Cursor::Left,
            Cursor::Left => self.cursor = Cursor::Top,
        }
    }
}

pub fn problem6_1(path: &str) -> usize {
    let file = File::open(path).expect("cant open the file");

    let reader = BufReader::new(file);

    let mut obstacle_set: HashSet<(usize, usize)> = HashSet::new();

    let mut star_point_pos: (usize, usize) = (0, 0);

    let matrix: Vec<Vec<String>> = reader
        .lines()
        .map(|x| {
            x.unwrap()
                .split_terminator("")
                .filter(|s| !s.is_empty())
                .map(|x| x.to_string())
                .collect()
        })
        .collect();

    let mut cursor = Cursor::Top;
    let max_row = matrix.len();
    let max_col = matrix[0].len();
    for i in 0..max_row {
        for j in 0..max_col {
            if matrix[i][j] == "#" {
                obstacle_set.insert((i, j));
                continue;
            }
            if matrix[i][j] == "." {
                continue;
            }
            if matrix[i][j] == "^" {
                cursor = Cursor::Top;
            }

            if matrix[i][j] == ">" {
                cursor = Cursor::Right;
            }

            if matrix[i][j] == "v" {
                cursor = Cursor::Down;
            }

            if matrix[i][j] == "<" {
                cursor = Cursor::Left;
            }
            star_point_pos = (i, j);
        }
    }

    // Including the guard's starting position
    let mut guard = Guard::new(
        star_point_pos.0 as i32,
        star_point_pos.1 as i32,
        cursor,
        HashSet::from([(star_point_pos.0, star_point_pos.1)]),
        max_row,
        max_col,
    );

    loop {
        let next = guard.next(&obstacle_set, None);
        if !next {
            break;
        }
    }
    let step = guard.step.len();

    step
}
pub fn problem6_2(path: &str) -> usize {
    let file = File::open(path).expect("cant open the file");

    let reader = BufReader::new(file);

    let mut obstacle_map: HashMap<(usize, usize), HashSet<Cursor>> = HashMap::new();

    let mut obstacle_set: HashSet<(usize, usize)> = HashSet::new();
    let mut star_point_pos: (usize, usize) = (0, 0);

    let matrix: Vec<Vec<String>> = reader
        .lines()
        .map(|x| {
            x.unwrap()
                .split_terminator("")
                .filter(|s| !s.is_empty())
                .map(|x| x.to_string())
                .collect()
        })
        .collect();

    let mut cursor = Cursor::Top;
    let max_row = matrix.len();
    let max_col = matrix[0].len();
    for i in 0..max_row {
        for j in 0..max_col {
            if matrix[i][j] == "#" {
                obstacle_map.insert((i, j), HashSet::new());
                obstacle_set.insert((i, j));
                continue;
            }
            if matrix[i][j] == "." {
                continue;
            }
            if matrix[i][j] == "^" {
                cursor = Cursor::Top;
            }

            if matrix[i][j] == ">" {
                cursor = Cursor::Right;
            }

            if matrix[i][j] == "v" {
                cursor = Cursor::Down;
            }

            if matrix[i][j] == "<" {
                cursor = Cursor::Left;
            }
            star_point_pos = (i, j);
        }
    }

    let mut count = 0;

    let mut records = HashSet::new();

    let mut guard = Guard::new(
        star_point_pos.0 as i32,
        star_point_pos.1 as i32,
        cursor,
        HashSet::new(),
        max_row,
        max_col,
    );
    loop {
        let next = guard.next(&obstacle_set, Some(&mut records));
        if !next {
            break;
        }
    }
    'middle: for r in records {
        let i = r.0;
        let j = r.1;
        let mut repeat = 0;
        let mut guard = Guard::new(
            star_point_pos.0 as i32,
            star_point_pos.1 as i32,
            cursor,
            HashSet::from([(star_point_pos.0, star_point_pos.1)]),
            max_row,
            max_col,
        );

        if i == star_point_pos.0 && j == star_point_pos.1 {
            continue 'middle;
        }

        if obstacle_map.contains_key(&(i, j)) {
            continue 'middle;
        }

        let mut obstacle_map = obstacle_map.clone();

        loop {
            let next = guard.next_with_new_obstacle(&mut obstacle_map, (i, j), &mut repeat);
            if repeat >= 1 {
                count += 1;
                continue 'middle;
            }
            if !next {
                continue 'middle;
            }
        }
    }

    count
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn tests_y2024_d6_1() {
        let file_path = "testdata/y2024_p6_simple.txt";
        assert_eq!(problem6_1(file_path), 41);

        let file_path = "testdata/y2024_p6.txt";
        assert_eq!(problem6_1(file_path), 4433);
    }

    #[test]
    fn tests_y2024_d6_2() {
        let file_path = "testdata/y2024_p6_simple.txt";
        assert_eq!(problem6_2(file_path), 6);

        let file_path = "testdata/y2024_p6.txt";
        assert_eq!(problem6_2(file_path), 1516);
    }
}
