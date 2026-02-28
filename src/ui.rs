use anyhow::{Result, anyhow};
use crossterm::{
    cursor,
    event::{self, Event, KeyCode, KeyEventKind},
    execute, queue,
    style::{self, Color, Stylize},
    terminal::{
        self, EnterAlternateScreen, LeaveAlternateScreen, disable_raw_mode, enable_raw_mode, size,
    },
};
use std::io::{self, Stdout, Write};

use crate::models::Position;

pub fn draw_board(max_size: (u16, u16), stdout: &mut Stdout, cols: u16, rows: u16) -> Result<()> {
    execute!(stdout, EnterAlternateScreen)?;

    let cell_w = 4;
    let cell_h = 2;

    let board_w = cols * cell_w + cols + 1;
    let board_h = rows * cell_h + rows + 1;

    let start_x = (max_size.0 - board_w) / 2;
    let start_y = (max_size.1 - board_h) / 2;

    for row in 0..=rows {
        // Horizontale Linie
        let mut top_line = String::new();
        for col in 0..=cols {
            top_line.push(match (row, col) {
                (0, 0) => '┌',
                (0, c) if c == cols => '┐',
                (r, 0) if r == rows => '└',
                (r, c) if r == rows && c == cols => '┘',
                (0, _) => '┬',
                (r, _) if r == rows => '┴',
                (_, 0) => '├',
                (_, c) if c == cols => '┤',
                _ => '┼',
            });

            if col < cols {
                top_line.push_str(&"─".repeat(cell_w as usize));
            }
        }

        queue!(
            stdout,
            cursor::MoveTo(start_x, start_y + row * (cell_h + 1))
        )?;
        queue!(
            stdout,
            style::PrintStyledContent(top_line.with(Color::White))
        )?;

        // Vertikale Zellbereiche
        if row < rows {
            for h in 0..cell_h {
                let mut mid_line = String::new();
                for col in 0..=cols {
                    mid_line.push('│');
                    if col < cols {
                        mid_line.push_str(&" ".repeat(cell_w as usize));
                    }
                }

                queue!(
                    stdout,
                    cursor::MoveTo(start_x, start_y + row * (cell_h + 1) + 1 + h)
                )?;
                queue!(stdout, style::Print(mid_line.clone()))?;
            }
        }
    }

    stdout.flush()?;
    Ok(())
}

pub fn draw_path(path: Vec<Position>) {}

pub fn draw_input() {}
