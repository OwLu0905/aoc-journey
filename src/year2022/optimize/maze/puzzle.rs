use super::{maze::Maze, position::Position};

pub struct Puzzle {
    pub maze: Maze,
    pub start: Position,
    pub end: Position,
}

impl Puzzle {
    pub fn new(maze: Maze, start_char: char, end_char: char) -> Self {
        let mut start = Position::new(0, 0);
        let mut end = Position::new(0, 0);

        for j in 0..maze.get_row() {
            for i in 0..maze.get_col() {
                if maze.heights[j as usize][i as usize] == start_char {
                    start = Position::new(j, i);
                }

                if maze.heights[j as usize][i as usize] == end_char {
                    end = Position::new(j, i);
                }
            }
        }

        Self { maze, start, end }
    }
}
