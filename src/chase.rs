use crate::models::{Direction, Position};
use rand::seq::IteratorRandom;

const ALL_DIRS: [Direction; 4] = [
    Direction::Up,
    Direction::Down,
    Direction::Left,
    Direction::Right,
];

pub enum ChaseMoveResult {
    Moved {
        old_player: Position,
        old_bot: Position,
    },
    Caught {
        old_player: Position,
        old_bot: Position,
    },
    Survived {
        old_player: Position,
        old_bot: Position,
    },
    InvalidMove,
}

pub struct ChaseGame {
    pub round: u32,
    pub cols: u16,
    pub rows: u16,
    pub player_pos: Position,
    pub bot_pos: Position,
    pub moves_survived: u32,
}

impl ChaseGame {
    pub fn new(cols: u16, rows: u16) -> Self {
        let (player_pos, bot_pos) = Self::spawn_positions(cols, rows);
        Self {
            round: 1,
            cols,
            rows,
            player_pos,
            bot_pos,
            moves_survived: 0,
        }
    }

    fn spawn_positions(cols: u16, rows: u16) -> (Position, Position) {
        let mut rng = rand::rng();
        let px = (0..cols).choose(&mut rng).unwrap_or(0);
        let py = (0..rows).choose(&mut rng).unwrap_or(0);
        let bx = if px < cols / 2 { cols - 1 } else { 0 };
        let by = if py < rows / 2 { rows - 1 } else { 0 };
        (Position::new(px, py), Position::new(bx, by))
    }

    pub fn moves_to_survive(&self) -> u32 {
        15 + (self.round - 1) * 5
    }

    pub fn bot_step_size(&self) -> u32 {
        1 + (self.round - 1) / 2
    }

    pub fn move_player(&mut self, dir: Direction) -> ChaseMoveResult {
        let old_player = self.player_pos;
        let old_bot = self.bot_pos;

        let new_pos = match self.player_pos.neighbor(dir) {
            Some(p) if p.x < self.cols && p.y < self.rows => p,
            _ => return ChaseMoveResult::InvalidMove,
        };
        self.player_pos = new_pos;

        if self.player_pos == self.bot_pos {
            return ChaseMoveResult::Caught {
                old_player,
                old_bot,
            };
        }

        self.move_bot();

        if self.player_pos == self.bot_pos {
            return ChaseMoveResult::Caught {
                old_player,
                old_bot,
            };
        }

        self.moves_survived += 1;

        if self.moves_survived >= self.moves_to_survive() {
            ChaseMoveResult::Survived {
                old_player,
                old_bot,
            }
        } else {
            ChaseMoveResult::Moved {
                old_player,
                old_bot,
            }
        }
    }

    fn move_bot(&mut self) {
        for _ in 0..self.bot_step_size() {
            if self.bot_pos == self.player_pos {
                break;
            }
            if let Some(dir) = self.best_bot_direction() {
                if let Some(new_pos) = self.bot_pos.neighbor(dir) {
                    self.bot_pos = new_pos;
                }
            }
        }
    }

    fn best_bot_direction(&self) -> Option<Direction> {
        ALL_DIRS
            .iter()
            .filter_map(|&dir| {
                self.bot_pos
                    .neighbor(dir)
                    .filter(|p| p.x < self.cols && p.y < self.rows)
                    .map(|p| (dir, p.manhattan_distance(self.player_pos)))
            })
            .min_by_key(|&(_, dist)| dist)
            .map(|(dir, _)| dir)
    }

    pub fn advance_round(&mut self) {
        self.round += 1;
        self.reset_positions();
    }

    pub fn restart(&mut self) {
        self.round = 1;
        self.reset_positions();
    }

    fn reset_positions(&mut self) {
        let (player_pos, bot_pos) = Self::spawn_positions(self.cols, self.rows);
        self.player_pos = player_pos;
        self.bot_pos = bot_pos;
        self.moves_survived = 0;
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn bot_moves_toward_player() {
        let mut game = ChaseGame {
            round: 1,
            cols: 10,
            rows: 10,
            player_pos: Position::new(5, 5),
            bot_pos: Position::new(0, 0),
            moves_survived: 0,
        };
        let old_dist = game.bot_pos.manhattan_distance(game.player_pos);
        game.move_bot();
        let new_dist = game.bot_pos.manhattan_distance(game.player_pos);
        assert!(new_dist < old_dist);
    }

    #[test]
    fn bot_step_size_increases() {
        let game1 = ChaseGame::new(10, 10);
        let mut game3 = ChaseGame::new(10, 10);
        game3.round = 3;
        let mut game5 = ChaseGame::new(10, 10);
        game5.round = 5;
        assert!(game1.bot_step_size() < game3.bot_step_size());
        assert!(game3.bot_step_size() < game5.bot_step_size());
    }

    #[test]
    fn caught_when_bot_reaches_player() {
        let mut game = ChaseGame {
            round: 1,
            cols: 10,
            rows: 10,
            player_pos: Position::new(1, 0),
            bot_pos: Position::new(0, 0),
            moves_survived: 0,
        };
        // Move player left into bot
        let result = game.move_player(Direction::Left);
        assert!(matches!(result, ChaseMoveResult::Caught { .. }));
    }

    #[test]
    fn invalid_move_at_boundary() {
        let mut game = ChaseGame {
            round: 1,
            cols: 10,
            rows: 10,
            player_pos: Position::new(0, 0),
            bot_pos: Position::new(9, 9),
            moves_survived: 0,
        };
        let result = game.move_player(Direction::Left);
        assert!(matches!(result, ChaseMoveResult::InvalidMove));
    }

    #[test]
    fn survived_after_enough_moves() {
        let mut game = ChaseGame {
            round: 1,
            cols: 20,
            rows: 20,
            player_pos: Position::new(0, 0),
            bot_pos: Position::new(19, 19),
            moves_survived: 0,
        };
        game.moves_survived = game.moves_to_survive() - 1;
        // Move right (away from bot at 19,19 is not optimal, but valid)
        let result = game.move_player(Direction::Right);
        assert!(
            matches!(result, ChaseMoveResult::Survived { .. })
                || matches!(result, ChaseMoveResult::Caught { .. })
        );
    }
}
