use std::fmt;

pub enum Direction {
    Up,
    Down,
    Left,
    Right,
}

#[derive(Default, Clone)]
pub struct Position {
    pub x: usize,
    pub y: usize,
}

impl fmt::Display for Position {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "({}, {})", self.x, self.y)
    }
}

impl Position {
    pub fn move_to(&self, d: &Direction) -> Self {
        match d {
            Direction::Down => self.down(),
            Direction::Up => self.up(),
            Direction::Left => self.left(),
            Direction::Right => self.right(),
        }
    }

    pub fn up(&self) -> Self {
        Self {
            x: self.x,
            y: self.y.saturating_sub(1),
        }
    }

    pub fn down(&self) -> Self {
        Self {
            x: self.x,
            y: self.y.saturating_add(1),
        }
    }

    pub fn left(&self) -> Self {
        Self {
            x: self.x.saturating_sub(1),
            y: self.y,
        }
    }

    pub fn right(&self) -> Self {
        Self {
            x: self.x.saturating_add(1),
            y: self.y,
        }
    }

    pub fn saturating_sub(&self, r: &Self) -> Self {
        Self {
            x: self.x.saturating_sub(r.x),
            y: self.y.saturating_sub(r.y),
        }
    }

    pub fn saturating_add(&self, r: &Self) -> Self {
        Self {
            x: self.x.saturating_add(r.x),
            y: self.y.saturating_add(r.y),
        }
    }
}
