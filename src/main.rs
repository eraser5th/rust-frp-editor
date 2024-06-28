#![warn(clippy::all, clippy::pedantic, clippy::arithmetic_side_effects)]
#![allow(
    clippy::missing_docs_in_private_items,
    clippy::implicit_return,
    clippy::shadow_reuse,
    clippy::print_stdout,
    clippy::wildcard_enum_match_arm,
    clippy::else_if_without_else
)]

use rust_frp_editor::frp_editor::Editor;
use sodium_rust::SodiumCtx;

fn main() {
    let sodium_ctx = SodiumCtx::new();
    Editor::new(&sodium_ctx).run(&sodium_ctx).unwrap();
}
