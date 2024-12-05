use super::{position::Position, Row};

#[derive(Debug)]
pub struct Maze {
    pub heights: Vec<Vec<char>>,
}

impl Maze {
    pub fn new(heights: Vec<Vec<char>>) -> Self {
        Self { heights }
    }
    pub fn get_row(&self) -> Row {
        self.heights.len() as i32
    }

    pub fn get_col(&self) -> Row {
        if let Some(col) = self.heights.get(0) {
            return col.len() as i32;
        }
        0
    }

    pub fn contains(&self, pos: &Position) -> bool {
        if pos.0 < 0 || pos.1 < 0 {
            return false;
        }

        let max_row = self.get_row();
        let max_col = self.get_col();

        if pos.0 >= max_row || pos.1 >= max_col {
            return false;
        }

        true
    }

    /// move upward
    pub fn check_move(&self, from: &Position, to: &Position) -> bool {
        if !self.contains(from) || !self.contains(to) {
            return false;
        }

        let from_row = from.0 as usize;
        let from_col = from.1 as usize;

        let to_row = to.0 as usize;
        let to_col = to.1 as usize;

        let mut from_height = self.heights[from_row][from_col];
        let mut to_height = self.heights[to_row][to_col];

        if from_height == 'S' {
            from_height = 'a'
        }

        if from_height == 'E' {
            from_height = 'z'
        }

        if to_height == 'S' {
            to_height = 'a'
        }

        if to_height == 'E' {
            to_height = 'z'
        }

        let gap = (to_height as u8) as i32 - (from_height as u8) as i32;

        gap <= 1
    }
    pub fn check_move_down() {
        // TODO: use end position to find the best start position (p12-2)
        todo!()
    }
}

#[cfg(test)]
mod tests {

    use crate::year2022::optimize::maze::position::Position;

    use super::Maze;

    /**
     * [0, 1, 0]
     * [0, 0, 1]
     * [0, 1, 0]
     */
    #[test]
    fn test_check_move() {
        let matrix = vec![
            vec!['a', 'c', 'a'],
            vec!['a', 'a', 'b'],
            vec!['a', 'b', 'a'],
        ];
        let _maze = Maze::new(matrix);
        let _center = Position::new(1, 1);
        // for i in center.neightbors() {
        //     if !maze.check_move(&center, &i) {
        //         dbg!(&center, &i);
        //     }
        // }
    }
}
