#![warn(clippy::all, clippy::pedantic, clippy::arithmetic_side_effects)]
#![allow(
    clippy::missing_docs_in_private_items,
    clippy::implicit_return,
    clippy::shadow_reuse,
    clippy::print_stdout,
    clippy::wildcard_enum_match_arm,
    clippy::else_if_without_else
)]

use std::io::stdout;
use std::sync::Arc;

use rust_frp_editor::frp_editor::Editor;
use sodium_rust::SodiumCtx;
use termion::raw::IntoRawMode;

fn main() {
    let stdout = Arc::new(stdout().into_raw_mode().unwrap());

    let default_panic = std::panic::take_hook();
    let cls_stdout = Arc::clone(&stdout);
    std::panic::set_hook(Box::new(move |info| {
        cls_stdout.suspend_raw_mode().unwrap();
        default_panic(info);
    }));

    let sodium_ctx = Arc::new(SodiumCtx::new());

    Editor::new(sodium_ctx, &stdout).run().unwrap();
}
