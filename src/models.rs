use anyhow::Result;

#[derive(Clone, Copy, PartialEq, Debug)]
pub enum Direction {
    Up,
    Down,
    Left,
    Right,
}

#[derive(Clone, Copy, PartialEq, Debug)]
pub struct Position {
    pub x: i32,
    pub y: i32,
}

impl Position {
    pub fn new(x: i32, y: i32) -> Self {
        Position { x, y }
    }

    pub fn move_horizontal(&mut self, amount: i32) -> Result<()> {
        if (self.x + amount) > 0 {
            self.x += amount;
        }

        Ok(())
    }

    pub fn move_vertical(&mut self, amount: i32) -> Result<()> {
        if (self.y + amount) > 0 {
            self.y += amount;
        }

        Ok(())
    }

    pub fn get_next_position(&self, direction: Direction) -> Self {
        match direction {
            Direction::Up => Position::new(self.x, self.y + 1),
            Direction::Down => Position::new(self.x, self.y - 1),
            Direction::Left => Position::new(self.x - 1, self.y),
            Direction::Right => Position::new(self.x + 1, self.y),
        }
    }
}

pub enum Field {
    Filled,
    Empty,
}
