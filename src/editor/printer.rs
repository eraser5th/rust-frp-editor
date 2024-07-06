use std::io::{self, Write};

use crossterm::cursor;
use crossterm::style;
use crossterm::style::Color;
use crossterm::terminal::Clear;
use crossterm::terminal::ClearType;

use super::position::Position;

pub fn clear_screen() {
    print!("{}", Clear(ClearType::All));
}

pub fn cursor_position(position: &Position) {
    let Position { x, y } = position;
    let x = x.saturating_add(1) as u16;
    let y = y.saturating_add(1) as u16;
    print!("{}", cursor::MoveTo(x, y));
}

pub fn flush() -> Result<(), std::io::Error> {
    io::stdout().flush()
}

pub fn cursor_hide() {
    print!("{}", cursor::Hide);
}

pub fn cursor_show() {
    print!("{}", cursor::Show);
}

pub fn clear_current_line() {
    print!("{}", Clear(ClearType::CurrentLine));
}

pub fn set_bg_color(color: Color) {
    print!("{}", style::SetBackgroundColor(color));
}

pub fn reset_bg_color() {
    print!("{}", style::SetBackgroundColor(Color::Reset));
}

pub fn set_fg_color(color: Color) {
    print!("{}", style::SetForegroundColor(color));
}

pub fn reset_fg_color() {
    print!("{}", style::SetForegroundColor(Color::Reset));
}
