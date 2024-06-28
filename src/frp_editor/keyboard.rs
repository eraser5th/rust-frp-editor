use std::io;

use sodium_rust::SodiumCtx;
use sodium_rust::Stream;
use sodium_rust::StreamSink;
use termion::event::Key;
use termion::input::TermRead;

use super::direction::Direction;

pub struct Keyboard {
    key_pressed_sink: StreamSink<Key>,
    pub key_pressed: Stream<Key>,
    pub arrow_key_pressed: Stream<Direction>,
}

impl Keyboard {
    pub fn new(sodium_ctx: &SodiumCtx) -> Self {
        let key_pressed_sink = sodium_ctx.new_stream_sink();
        let key_pressed = key_pressed_sink.stream();
        let arrow_key_pressed = key_pressed.map(|k: &Key| to_direction(k)).filter_option();

        Self {
            key_pressed_sink,
            key_pressed,
            arrow_key_pressed,
        }
    }

    pub fn observe_keypress(&self) -> Result<(), std::io::Error> {
        let pressed_key = loop {
            if let Some(key) = io::stdin().lock().keys().next() {
                break key?;
            }
        };
        self.key_pressed_sink.send(pressed_key);
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
