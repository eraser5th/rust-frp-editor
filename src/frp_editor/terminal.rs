use sodium_rust::{Cell, SodiumCtx};
use std::io::{self, Write};
use termion::color;

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
        let size = termion::terminal_size()?;
        Ok(Self {
            c_size: sodium_ctx.new_cell(Size {
                width: size.0,
                height: size.1.saturating_sub(2),
            }),
        })
    }

    pub fn clear_screen() {
        print!("{}", termion::clear::All);
    }

    pub fn cursor_position(position: &Position) {
        let Position { x, y } = position;
        let x = x.saturating_add(1) as u16;
        let y = y.saturating_add(1) as u16;
        print!("{}", termion::cursor::Goto(x, y));
    }

    pub fn flush() -> Result<(), std::io::Error> {
        io::stdout().flush()
    }

    pub fn cursor_hide() {
        print!("{}", termion::cursor::Hide);
    }

    pub fn cursor_show() {
        print!("{}", termion::cursor::Show);
    }

    pub fn clear_current_line() {
        print!("{}", termion::clear::CurrentLine);
    }

    pub fn set_bg_color(color: color::Rgb) {
        print!("{}", color::Bg(color));
    }

    pub fn reset_bg_color() {
        print!("{}", color::Bg(color::Reset));
    }

    pub fn set_fg_color(color: color::Rgb) {
        print!("{}", color::Fg(color));
    }

    pub fn reset_fg_color() {
        print!("{}", color::Fg(color::Reset));
    }
}
