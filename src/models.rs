#[derive(Clone, Copy, PartialEq, Eq, Debug)]
pub enum Direction {
    Up,
    Down,
    Left,
    Right,
}

#[derive(Clone, Copy, PartialEq, Eq, Debug, Hash)]
pub struct Position {
    pub x: u16,
    pub y: u16,
}

impl Position {
    pub fn new(x: u16, y: u16) -> Self {
        Self { x, y }
    }

    pub fn neighbor(self, direction: Direction) -> Option<Self> {
        match direction {
            Direction::Up => self.y.checked_sub(1).map(|y| Self::new(self.x, y)),
            Direction::Down => Some(Self::new(self.x, self.y + 1)),
            Direction::Left => self.x.checked_sub(1).map(|x| Self::new(x, self.y)),
            Direction::Right => Some(Self::new(self.x + 1, self.y)),
        }
    }

    pub fn manhattan_distance(self, other: Self) -> u32 {
        let dx = (i32::from(self.x) - i32::from(other.x)).unsigned_abs();
        let dy = (i32::from(self.y) - i32::from(other.y)).unsigned_abs();
        dx + dy
    }

    pub fn direction_to(self, other: Self) -> Option<Direction> {
        let dx = i32::from(other.x) - i32::from(self.x);
        let dy = i32::from(other.y) - i32::from(self.y);
        match (dx, dy) {
            (0, -1) => Some(Direction::Up),
            (0, 1) => Some(Direction::Down),
            (-1, 0) => Some(Direction::Left),
            (1, 0) => Some(Direction::Right),
            _ => None,
        }
    }
}
