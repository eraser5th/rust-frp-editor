use termion::color;

use crate::Terminal;

pub mod components;

const STATUS_FG_COLOR: color::Rgb = color::Rgb(63, 63, 63);
const STATUS_BG_COLOR: color::Rgb = color::Rgb(239, 239, 239);

#[derive(Default)]
pub struct StatusLineComponent {
    pub text: String,
}

impl StatusLineComponent {
    pub fn to_string(&self) -> String {
        self.text.clone()
    }

    pub fn update(&mut self, text: String) {
        self.text = text
    }
}

#[derive(Default)]
pub struct StatusLine {
    pub components: Vec<StatusLineComponent>,
    pub separator: String,
    pub width: usize,
}

impl StatusLine {
    pub fn draw(&self) {
        Terminal::set_bg_color(STATUS_BG_COLOR);
        Terminal::set_fg_color(STATUS_FG_COLOR);
        println!("{}\r", self.to_string());
        Terminal::reset_bg_color();
        Terminal::reset_fg_color();
    }

    pub fn push(&mut self, component: StatusLineComponent) {
        self.components.push(component)
    }

    pub fn set_separator(&mut self, separator: &str) {
        self.separator = separator.to_string()
    }

    pub fn set_width(&mut self, width: usize) {
        self.width = width
    }
}

impl StatusLine {
    fn to_string(&self) -> String {
        let mut status = String::new();

        status.push_str(
            &self
                .components
                .iter()
                .map(|c| c.to_string())
                .collect::<Vec<String>>()
                .join(&self.separator),
        );

        status.push_str(&" ".repeat(self.width));
        status.truncate(self.width);

        status
    }
}
