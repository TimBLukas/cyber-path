use crate::models::{Direction, Position};
use anyhow::{Result, bail};
use rand::seq::IteratorRandom;
use std::collections::HashSet;

const ALL_DIRS: [Direction; 4] = [
    Direction::Up,
    Direction::Down,
    Direction::Left,
    Direction::Right,
];

const MAX_PATH_ATTEMPTS: u32 = 1_000;

#[derive(Debug)]
pub enum MoveResult {
    Correct(Position),
    RoundComplete,
    Wrong,
}

pub struct Game {
    pub round: u32,
    pub cols: u16,
    pub rows: u16,
    pub path: Vec<Position>,
    pub player_index: usize,
}

impl Game {
    pub fn new(cols: u16, rows: u16) -> Self {
        Self {
            round: 1,
            cols,
            rows,
            path: Vec::new(),
            player_index: 0,
        }
    }

    pub fn move_count(&self) -> u32 {
        2 + self.round
    }

    pub fn preview_step_ms(&self) -> u64 {
        700u64.saturating_sub(u64::from(self.round) * 30).max(200)
    }

    pub fn preview_hold_ms(&self) -> u64 {
        2000u64.saturating_sub(u64::from(self.round) * 100).max(500)
    }

    pub fn generate_path(&mut self) -> Result<()> {
        self.path = create_path(self.cols, self.rows, self.move_count())?;
        self.player_index = 0;
        Ok(())
    }

    pub fn start_position(&self) -> Position {
        self.path[0]
    }

    pub fn remaining_path(&self) -> &[Position] {
        &self.path[self.player_index + 1..]
    }

    pub fn check_move(&mut self, direction: Direction) -> MoveResult {
        let current = self.path[self.player_index];
        let expected = self.path[self.player_index + 1];

        if current.direction_to(expected) == Some(direction) {
            self.player_index += 1;
            if self.player_index >= self.path.len() - 1 {
                MoveResult::RoundComplete
            } else {
                MoveResult::Correct(expected)
            }
        } else {
            MoveResult::Wrong
        }
    }

    pub fn advance_round(&mut self) {
        self.round += 1;
    }
}

fn create_path(cols: u16, rows: u16, moves: u32) -> Result<Vec<Position>> {
    let mut rng = rand::rng();

    for _ in 0..MAX_PATH_ATTEMPTS {
        let x = (0..cols).choose(&mut rng).unwrap_or(0);
        let y = (0..rows).choose(&mut rng).unwrap_or(0);
        let start = Position::new(x, y);

        let mut path = vec![start];
        let mut visited = HashSet::from([start]);
        let mut stuck = false;

        for _ in 0..moves {
            let current = *path.last().unwrap();
            let candidates: Vec<Position> = ALL_DIRS
                .iter()
                .filter_map(|&dir| {
                    current
                        .neighbor(dir)
                        .filter(|p| p.x < cols && p.y < rows && !visited.contains(p))
                })
                .collect();

            if let Some(&next) = candidates.iter().choose(&mut rng) {
                visited.insert(next);
                path.push(next);
            } else {
                stuck = true;
                break;
            }
        }

        if !stuck {
            return Ok(path);
        }
    }

    bail!("failed to generate a valid path after {MAX_PATH_ATTEMPTS} attempts")
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn path_has_correct_length() {
        let path = create_path(10, 10, 5).unwrap();
        assert_eq!(path.len(), 6);
    }

    #[test]
    fn path_has_no_duplicates() {
        let path = create_path(10, 10, 8).unwrap();
        let set: HashSet<_> = path.iter().copied().collect();
        assert_eq!(set.len(), path.len());
    }

    #[test]
    fn path_steps_are_adjacent() {
        let path = create_path(10, 10, 6).unwrap();
        for pair in path.windows(2) {
            assert!(pair[0].direction_to(pair[1]).is_some());
        }
    }

    #[test]
    fn path_stays_in_bounds() {
        let cols = 5;
        let rows = 5;
        let path = create_path(cols, rows, 4).unwrap();
        for pos in &path {
            assert!(pos.x < cols && pos.y < rows);
        }
    }

    #[test]
    fn check_move_correct() {
        let mut game = Game::new(10, 10);
        game.path = vec![
            Position::new(1, 1),
            Position::new(2, 1),
            Position::new(3, 1),
        ];
        game.player_index = 0;
        assert!(matches!(
            game.check_move(Direction::Right),
            MoveResult::Correct(_)
        ));
    }

    #[test]
    fn check_move_wrong() {
        let mut game = Game::new(10, 10);
        game.path = vec![
            Position::new(1, 1),
            Position::new(2, 1),
            Position::new(3, 1),
        ];
        game.player_index = 0;
        assert!(matches!(game.check_move(Direction::Up), MoveResult::Wrong));
    }

    #[test]
    fn check_move_round_complete() {
        let mut game = Game::new(10, 10);
        game.path = vec![Position::new(1, 1), Position::new(2, 1)];
        game.player_index = 0;
        assert!(matches!(
            game.check_move(Direction::Right),
            MoveResult::RoundComplete
        ));
    }
}
