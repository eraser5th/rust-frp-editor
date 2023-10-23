pub struct Position {
    pub x: usize,
    pub y: usize,
}

impl Position {
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
}
