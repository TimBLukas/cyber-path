use crate::models::Position;
use rand::seq::IteratorRandom;

pub enum FlappyMoveResult {
    Moved {
        old_player: Position,
        old_pipe_positions: Vec<u16>,
    },
    Collision,
    ScoredPoint {
        old_player: Position,
        old_pipe_positions: Vec<u16>,
    },
}

pub struct FlappyGame {
    pub cols: u16,
    pub rows: u16,
    pub player_y: u16,
    pub player_x: u16,
    pub pipes: Vec<Pipe>,
    pub score: u32,
    pub ticks: u32,
}

pub struct Pipe {
    pub x: u16,
    pub gap_y: u16,
    pub gap_size: u16,
    pub scored: bool,
}

impl FlappyGame {
    pub fn new(cols: u16, rows: u16) -> Self {
        Self {
            cols,
            rows,
            player_y: rows / 2,
            player_x: cols / 4,
            pipes: vec![],
            score: 0,
            ticks: 0,
        }
    }

    pub fn player_pos(&self) -> Position {
        Position::new(self.player_x, self.player_y)
    }

    pub fn tick_ms(&self) -> u64 {
        150u64.saturating_sub(u64::from(self.score) * 5).max(80)
    }

    pub fn jump(&mut self) {
        self.player_y = self.player_y.saturating_sub(2);
    }

    pub fn tick(&mut self) -> FlappyMoveResult {
        let old_player = self.player_pos();
        let old_pipe_positions: Vec<u16> = self.pipes.iter().map(|p| p.x).collect();
        
        self.ticks += 1;

        if self.ticks % 3 == 0 {
            self.player_y = (self.player_y + 1).min(self.rows - 1);
        }

        if self.ticks % 15 == 0 {
            self.spawn_pipe();
        }

        let mut scored = false;
        self.pipes.retain_mut(|pipe| {
            pipe.x = pipe.x.saturating_sub(1);
            if !pipe.scored && self.player_x > pipe.x {
                pipe.scored = true;
                scored = true;
            }
            pipe.x > 0
        });

        if self.check_collision() {
            return FlappyMoveResult::Collision;
        }

        if scored {
            self.score += 1;
            FlappyMoveResult::ScoredPoint {
                old_player,
                old_pipe_positions,
            }
        } else {
            FlappyMoveResult::Moved {
                old_player,
                old_pipe_positions,
            }
        }
    }

    fn spawn_pipe(&mut self) {
        let mut rng = rand::rng();
        let gap_size = 3u16.saturating_sub((self.score / 10) as u16).max(2);
        let gap_y = (gap_size..self.rows.saturating_sub(gap_size))
            .choose(&mut rng)
            .unwrap_or(self.rows / 2);

        self.pipes.push(Pipe {
            x: self.cols - 1,
            gap_y,
            gap_size,
            scored: false,
        });
    }

    fn check_collision(&self) -> bool {
        if self.player_y >= self.rows {
            return true;
        }

        for pipe in &self.pipes {
            if self.player_x >= pipe.x.saturating_sub(1) && self.player_x <= pipe.x {
                let gap_start = pipe.gap_y.saturating_sub(pipe.gap_size / 2);
                let gap_end = pipe.gap_y + pipe.gap_size / 2;
                if self.player_y < gap_start || self.player_y > gap_end {
                    return true;
                }
            }
        }
        false
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
    fn player_starts_in_middle() {
        let game = FlappyGame::new(10, 10);
        assert_eq!(game.player_y, 5);
    }

    #[test]
    fn jump_moves_player_up() {
        let mut game = FlappyGame::new(10, 10);
        let y_before = game.player_y;
        game.jump();
        assert!(game.player_y < y_before);
    }

    #[test]
    fn gravity_pulls_player_down() {
        let mut game = FlappyGame::new(10, 10);
        game.player_y = 5;
        game.tick();
        game.tick();
        game.tick();
        assert_eq!(game.player_y, 6);
    }

    #[test]
    fn collision_at_ceiling() {
        let mut game = FlappyGame::new(10, 10);
        game.player_y = 10;
        let result = game.tick();
        assert!(matches!(result, FlappyMoveResult::Collision));
    }

    #[test]
    fn pipes_spawn_over_time() {
        let mut game = FlappyGame::new(10, 10);
        for _ in 0..15 {
            game.tick();
        }
        assert!(!game.pipes.is_empty());
    }

    #[test]
    fn pipes_move_left() {
        let mut game = FlappyGame::new(10, 10);
        game.pipes.push(Pipe {
            x: 5,
            gap_y: 5,
            gap_size: 3,
            scored: false,
        });
        game.tick();
        assert_eq!(game.pipes[0].x, 4);
    }

    #[test]
    fn score_increases_when_passing_pipe() {
        let mut game = FlappyGame::new(10, 10);
        game.player_x = 7;
        game.pipes.push(Pipe {
            x: 7,
            gap_y: game.player_y,
            gap_size: 3,
            scored: false,
        });
        game.tick();
        assert_eq!(game.score, 1);
    }
}
