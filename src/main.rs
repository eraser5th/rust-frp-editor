#![warn(clippy::all, clippy::pedantic)]
mod editor;
mod position;
mod terminal;

use editor::Editor;
pub use position::Position;
pub use terminal::Terminal;

fn main() {
    Editor::default().run();
}
