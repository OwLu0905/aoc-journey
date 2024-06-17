use std::collections::VecDeque;

use super::{maze::Maze, position::Position, puzzle::Puzzle};

pub struct Explorer {
    maze: Maze,
    end: Position,
    distance: Vec<Vec<i32>>,
    queue: VecDeque<Position>,
}

impl Explorer {
    pub fn new(puzzle: Puzzle) -> Self {
        let maze = puzzle.maze;
        let start = puzzle.start;
        let end = puzzle.end;
        let row = maze.get_row() as usize;
        let col = maze.get_col() as usize;
        let mut queue: VecDeque<Position> = VecDeque::new();
        let mut distance = vec![vec![-1; col]; row];
        distance[start.0 as usize][start.1 as usize] = 0;
        queue.push_front(start);

        Explorer {
            maze,
            end,
            distance,
            queue,
        }
    }

    pub fn best_path(&mut self) -> Option<i32> {
        let end_item = &self.end;
        let end_row = end_item.0 as usize;
        let end_col = end_item.1 as usize;
        let maze = &self.maze;
        while self.queue.len() != 0 && self.distance[end_row][end_col] == -1 {
            let pop_neightbor = self.queue.pop_front();

            if let Some(pop_item) = pop_neightbor {
                for neightbors in pop_item.neightbors() {
                    if !maze.check_move(&pop_item, &neightbors) {
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

    use crate::year2022::optimize::maze::{maze::Maze, puzzle::Puzzle};

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

        let puzzle = Puzzle::new(maze, 'S', 'E');

        let mut explorer = Explorer::new(puzzle);
        let result = explorer.best_path();

        assert_eq!(Some(352), result);
    }
}
