use std::ops::{Index, IndexMut};

pub enum DungeonCell {
    Empty,
    Wall,
    Floor,
    Door,
    OpenDoor
}

/// Represents the cells of the current map.
pub struct DungeonFloor {
    pub width: usize,
    pub height: usize,
    pub cells: Vec<DungeonCell>,
}

impl Index<(usize, usize)> for DungeonFloor {
    type Output = DungeonCell;

    fn index(&self, index: (usize, usize)) -> &Self::Output {
        let (x, y) = index;

        if x >= self.width || y >= self.height {
            panic!("x: {}, y: {} are out of bounds. size: ({}, {})",
                x, y, self.width, self.height);
        }

        &self.cells[self.width * y + x]
    }
}

impl IndexMut<(usize, usize)> for DungeonFloor {
    fn index_mut(&mut self, index: (usize, usize)) -> &mut Self::Output {
        let (x, y) = index;

        if x >= self.width || y >= self.height {
            panic!("x: {}, y: {} are out of bounds. size: ({}, {})",
                x, y, self.width, self.height);
        }

        &mut self.cells[self.width * y + x]
    }
}
