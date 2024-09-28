
use crate::game::prelude::*;
use pathfinding::prelude::astar;
use std::collections::HashMap;

#[derive(Component)]
pub struct Path(pub Vec<(Pos, usize)>);

#[derive(Component)]
pub struct Obstacle;

#[derive(Clone, Debug, Eq, Hash, Ord, PartialEq, PartialOrd)]
struct Pos(i32, i32);

#[derive(Resource)]
pub struct Pathfinder {
    pub obstacles: HashMap<Entity, Pos>,
    pub max: i32,
}

impl Pathfinder {
    pub fn new(max: i32) -> Self {
        Self {
            obstacles: HashMap::new(),
            max,
        }
    }

    pub fn find_path(&self, start: Pos, goal: Pos) -> Option<Vec<Pos>> {
        if !self.is_within_bounds(&start) || !self.is_within_bounds(&goal) {
            return None;
        }

        astar(
            &start,
            |p| self.successors(p),
            |p| p.distance(&goal),
            |p| *p == goal,
        )
        .map(|(path, _)| path)
    }

    fn successors(&self, pos: &Pos) -> Vec<(Pos, usize)> {
        let &Pos(x, y) = pos;
        vec![
            Pos(x + 1, y),     // Right
            Pos(x - 1, y),     // Left
            Pos(x, y + 1),     // Up
            Pos(x, y - 1),     // Down
            Pos(x + 1, y + 1), // Top-right
            Pos(x - 1, y + 1), // Top-left
            Pos(x + 1, y - 1), // Bottom-right
            Pos(x - 1, y - 1), // Bottom-left
        ]
        .into_iter()
        .filter(|p| !self.is_obstacle(p) && self.is_within_bounds(p))
        .map(|p| (p.clone(), 1))
        .collect()
    }

    fn is_obstacle(&self, pos: &Pos) -> bool {
        self.obstacles.values().any(|obstacle_pos| *obstacle_pos == *pos)
    }

    fn is_within_bounds(&self, pos: &Pos) -> bool {
        pos.0 >= -self.max && pos.0 <= self.max && pos.1 >= -self.max && pos.1 <= self.max
    }
}

impl Pos {
    fn distance(&self, other: &Pos) -> usize {
        (self.0.abs_diff(other.0) + self.1.abs_diff(other.1)) as usize
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use bevy::prelude::*;
    use std::collections::HashMap;

    #[test]
    fn test_find_path_no_obstacles() {
        let pathfinder = Pathfinder::new(10);
        let start = Pos(-2, -2);
        let goal = Pos(9, 9);
        let path = pathfinder.find_path(start.clone(), goal.clone()).unwrap();
        assert_eq!(path.first().unwrap(), &start);
        assert_eq!(path.last().unwrap(), &goal);
    }

    #[test]
    fn test_find_path_with_obstacles() {
        let mut pathfinder = Pathfinder::new(10);
        pathfinder.obstacles.insert(Entity::from_raw(1), Pos(2, 2));
        pathfinder.obstacles.insert(Entity::from_raw(2), Pos(2, 3));
        pathfinder.obstacles.insert(Entity::from_raw(3), Pos(2, 4));
        let start = Pos(0, 0);
        let goal = Pos(5, 5);
        let path = pathfinder.find_path(start.clone(), goal.clone()).unwrap();
        assert!(path.contains(&goal));
        assert!(!path.contains(&Pos(2, 2)));
        assert!(!path.contains(&Pos(2, 3)));
        assert!(!path.contains(&Pos(2, 4)));
    }

    #[test]
    fn test_no_path_due_to_obstacles() {
        let mut pathfinder = Pathfinder::new(5);

        let goal = Pos(0, 0);

        let obstacle_positions = vec![
            Pos(-1, -1), Pos(0, -1), Pos(1, -1),
            Pos(-1, 0),             Pos(1, 0),
            Pos(-1, 1),  Pos(0, 1),  Pos(1, 1),
        ];

        for (i, pos) in obstacle_positions.into_iter().enumerate() {
            pathfinder.obstacles.insert(Entity::from_raw(i as u32), pos);
        }

        let start = Pos(2, 2);

        let path = pathfinder.find_path(start.clone(), goal.clone());

        assert!(path.is_none(), "Expected no path, but found one");
    }

    #[test]
    fn test_narrow_passage() {
        let mut pathfinder = Pathfinder::new(2);

        let goal = Pos(0, 0);

        let obstacle_positions = vec![
            Pos(-1, -1), Pos(0, -1), Pos(1, -1),
            Pos(-1, 0),             
            Pos(-1, 1),  Pos(0, 1),  Pos(1, 1),
        ];

        for (i, pos) in obstacle_positions.into_iter().enumerate() {
            pathfinder.obstacles.insert(Entity::from_raw(i as u32), pos);
        }

        let start = Pos(2, 2);

        let path = pathfinder.find_path(start.clone(), goal.clone()).unwrap();

        assert!(path.contains(&goal));
        assert!(path.contains(&Pos(1, 0)));
    }

}

