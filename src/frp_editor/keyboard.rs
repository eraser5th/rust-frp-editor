use std::io;

use sodium_rust::SodiumCtx;
use sodium_rust::Stream;
use sodium_rust::StreamSink;
use termion::event::Key;
use termion::input::TermRead;

use super::direction::Direction;

pub struct Keyboard {
    ssink_key_pressed: StreamSink<Key>,
    pub s_key_pressed: Stream<Key>,
    pub s_arrow_key_pressed: Stream<Direction>,
}

impl Keyboard {
    pub fn new(sodium_ctx: &SodiumCtx) -> Self {
        let ssink_key_pressed = sodium_ctx.new_stream_sink();
        let s_key_pressed = ssink_key_pressed.stream();
        let s_arrow_key_pressed = s_key_pressed.map(|k: &Key| to_direction(k)).filter_option();

        Self {
            ssink_key_pressed,
            s_key_pressed,
            s_arrow_key_pressed,
        }
    }

    pub fn observe_keypress(&self) -> Result<(), std::io::Error> {
        let pressed_key = loop {
            if let Some(key) = io::stdin().lock().keys().next() {
                break key?;
            }
        };
        self.ssink_key_pressed.send(pressed_key);
        Ok(())
    }
}

fn to_direction(k: &Key) -> Option<Direction> {
    match k {
        Key::Up => Some(Direction::Up),
        Key::Down => Some(Direction::Down),
        Key::Left => Some(Direction::Left),
        Key::Right => Some(Direction::Right),
        _ => None,
    }
}
