use anyhow::{Result, ensure};
use crossterm::{
    cursor, queue,
    style::{self, Color, Stylize},
    terminal,
};
use std::io::{Stdout, Write};
use std::{thread, time::Duration};

use crate::models::Position;

const CELL_W: u16 = 4;
const CELL_H: u16 = 2;

const MIN_COLS: u16 = 5;
const MIN_ROWS: u16 = 4;
const MAX_COLS: u16 = 16;
const MAX_ROWS: u16 = 10;

const MARGIN_TOP: u16 = 2;
const MARGIN_BOTTOM: u16 = 4;
const MARGIN_SIDE: u16 = 2;

pub struct Board {
    pub cols: u16,
    pub rows: u16,
    origin_x: u16,
    origin_y: u16,
    term_w: u16,
}

impl Board {
    pub fn from_terminal() -> Result<Self> {
        let (term_w, term_h) = terminal::size()?;
        ensure!(
            term_w >= 30 && term_h >= 15,
            "terminal too small (minimum 30x15, got {term_w}x{term_h})"
        );

        let avail_w = term_w.saturating_sub(MARGIN_SIDE * 2);
        let avail_h = term_h.saturating_sub(MARGIN_TOP + MARGIN_BOTTOM);

        let cols = (avail_w.saturating_sub(1) / (CELL_W + 1)).clamp(MIN_COLS, MAX_COLS);
        let rows = (avail_h.saturating_sub(1) / (CELL_H + 1)).clamp(MIN_ROWS, MAX_ROWS);

        let board_w = cols * (CELL_W + 1) + 1;
        let origin_x = term_w.saturating_sub(board_w) / 2;

        Ok(Self {
            cols,
            rows,
            origin_x,
            origin_y: MARGIN_TOP,
            term_w,
        })
    }

    fn cell_pos(&self, pos: Position) -> (u16, u16) {
        (
            self.origin_x + 1 + pos.x * (CELL_W + 1),
            self.origin_y + 1 + pos.y * (CELL_H + 1),
        )
    }

    fn status_row(&self) -> u16 {
        let board_h = self.rows * (CELL_H + 1) + 1;
        self.origin_y + board_h + 1
    }

    pub fn draw_grid(&self, stdout: &mut Stdout) -> Result<()> {
        for row in 0..=self.rows {
            let mut line = String::new();
            for col in 0..=self.cols {
                line.push(match (row, col) {
                    (0, 0) => '┌',
                    (0, c) if c == self.cols => '┐',
                    (r, 0) if r == self.rows => '└',
                    (r, c) if r == self.rows && c == self.cols => '┘',
                    (0, _) => '┬',
                    (r, _) if r == self.rows => '┴',
                    (_, 0) => '├',
                    (_, c) if c == self.cols => '┤',
                    _ => '┼',
                });
                if col < self.cols {
                    line.push_str(&"─".repeat(CELL_W as usize));
                }
            }

            queue!(
                stdout,
                cursor::MoveTo(self.origin_x, self.origin_y + row * (CELL_H + 1)),
                style::PrintStyledContent(line.with(Color::DarkGrey))
            )?;

            if row < self.rows {
                for h in 0..CELL_H {
                    let mut mid = String::new();
                    for col in 0..=self.cols {
                        mid.push('│');
                        if col < self.cols {
                            mid.push_str(&" ".repeat(CELL_W as usize));
                        }
                    }
                    queue!(
                        stdout,
                        cursor::MoveTo(
                            self.origin_x,
                            self.origin_y + row * (CELL_H + 1) + 1 + h
                        ),
                        style::PrintStyledContent(mid.with(Color::DarkGrey))
                    )?;
                }
            }
        }

        stdout.flush()?;
        Ok(())
    }

    pub fn fill_cell(&self, stdout: &mut Stdout, pos: Position, color: Color) -> Result<()> {
        let (cx, cy) = self.cell_pos(pos);
        let fill = " ".repeat(CELL_W as usize);
        for dy in 0..CELL_H {
            queue!(
                stdout,
                cursor::MoveTo(cx, cy + dy),
                style::PrintStyledContent(fill.clone().on(color))
            )?;
        }
        stdout.flush()?;
        Ok(())
    }

