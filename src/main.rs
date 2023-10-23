#![warn(clippy::all, clippy::pedantic)]
mod document;
mod editor;
mod position;
mod row;
mod terminal;

pub use document::Document;
use editor::Editor;
pub use position::Position;
pub use row::Row;
pub use terminal::Terminal;

fn main() {
    Editor::default().run();
}
