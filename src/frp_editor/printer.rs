use std::io::{self, Write};

use termion::color;

use super::position::Position;

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