    pub fn clear_cell(&self, stdout: &mut Stdout, pos: Position) -> Result<()> {
        let (cx, cy) = self.cell_pos(pos);
        let fill = " ".repeat(CELL_W as usize);
        for dy in 0..CELL_H {
            queue!(
                stdout,
                cursor::MoveTo(cx, cy + dy),
                style::Print(&fill)
            )?;
        }
        stdout.flush()?;
        Ok(())
    }

    pub fn animate_path(
        &self,
        stdout: &mut Stdout,
        path: &[Position],
        step_ms: u64,
        mut on_step: impl FnMut() -> Result<()>,
    ) -> Result<()> {
        let delay = Duration::from_millis(step_ms);
        for (i, &pos) in path.iter().enumerate() {
            let color = if i == 0 { Color::Cyan } else { Color::Green };
            self.fill_cell(stdout, pos, color)?;
            on_step()?;
            thread::sleep(delay);
        }
        Ok(())
    }

    pub fn clear_path(&self, stdout: &mut Stdout, path: &[Position]) -> Result<()> {
        for &pos in path {
            self.clear_cell(stdout, pos)?;
        }
        Ok(())
    }

    pub fn draw_title(&self, stdout: &mut Stdout, title: &str) -> Result<()> {
        let x = self.term_w.saturating_sub(title.len() as u16) / 2;
        queue!(
            stdout,
            cursor::MoveTo(x, 0),
            style::PrintStyledContent(title.with(Color::Cyan).bold())
        )?;
        stdout.flush()?;
        Ok(())
    }

    pub fn draw_status(&self, stdout: &mut Stdout, text: &str, color: Color) -> Result<()> {
        let row = self.status_row();
        queue!(
            stdout,
            cursor::MoveTo(0, row),
            style::Print(" ".repeat(self.term_w as usize))
        )?;
        let x = self.term_w.saturating_sub(text.len() as u16) / 2;
        queue!(
            stdout,
            cursor::MoveTo(x, row),
            style::PrintStyledContent(text.with(color))
        )?;
        stdout.flush()?;
        Ok(())
    }

    pub fn draw_chase_info(
        &self,
        stdout: &mut Stdout,
        round: u32,
        survived: u32,
        target: u32,
        bot_steps: u32,
    ) -> Result<()> {
        let info = format!(
            "Round {}  |  {}/{}  survived  |  Bot speed: {}  |  Q to quit",
            round, survived, target, bot_steps
        );
        let row = self.status_row() + 1;
        queue!(
            stdout,
            cursor::MoveTo(0, row),
            style::Print(" ".repeat(self.term_w as usize)),
            cursor::MoveTo(self.term_w.saturating_sub(info.len() as u16) / 2, row),
            style::PrintStyledContent(info.with(Color::DarkGrey))
        )?;
        stdout.flush()?;
        Ok(())
    }

    pub fn draw_snake_info(
        &self,
        stdout: &mut Stdout,
        score: u32,
        level: u32,
    ) -> Result<()> {
        let info = format!("Score: {}  |  Level: {}  |  Q to quit", score, level);
        let row = self.status_row() + 1;
        queue!(
            stdout,
            cursor::MoveTo(0, row),
            style::Print(" ".repeat(self.term_w as usize)),
            cursor::MoveTo(self.term_w.saturating_sub(info.len() as u16) / 2, row),
            style::PrintStyledContent(info.with(Color::DarkGrey))
        )?;
        stdout.flush()?;
        Ok(())
    }

    pub fn draw_round_info(&self, stdout: &mut Stdout, round: u32, moves: u32) -> Result<()> {
        let info = format!("Round {}  |  {} moves  |  Q to quit", round, moves);
        let row = self.status_row() + 1;
        queue!(
            stdout,
            cursor::MoveTo(0, row),
            style::Print(" ".repeat(self.term_w as usize)),
            cursor::MoveTo(self.term_w.saturating_sub(info.len() as u16) / 2, row),
            style::PrintStyledContent(info.with(Color::DarkGrey))
        )?;
        stdout.flush()?;
        Ok(())
    }
}
