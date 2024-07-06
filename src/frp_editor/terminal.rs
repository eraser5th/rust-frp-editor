use crossterm::terminal::size;
use sodium_rust::{Cell, SodiumCtx};

use super::Position;

#[derive(Clone)]
pub struct Size {
    pub width: u16,
    pub height: u16,
}

impl Size {
    pub fn is_in(&self, p: &Position) -> bool {
        p.x < self.width as usize && p.y < self.height as usize
    }
}

pub struct Terminal {
    pub c_size: Cell<Size>,
}

impl Terminal {
    pub fn new(sodium_ctx: &SodiumCtx) -> Result<Self, std::io::Error> {
        let size = size()?;
        Ok(Self {
            c_size: sodium_ctx.new_cell(Size {
                width: size.0,
                height: size.1.saturating_sub(2),
            }),
        })
    }
}
