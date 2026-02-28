use crate::models::{Direction, Position};
use anyhow::{Result, anyhow};
use crossterm::{
    cursor,
    event::{self, Event, KeyCode, KeyEventKind},
    execute, queue,
    style::{self, Stylize},
    style::{Color, style},
    terminal::{
        self, EnterAlternateScreen, LeaveAlternateScreen, disable_raw_mode, enable_raw_mode, size,
    },
};
use rand;
use rand::seq::IteratorRandom;
use std::{
    io::{Stdout, Write, stdout},
    time::SystemTime,
};

pub fn create_path(max_size: (i32, i32), length: u32) -> Vec<Position> {
    let start_position = get_random_position(&max_size);
    let mut path = vec![Position::new(start_position.0, start_position.1)];

    for _ in 0..=length {
        let direction = get_random_direction();

        let next_pos = path[path.len()].get_next_position(direction);
        path.push(next_pos);
    }

    path
}

pub fn check_user_path() {}

fn get_random_position(max_size: &(i32, i32)) -> (i32, i32) {
    let mut rng = rand::rng();
    let x = (0..max_size.0).choose(&mut rng).unwrap_or_default();
    let y = (0..max_size.1).choose(&mut rng).unwrap_or_default();

    (x, y)
}

fn get_random_direction() -> Direction {
    let mut rng = rand::rng();
    let num = (0..4).choose(&mut rng).unwrap_or_default();

    match num {
        0 => Direction::Up,
        1 => Direction::Down,
        2 => Direction::Left,
        3 => Direction::Right,
        _ => Direction::Right,
    }
}
