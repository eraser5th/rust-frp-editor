#![warn(clippy::all, clippy::pedantic, clippy::arithmetic_side_effects)]
#![allow(
    clippy::missing_docs_in_private_items,
    clippy::implicit_return,
    clippy::shadow_reuse,
    clippy::print_stdout,
    clippy::wildcard_enum_match_arm,
    clippy::else_if_without_else
)]

use std::sync::Arc;

use crossterm::terminal::{disable_raw_mode, enable_raw_mode};
use sodium_rust::SodiumCtx;

use rust_frp_editor::editor::Editor;

fn main() {
    enable_raw_mode().unwrap();

    let default_panic = std::panic::take_hook();
    std::panic::set_hook(Box::new(move |info| {
        disable_raw_mode().unwrap();
        default_panic(info);
    }));

    let sodium_ctx = Arc::new(SodiumCtx::new());

    Editor::new(sodium_ctx).run().unwrap();
}
