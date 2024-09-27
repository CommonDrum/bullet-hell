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

    fn in_bounds(&self, x: isize, y: isize) -> bool {
        x >= 0 && x < self.width && y >= 0 && y < self.height
    }

    fn is_walkable(&self, x: isize, y: isize) -> bool {
        self.tiles
            .get(y as usize)
            .and_then(|row| row.get(x as usize))
            .cloned()
            .unwrap_or(false)
    }

    fn successors(&self, position: (isize, isize)) -> Vec<((isize, isize), u32)> {
        let (x, y) = position;
        let possible_moves = vec![
            (x + 1, y + 2),
            (x + 1, y - 2),
            (x - 1, y + 2),
            (x - 1, y - 2),
            (x + 2, y + 1),
            (x + 2, y - 1),
            (x - 2, y + 1),
            (x - 2, y - 1),
        ];

        possible_moves
            .into_iter()
            .filter(|&(nx, ny)| self.in_bounds(nx, ny) && self.is_walkable(nx, ny))
            .map(|p| (p, 1)) // Cost of each move is 1
            .collect()
    }

    fn heuristic(&self, current: (isize, isize), goal: (isize, isize)) -> u32 {
        ((current.0 - goal.0).abs() + (current.1 - goal.1).abs()) as u32
    }

    pub fn find_path(
        &self,
        start: (isize, isize),
        goal: (isize, isize),
    ) -> Option<(Vec<(isize, isize)>, isize)> {
        astar(
            &start,
            |p| self.successors(*p),
            |p| self.heuristic(*p, goal) / 3,
            |&p| p == goal,
        )
        .map(|(path, cost)| (path, cost as isize))
    }

    pub fn set_tile(&mut self, x: isize, y: isize, walkable: bool) {
        if self.in_bounds(x, y) {
            if let Some(row) = self.tiles.get_mut(y as usize) {
                if let Some(tile) = row.get_mut(x as usize) {
                    *tile = walkable;
                }
            }
        }
    }
}

