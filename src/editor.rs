use std::env;
use std::time::Duration;
use std::time::Instant;
use termion::event::Key;

use crate::status_line::components::StatusLineFileNameComponent;
use crate::status_line::components::StatusLineLineIndicatorComponent;
use crate::status_line::components::StatusLineModifiedComponent;
use crate::status_line::StatusLineComponent;
use crate::utils;
use crate::Document;
use crate::Position;
use crate::Row;
use crate::StatusLine;
use crate::StatusMessage;
use crate::Terminal;

const VERSION: &str = env!("CARGO_PKG_VERSION");
const QUIT_TIMES: u8 = 3;

pub struct Editor {
    should_quit: bool,
    terminal: Terminal,
    cursor_position: Position,
    document: Document,
    offset: Position,
    status_message: StatusMessage,
    quit_times: u8,
}

// Main logic
impl Editor {
    pub fn run(&mut self) {
        loop {
            if self.should_quit {
                if let Err(error) = self.quit() {
                    die(&error);
                }
                break;
            }
            if let Err(error) = self.refresh_screen() {
                die(&error);
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

        let mut status_message = StatusMessage::from("HELP: Ctrl-S = save | Ctrl-Q = quit");

        let document = if let Some(file_name) = args.get(1) {
            let doc = Document::open(file_name);
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
            quit_times: QUIT_TIMES,
        }
    }
}

// Utilities
impl Editor {
    fn quit(&self) -> Result<(), std::io::Error> {
        Terminal::clear_screen();
        Terminal::cursor_position(&Position::default());
        println!("Goodbye!\r");
        Terminal::flush()
    }

    pub fn refresh_screen(&self) -> Result<(), std::io::Error> {
        Terminal::cursor_hide();
        Terminal::clear_screen();
        Terminal::cursor_position(&Position::default());
        if self.document.is_empty() {
            self.draw_dashboard();
        } else {
            self.draw_rows();
        };
        self.draw_status_bar();
        self.draw_message_bar();
        Terminal::cursor_position(&self.cursor_position.saturating_sub(&self.offset));
        Terminal::cursor_show();
        Terminal::flush()
    }

    fn draw_row(&self, row: &Row) {
        let width = self.terminal.size().width as usize;
        let start = self.offset.x;
        let end = self.offset.x.saturating_add(width);
        let row = row.render(start, end);
        println!("{}\r", row)
    }

    fn draw_welcome_message(&self) {
        let welcome_message = format!("Hecto editor -- version {}", VERSION);
        let welcome_message =
            utils::centered_text(&welcome_message, self.terminal.size().width as usize);
        println!("~{}\r", welcome_message);
    }

    fn draw_dashboard(&self) {
        let height = self.terminal.size().height;
        for terminal_row in 0..height {
            Terminal::clear_current_line();
            if terminal_row == height / 3 {
                self.draw_welcome_message();
            } else {
                println!("~\r");
            }
        }
    }

    fn draw_rows(&self) {
        let height = self.terminal.size().height;
        for terminal_row in 0..height {
            Terminal::clear_current_line();
            if let Some(row) = self
                .document
                .row(self.offset.y.saturating_add(terminal_row as usize))
            {
                self.draw_row(row);
            } else {
                println!("~\r");
            }
        }
    }

    fn draw_status_bar(&self) {
        let components: Vec<StatusLineComponent> = vec![
            StatusLineFileNameComponent::new(self.document.file_name.clone()),
            StatusLineModifiedComponent::new(self.document.is_dirty()),
            StatusLineLineIndicatorComponent::new(self.cursor_position.y, self.document.len()),
        ];

        StatusLine {
            components,
            width: self.terminal.size().width as usize,
            separator: " ".to_string(),
        }
        .draw();
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

    fn quit_confirmation(&mut self) {
        self.status_message.set_status(&format!(
            "WARNING! File has unsaved changes. Press Ctrl-Q {} more times to quit.",
            self.quit_times
        ));
    }

    fn process_keypress(&mut self) -> Result<(), std::io::Error> {
        let pressed_key = Terminal::read_key()?;
        match pressed_key {
            Key::Ctrl('q') => {
                if !self.document.is_dirty() {
                    self.should_quit = true;
                    return Ok(());
                }
                if self.quit_times > 0 {
                    self.quit_confirmation();
                    self.quit_times -= 1;
                    return Ok(());
                }
            }
            Key::Ctrl('s') => self.save(),
            Key::Char(c) => {
                self.document.insert(&self.cursor_position, c);
                self.move_cursor(Key::Right);
            }
            Key::Delete => self.document.delete(&self.cursor_position),
            Key::Backspace => {
                if self.cursor_position.x > 0 || self.cursor_position.y > 0 {
                    self.move_cursor(Key::Left);
                    self.document.delete(&self.cursor_position);
                }
            }
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
        if self.quit_times < QUIT_TIMES {
            self.quit_times = QUIT_TIMES;
            self.status_message.set_status("");
        }
        Ok(())
    }

    fn scroll(&mut self) {
        let Position { x, y } = self.cursor_position;
        let width = self.terminal.size().width as usize;
        let height = self.terminal.size().height as usize;

        let offset = &mut self.offset;
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

    fn prompt(&mut self, prompt: &str) -> Result<Option<String>, std::io::Error> {
        let mut result = String::new();
        loop {
            self.status_message
                .set_status(&format!("{}{}", prompt, result));
            self.refresh_screen()?;
            match Terminal::read_key()? {
                Key::Backspace => result.truncate(result.len().saturating_sub(1)),
                Key::Char('\n') => break,
                Key::Char(c) => {
                    if !c.is_control() {
                        result.push(c);
                    }
                }
                Key::Esc => {
                    result.truncate(0);
                    break;
                }
                _ => (),
            }
        }

        self.status_message.set_status("");
        if result.is_empty() {
            Ok(None)
        } else {
            Ok(Some(result))
        }
    }

    fn save(&mut self) {
        if self.document.file_name.is_none() {
            if let Some(new_name) = self.prompt("Save as: ").unwrap_or(None) {
                self.document.file_name = Some(new_name);
            } else {
                self.status_message.set_status("Save aborted.");
                return;
            }
        }

        self.status_message
            .set_status(if self.document.save().is_ok() {
                "File saved successfully."
            } else {
                "Error writing file!"
            })
    }
}

fn die(e: &std::io::Error) {
    Terminal::clear_screen();
    panic!("{}", e);
}
