use crate::game::prelude::*;
use pathfinding::prelude::astar;

#[derive(Resource)]
pub struct Map {
    width: isize,
    height: isize,
    tiles: Vec<Vec<bool>>, // true = walkable, false = non-walkable
}

impl Map {
    pub fn new(width: isize, height: isize) -> Self {
        let tiles = vec![vec![true; width as usize]; height as usize];
        Self { width, height, tiles }
    }

    pub fn set_tile(&mut self, x: isize, y: isize, walkable: bool) {
        if x >= 0 && y >= 0 && x < self.width && y < self.height {
            self.tiles[y as usize][x as usize] = walkable;
        }
    }

    pub fn is_walkable(&self, x: isize, y: isize) -> bool {
        if x >= 0 && y >= 0 && x < self.width && y < self.height {
            self.tiles[y as usize][x as usize]
        } else {
            false
        }
    }

    pub fn neighbors(&self, x: isize, y: isize) -> Vec<(isize, isize)> {
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
            let new_x = x + x_offset;
            let new_y = y + y_offset;

            if new_x >= 0 && new_y >= 0 && new_x < self.width && new_y < self.height {
                if self.is_walkable(new_x, new_y) {
                    neighbors.push((new_x, new_y));
                }
            }
        }

        neighbors
    }

    pub fn find_path(
        &self,
        start: (isize, isize),
        goal: (isize, isize),
    ) -> Option<(Vec<(isize, isize)>, isize)> {
        astar(
            &start,
            |&(x, y)| self.neighbors(x, y).into_iter().map(|p| (p, 1)), // Only return walkable neighbors
            |&(x, y)| (x - goal.0).abs() + (y - goal.1).abs(),
            |&p| p == goal,
        )
    }
}

