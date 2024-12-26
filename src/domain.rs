use std::mem::swap;
use std::vec::Vec;

use crate::domain::State::{Alive, Dead};

#[derive(Debug, Clone, PartialEq)]
pub enum State {
    Dead,
    Alive,
}

impl State {
    pub fn is_alive(&self) -> bool {
        self == &Alive
    }
}

pub struct Board {
    width: usize,
    height: usize,
    cells: Vec<State>,
}

impl Board {
    pub fn new(width: usize, height: usize, living_cells: Vec<(usize, usize)>) -> Self {
        let mut vec = vec!(Dead; width * height);
        for (x, y) in living_cells {
            let index = y * width + x;
            if x > width || y > height {
                panic!("cannot insert a cell with index {}, {}", x, y);
            }
            vec[index] = Alive;
        }
        Board { width, height, cells: vec }
    }

    ///
    /// On renvoie ```Dead``` quand la cellule n'existe pas
    pub fn cell(&self, x: usize, y: usize) -> &State {
        if x < self.width && y < self.height {
            let index = y * self.width + x;
            return self.cells.get(index as usize).unwrap_or_else(|| &Dead);
        }
        &Dead
    }

    pub fn update(&mut self) {
        let mut vec = Vec::with_capacity(self.width * self.height);
        for (i, current_cell) in self.cells.iter().enumerate() {
            let (x, y) = self.coordinates(i);
            let neighbors = self.neighbors(x, y);

            let alive_neighbors = neighbors.iter()
                .filter(|s| s.is_alive())
                .count();

            let new_state = match (alive_neighbors, current_cell) {
                (3, _) => Alive,
                (2, Alive) => Alive,
                (_, _) => Dead,
            };
            vec.push(new_state);
        }
        swap(&mut self.cells, &mut vec);
    }

    fn neighbors(&self, x: usize, y: usize) -> Vec<&State> {
        let mut neighbors = Vec::with_capacity(8);

        neighbors.push(self.cell(x.wrapping_sub(1), y.wrapping_sub(1)));
        neighbors.push(self.cell(x, y.wrapping_sub(1)));
        neighbors.push(self.cell(x + 1, y.wrapping_sub(1)));
        neighbors.push(self.cell(x.wrapping_sub(1), y + 1));
        neighbors.push(self.cell(x, y + 1));
        neighbors.push(self.cell(x + 1, y + 1));
        neighbors.push(self.cell(x.wrapping_sub(1), y));
        neighbors.push(self.cell(x + 1, y));

        neighbors
    }

    pub fn coordinates(&self, index: usize) -> (usize, usize) {
        (index % self.width, index / self.height)
    }
    pub fn width(&self) -> usize {
        self.width
    }
    pub fn height(&self) -> usize {
        self.height
    }
}

#[cfg(test)]
mod tests {
    // Note this useful idiom: importing names from outer (for mod tests) scope.
    use super::*;

    #[test]
    fn should_create_new_board() {
        // Given / When
        let board = Board::new(2, 2, vec!());

        // Then
        assert!(matches!(board.width, 2));
        assert!(matches!(board.height, 2));
        assert!(matches!(board.cells[0], Dead));
        assert!(matches!(board.cells[1], Dead));
        assert!(matches!(board.cells[2], Dead));
        assert!(matches!(board.cells[3], Dead));
    }

    #[test]
    fn should_create_new_board_with_alive_cells() {
        // Given / When
        let board = Board::new(2, 2, vec!((1, 1)));

        // Then
        assert!(matches!(board.width, 2));
        assert!(matches!(board.height, 2));
        assert!(matches!(board.cells[0], Dead));
        assert!(matches!(board.cells[1], Dead));
        assert!(matches!(board.cells[2], Dead));
        assert!(matches!(board.cells[3], Alive));
    }

    #[test]
    fn should_find_cell_by_coordinate() {
        // Given
        let board = Board::new(2, 2, vec!((1, 1)));

        // When
        let result = board.cell(1, 1);

        // Then
        assert!(matches!(result, Alive));
    }
}