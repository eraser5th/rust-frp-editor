use std::env;
use std::time::Duration;
use std::time::Instant;
use termion::color;
use termion::event::Key;

use crate::Document;
use crate::Position;
use crate::Row;
use crate::Terminal;

const STATUS_FG_COLOR: color::Rgb = color::Rgb(63, 63, 63);
const STATUS_BG_COLOR: color::Rgb = color::Rgb(239, 239, 239);
const VERSION: &str = env!("CARGO_PKG_VERSION");

struct StatusMessage {
    text: String,
    time: Instant,
}

impl StatusMessage {
    fn from(message: &str) -> Self {
        Self {
            time: Instant::now(),
            text: message.to_string(),
        }
    }

    pub fn set_status(&mut self, message: &str) {
        self.text = message.to_string();
    }
}

pub struct Editor {
    should_quit: bool,
    terminal: Terminal,
    cursor_position: Position,
    document: Document,
    offset: Position,
    status_message: StatusMessage,
}

// Main logic
impl Editor {
    pub fn run(&mut self) {
        loop {
            if let Err(error) = self.refresh_screen() {
                die(&error);
            }
            if self.should_quit {
                break;
            }
            if let Err(error) = self.process_keypress() {
                die(&error);
            }
        }
    }
}

// Default
impl Editor {
    pub fn default() -> Self {
        let args: Vec<String> = env::args().collect();

        let mut status_message = StatusMessage::from("HELP: Ctrl-Q = quit");

        let document = if args.len() > 1 {
            let file_name = &args[1];
            let doc = Document::open(&file_name);
            if let Ok(doc) = doc {
                doc
            } else {
                status_message.set_status(&format!("ERR: Could not open file: {}", file_name));
                Document::default()
            }
        } else {
            Document::default()
        };

        Self {
            should_quit: false,
            terminal: Terminal::default().expect("Failed to initialize terminal"),
            cursor_position: Position::default(),
            document,
            offset: Position::default(),
            status_message,
        }
    }
}

// Utilities
impl Editor {
    fn refresh_screen(&self) -> Result<(), std::io::Error> {
        Terminal::cursor_hide();
        Terminal::clear_screen();
        Terminal::cursor_position(&Position::default());
        if self.should_quit {
            Terminal::clear_screen();
            println!("Goodbye!\r");
        } else {
            self.draw_rows();
            self.draw_status_bar();
            self.draw_message_bar();
            Terminal::cursor_position(&Position {
                x: self.cursor_position.x.saturating_sub(self.offset.x),
                y: self.cursor_position.y.saturating_sub(self.offset.y),
            });
        }
        Terminal::cursor_show();
        Terminal::flush()
    }

    fn draw_welcome_message(&self) {
        let mut welcome_message = format!("Hecto editor -- version {}", VERSION);
        let width = self.terminal.size().width as usize;
        let len = welcome_message.len();
        let padding = width.saturating_sub(len) / 2;
        let spaces = " ".repeat(padding.saturating_sub(1));
        welcome_message = format!("~{}{}", spaces, welcome_message);
        welcome_message.truncate(width);
        println!("{}\r", welcome_message);
    }

    fn draw_row(&self, row: &Row) {
        let width = self.terminal.size().width as usize;
        let start = self.offset.x;
        let end = self.offset.x + width;
        let row = row.render(start, end);
        println!("{}\r", row)
    }

    fn draw_rows(&self) {
        let height = self.terminal.size().height;
        for terminal_row in 0..height {
            Terminal::clear_current_line();
            if let Some(row) = self.document.row(terminal_row as usize + self.offset.y) {
                self.draw_row(row);
            } else if self.document.is_empty() && terminal_row == height / 3 {
                self.draw_welcome_message();
            } else {
                println!("~\r");
            }
        }
    }

    fn draw_status_bar(&self) {
        let width = self.terminal.size().width as usize;

        let file_name = if let Some(name) = &self.document.file_name {
            let mut name = name.clone();
            name.truncate(20);
            name
        } else {
            "[No Name]".to_string()
        };

        let line_indicator = format!(
            "{}/{}",
            self.cursor_position.y.saturating_add(1),
            self.document.len()
        );

        let mut status = format!("{} - {}{}", file_name, line_indicator, &" ".repeat(width));
        status.truncate(width);

        Terminal::set_bg_color(STATUS_BG_COLOR);
        Terminal::set_fg_color(STATUS_FG_COLOR);
        println!("{}\r", status);
        Terminal::reset_bg_color();
        Terminal::reset_fg_color();
    }

    fn draw_message_bar(&self) {
        Terminal::clear_current_line();
        let message = &self.status_message;
        if Instant::now() - message.time < Duration::new(5, 0) {
            let mut text = message.text.clone();
            text.truncate(self.terminal.size().width as usize);
            print!("{}", text);
        }
    }

    fn process_keypress(&mut self) -> Result<(), std::io::Error> {
        let pressed_key = Terminal::read_key()?;
        match pressed_key {
            Key::Ctrl('q') => self.should_quit = true,
            Key::Up
            | Key::Down
            | Key::Left
            | Key::Right
            | Key::PageUp
            | Key::PageDown
            | Key::End
            | Key::Home => self.move_cursor(pressed_key),
            _ => (),
        }
        self.scroll();
        Ok(())
    }

    fn scroll(&mut self) {
        let Position { x, y } = self.cursor_position;
        let width = self.terminal.size().width as usize;
        let height = self.terminal.size().height as usize;

        let mut offset = &mut self.offset;
        if y < offset.y {
            offset.y = y;
        } else if y >= offset.y.saturating_add(height) {
            offset.y = y.saturating_sub(height).saturating_add(1);
        }
        if x < offset.x {
            offset.x = x;
        } else if x >= offset.x.saturating_add(width) {
            offset.x = x.saturating_sub(width).saturating_add(1);
        }
    }

    fn move_cursor(&mut self, key: Key) {
        let Position { x, y } = self.cursor_position;
        let height = self.document.len();
        let mut width = if let Some(row) = self.document.row(y) {
            row.len()
        } else {
            0
        };

        self.cursor_position = match key {
            Key::Up => self.cursor_position.up(),
            Key::Down => {
                if y < height {
                    self.cursor_position.down()
                } else {
                    Position { x, y }
                }
            }
            Key::Left => {
                if x > 0 {
                    self.cursor_position.left()
                } else if y > 0 {
                    if let Some(row) = self.document.row(y - 1) {
                        Position {
                            x: row.len(),
                            y: self.cursor_position.up().y,
                        }
                    } else {
                        Position {
                            x: 0,
                            y: self.cursor_position.up().y,
                        }
                    }
                } else {
                    Position { x, y }
                }
            }
            Key::Right => {
                if x < width {
                    self.cursor_position.right()
                } else if y < height {
                    Position { x: 0, y: y + 1 }
                } else {
                    Position { x, y }
                }
            }
            Key::PageUp => Position { x, y: 0 },
            Key::PageDown => Position { x, y: height },
            Key::Home => Position { x: 0, y },
            Key::End => Position { x: width, y },
            _ => Position { x, y },
        };

        width = if let Some(row) = self.document.row(self.cursor_position.y) {
            row.len()
        } else {
            0
        };
        if x > width {
            self.cursor_position.x = width;
        }
    }
}

fn die(e: &std::io::Error) {
    Terminal::clear_screen();
    panic!("{}", e);
}
