use crate::models::Direction;
use anyhow::Result;
use crossterm::event::{self, Event, KeyCode, KeyEventKind};

pub enum Input {
    Move(Direction),
    Quit,
}

pub fn read_input() -> Result<Input> {
    loop {
        if let Event::Key(key) = event::read()? {
            if key.kind != KeyEventKind::Press {
                continue;
            }
            let input = match key.code {
                KeyCode::Char('w' | 'W') | KeyCode::Up => Input::Move(Direction::Up),
                KeyCode::Char('a' | 'A') | KeyCode::Left => Input::Move(Direction::Left),
                KeyCode::Char('s' | 'S') | KeyCode::Down => Input::Move(Direction::Down),
                KeyCode::Char('d' | 'D') | KeyCode::Right => Input::Move(Direction::Right),
                KeyCode::Char('q') | KeyCode::Esc => Input::Quit,
                _ => continue,
            };
            return Ok(input);
        }
    }
}

pub enum PostRoundInput {
    PlayAgain,
    Quit,
}

pub fn read_post_round() -> Result<PostRoundInput> {
    loop {
        if let Event::Key(key) = event::read()? {
            if key.kind != KeyEventKind::Press {
                continue;
            }
            match key.code {
                KeyCode::Char('r' | 'R') => return Ok(PostRoundInput::PlayAgain),
                KeyCode::Char('q') | KeyCode::Esc => return Ok(PostRoundInput::Quit),
                _ => continue,
            }
        }
    }
}

pub fn wait_for_any_key() -> Result<()> {
    loop {
        if let Event::Key(key) = event::read()?
            && key.kind == KeyEventKind::Press
        {
            return Ok(());
        }
    }
}
