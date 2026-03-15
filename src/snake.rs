use crate::models::{Direction, Position};
use rand::seq::IteratorRandom;
use std::collections::{HashSet, VecDeque};

pub enum SnakeMoveResult {
    Moved { tail_removed: Position },
    AteFood { new_food: Position },
    Collision,
}

pub struct SnakeGame {
    pub cols: u16,
    pub rows: u16,
    pub body: VecDeque<Position>,
    pub direction: Direction,
    pub food: Position,
    pub score: u32,
}

impl SnakeGame {
    pub fn new(cols: u16, rows: u16) -> Self {
        let cx = cols / 2;
        let cy = rows / 2;
        let mut body = VecDeque::new();
        body.push_back(Position::new(cx, cy));
        body.push_back(Position::new(cx - 1, cy));
        body.push_back(Position::new(cx.saturating_sub(2), cy));

        let mut game = Self {
            cols,
            rows,
            body,
            direction: Direction::Right,
            food: Position::new(0, 0),
            score: 0,
        };
        game.food = game.spawn_food();
        game
    }

    pub fn head(&self) -> Position {
        self.body[0]
    }

    pub fn tick_ms(&self) -> u64 {
        300u64.saturating_sub(u64::from(self.score) * 10).max(100)
    }

    pub fn level(&self) -> u32 {
        1 + self.score / 5
    }

    pub fn try_change_direction(&mut self, dir: Direction) {
        let allowed = !matches!(
            (self.direction, dir),
            (Direction::Up, Direction::Down)
                | (Direction::Down, Direction::Up)
                | (Direction::Left, Direction::Right)
                | (Direction::Right, Direction::Left)
        );
        if allowed {
            self.direction = dir;
        }
    }

    pub fn tick(&mut self) -> SnakeMoveResult {
        let head = self.head();
        let new_head = match head.neighbor(self.direction) {
            Some(p) if p.x < self.cols && p.y < self.rows => p,
            _ => return SnakeMoveResult::Collision,
        };

        if self.body.contains(&new_head) {
            return SnakeMoveResult::Collision;
        }

        self.body.push_front(new_head);

        if new_head == self.food {
            self.score += 1;
            let new_food = self.spawn_food();
            SnakeMoveResult::AteFood { new_food }
        } else {
            let tail = self.body.pop_back().unwrap();
            SnakeMoveResult::Moved { tail_removed: tail }
        }
    }

    fn spawn_food(&self) -> Position {
        let mut rng = rand::rng();
        let occupied: HashSet<Position> = self.body.iter().copied().collect();
        (0..self.cols)
            .flat_map(|x| (0..self.rows).map(move |y| Position::new(x, y)))
            .filter(|p| !occupied.contains(p))
            .choose(&mut rng)
            .unwrap_or(Position::new(0, 0))
    }

    pub fn restart(&mut self) {
        let cols = self.cols;
        let rows = self.rows;
        *self = Self::new(cols, rows);
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn snake_starts_with_three_segments() {
        let game = SnakeGame::new(10, 10);
        assert_eq!(game.body.len(), 3);
    }

    #[test]
    fn snake_moves_in_direction() {
        let mut game = SnakeGame::new(10, 10);
        let head_before = game.head();
        game.direction = Direction::Right;
        if let SnakeMoveResult::Moved { .. } | SnakeMoveResult::AteFood { .. } = game.tick() {
            assert_eq!(game.head().x, head_before.x + 1);
            assert_eq!(game.head().y, head_before.y);
        }
    }

    #[test]
    fn cannot_reverse_direction() {
        let mut game = SnakeGame::new(10, 10);
        game.direction = Direction::Right;
        game.try_change_direction(Direction::Left);
        assert_eq!(game.direction, Direction::Right);
    }

    #[test]
    fn can_change_to_perpendicular() {
        let mut game = SnakeGame::new(10, 10);
        game.direction = Direction::Right;
        game.try_change_direction(Direction::Up);
        assert_eq!(game.direction, Direction::Up);
    }

    #[test]
    fn eating_food_grows_snake() {
        let mut game = SnakeGame::new(10, 10);
        let initial_len = game.body.len();
        game.food = Position::new(game.head().x + 1, game.head().y);
        game.direction = Direction::Right;
        assert!(matches!(game.tick(), SnakeMoveResult::AteFood { .. }));
        assert_eq!(game.body.len(), initial_len + 1);
    }

    #[test]
    fn collision_with_wall() {
        let mut game = SnakeGame::new(10, 10);
        game.body.clear();
        game.body.push_back(Position::new(9, 5));
        game.direction = Direction::Right;
        assert!(matches!(game.tick(), SnakeMoveResult::Collision));
    }

    #[test]
    fn collision_with_self() {
        let mut game = SnakeGame::new(10, 10);
        game.body.clear();
        game.body.push_back(Position::new(5, 5));
        game.body.push_back(Position::new(6, 5));
        game.body.push_back(Position::new(6, 4));
        game.body.push_back(Position::new(5, 4));
        game.body.push_back(Position::new(4, 4));
        game.body.push_back(Position::new(4, 5));
        game.body.push_back(Position::new(4, 6));
        game.body.push_back(Position::new(5, 6));
        game.direction = Direction::Down;
        assert!(matches!(game.tick(), SnakeMoveResult::Collision));
    }

    #[test]
    fn level_increases_every_five_points() {
        let mut game = SnakeGame::new(10, 10);
        assert_eq!(game.level(), 1);
        game.score = 5;
        assert_eq!(game.level(), 2);
        game.score = 10;
        assert_eq!(game.level(), 3);
    }

    #[test]
    fn tick_speed_decreases_with_score() {
        let mut game = SnakeGame::new(10, 10);
        let slow = game.tick_ms();
        game.score = 10;
        let fast = game.tick_ms();
        assert!(fast < slow);
    }
}
