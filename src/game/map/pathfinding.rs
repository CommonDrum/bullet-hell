use crate::game::prelude::*;
use pathfinding::prelude::astar;

pub struct Grid {
    width: usize,
    height: usize,
    tiles: Vec<Vec<bool>>,
}

impl Grid {
    pub fn new(width: usize, height: usize) -> Self {
        let tiles = vec![vec![true; width]; height];
        Self { width, height, tiles }
    }

    pub fn set_tile(&mut self, x: usize, y: usize, walkable: bool) {
        if x < self.width && y < self.height {
            self.tiles[y][x] = walkable;
        }
    }

    pub fn is_walkable(&self, x: usize, y: usize) -> Option<bool> {
        if x < self.width && y < self.height {
            Some(self.tiles[y][x])
        } else {
            None
        }
    }

    pub fn neighbors(&self, x: usize, y: usize) -> Vec<(usize, usize)> {
        let mut neighbors = Vec::new();
        let directions = [
            (0, 1),   // Down
            (1, 0),   // Right
            (0, -1),  // Up
            (-1, 0),  // Left
            (1, 1),   // Down-right
            (1, -1),  // Up-right
            (-1, 1),  // Down-left
            (-1, -1), // Up-left
        ];

        for (x_offset, y_offset) in directions.iter() {
            let new_x = x as isize + x_offset;
            let new_y = y as isize + y_offset;

            if new_x >= 0 && new_y >= 0 {
                let new_x = new_x as usize;
                let new_y = new_y as usize;

                if new_x < self.width && new_y < self.height && self.tiles[new_y][new_x] {
                    neighbors.push((new_x, new_y));
                }
            }
        }

        neighbors
    }

    pub fn find_path(
        &self,
        start: (usize, usize),
        goal: (usize, usize),
    ) -> Option<(Vec<(usize, usize)>, usize)> {
        astar(
            &start,
            |&(x, y)| self.neighbors(x, y).into_iter().map(|p| (p, 1)),
            |&(x, y)| (x as isize - goal.0 as isize).abs() + (y as isize - goal.1 as isize),
            |&p| p == goal,
        )
    }
}
