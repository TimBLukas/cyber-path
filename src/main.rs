mod game;
mod input;
mod models;
mod ui;

use anyhow::{Result, anyhow};
pub use crossterm::{cursor, terminal::size};
use std::{
    io::{Stdout, Write, stdout},
    time::SystemTime,
};

fn main() {
    let max_size = size().unwrap_or((100, 100));
    let mut stdout = stdout();

    let _ = ui::draw_board(max_size, &mut stdout, 14, 12);
}
