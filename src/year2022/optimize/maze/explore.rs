use std::collections::VecDeque;

use super::{maze::Maze, position::Position, Column, Row};

pub struct Explorer {
    maze: Maze,
    distance: Vec<Vec<i32>>,
    queue: VecDeque<Position>,
}

impl Explorer {
    pub fn new(maze: Maze, start: Position) -> Self {
        let row = maze.get_row() as usize;
        let col = maze.get_col() as usize;
        let mut queue: VecDeque<Position> = VecDeque::new();
        let mut distance = vec![vec![-1; col]; row];
        distance[start.0 as usize][start.1 as usize] = 0;
        queue.push_front(start);
        Explorer {
            maze,
            distance,
            queue,
        }
    }

    pub fn best_path(&mut self, end_row: Row, end_col: Column) -> Option<i32> {
        while self.queue.len() != 0 && self.distance[end_row as usize][end_col as usize] == -1 {
            let pop_neightbor = self.queue.pop_front();
            dbg!(&pop_neightbor);

            if let Some(pop_item) = pop_neightbor {
                for neightbors in pop_item.neightbors() {
                    if !self.maze.check_move(&pop_item, &neightbors) {
                        continue;
                    }

                    if self.distance[neightbors.0 as usize][neightbors.1 as usize] == -1 {
                        self.distance[neightbors.0 as usize][neightbors.1 as usize] =
                            self.distance[pop_item.0 as usize][pop_item.1 as usize] + 1;
                        self.queue
                            .push_back(Position::new(neightbors.0, neightbors.1));
                    }
                }
            }
        }

        match self.distance[end_row as usize][end_col as usize] {
            -1 => None,
            x => Some(x),
        }
    }
}

#[cfg(test)]
mod tests {
    use std::{
        fs::File,
        io::{BufRead, BufReader},
    };

    use crate::year2022::optimize::maze::{maze::Maze, position::Position};

    use super::Explorer;

    #[test]
    fn test_check_explorer() {
        let path = "./testdata/y2022_p12.txt";
        let file = File::open(path).expect("no file exists");
        let reader = BufReader::new(file);

        let matrix = reader
            .lines()
            .map(|x| x.unwrap().as_bytes().to_vec())
            .collect::<Vec<_>>();

        let matrix = matrix
            .iter()
            .map(|i| {
                let to_char = i.iter().map(|j| *j as char).collect::<Vec<char>>();
                to_char
            })
            .collect::<Vec<Vec<char>>>();

        let maze = Maze::new(matrix);
        let start = Position::new(20, 0);

        let mut explorer = Explorer::new(maze, start);
        let result = explorer.best_path(20, 46);

        assert_eq!(Some(352), result);
    }
}
