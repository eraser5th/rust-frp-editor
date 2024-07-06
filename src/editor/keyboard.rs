use crossterm::event::read;
use crossterm::event::Event;
use crossterm::event::KeyCode;
use crossterm::event::KeyEvent;
use sodium_rust::SodiumCtx;
use sodium_rust::Stream;
use sodium_rust::StreamSink;

use super::direction::Direction;

pub struct Keyboard {
    ssink_key_pressed: StreamSink<KeyEvent>,
    pub s_key_pressed: Stream<KeyEvent>,
    pub s_arrow_key_pressed: Stream<Direction>,
}

impl Keyboard {
    pub fn new(sodium_ctx: &SodiumCtx) -> Self {
        let ssink_key_pressed = sodium_ctx.new_stream_sink();
        let s_key_pressed = ssink_key_pressed.stream();
        let s_arrow_key_pressed = s_key_pressed.map(|k| to_direction(k)).filter_option();

        Self {
            ssink_key_pressed,
            s_key_pressed,
            s_arrow_key_pressed,
        }
    }

    pub fn observe_keypress(&self) -> () {
        let Ok(event) = read() else {
            panic!("hoge");
        };
        match event {
            Event::Key(key) => self.ssink_key_pressed.send(key),
            _ => (),
        };
    }
}

fn to_direction(k: &KeyEvent) -> Option<Direction> {
    match k.code {
        KeyCode::Up => Some(Direction::Up),
        KeyCode::Down => Some(Direction::Down),
        KeyCode::Left => Some(Direction::Left),
        KeyCode::Right => Some(Direction::Right),
        _ => None,
    }
}
