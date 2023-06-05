use std::mem::swap;
use std::vec::Vec;
use crate::domain::State::{Alive, Dead};

#[derive(Debug, Clone, PartialEq)]
pub enum State {
    Dead,
    Alive,
}

pub struct Board {
    width: usize,
    height: usize,
    cells: Vec<State>,
}

impl Board {
    pub fn new(width: usize, height: usize) -> Self {
        let vec = vec!(Dead; width * height);
        Board { width, height, cells: vec }
    }

    pub fn cell(&self, x: isize, y: isize) -> &State {
        let index = y * self.width as isize + x;
        return self.cells.get(index as usize).unwrap_or_else(|| &Dead);
    }

    pub fn alive(&mut self, x: usize, y: usize) {
        if x > 0 && y > 0 {
            let index = y * self.width + x;
            self.cells.insert(index, Alive);
        }
    }

    pub fn update(&mut self) {
        let mut vec = Vec::new();
        for (i, state) in self.cells.iter().enumerate() {
            let (x, y) = self.coordinates(i);
            let neighbors = self.neighbors(x, y);

            let num = neighbors.iter()
                .map(|s| if **s == Alive { 1 } else { 0 })
                .fold(0, |k, i| k + i);

            let new_state = match (num, &state) {
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

        neighbors.push(self.cell(x as isize - 1, y as isize - 1));
        neighbors.push(self.cell(x as isize, y as isize - 1));
        neighbors.push(self.cell(x as isize + 1, y as isize - 1));
        neighbors.push(self.cell(x as isize - 1, y as isize + 1));
        neighbors.push(self.cell(x as isize, y as isize + 1));
        neighbors.push(self.cell(x as isize + 1, y as isize + 1));
        neighbors.push(self.cell(x as isize - 1, y as isize));
        neighbors.push(self.cell(x as isize + 1, y as isize));

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
