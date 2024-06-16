use super::{Column, Row};

#[derive(Debug)]
pub struct Position(pub Row, pub Column);

impl PartialEq for Position {
    fn eq(&self, other: &Self) -> bool {
        self.0 == other.0 && self.1 == other.1
    }
}

impl Position {
    pub const DR: [i32; 4] = [-1, 1, 0, 0];
    pub const DC: [i32; 4] = [0, 0, -1, 1];

    pub fn new(row: Row, col: Column) -> Self {
        Self(row, col)
    }
    pub fn neightbors(&self) -> Vec<Self> {
        let row = self.0;
        let col = self.1;
        let mut neightbor = Vec::new();
        for i in 0..4 {
            let new_row = row + Self::DR[i];
            let new_col = col + Self::DC[i];
            neightbor.push(Position(new_row, new_col));
        }
        neightbor
    }
}
