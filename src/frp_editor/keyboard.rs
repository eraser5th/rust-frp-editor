use sodium_rust::SodiumCtx;
use sodium_rust::Stream;
use sodium_rust::StreamSink;
use termion::event::Key;

use crate::Terminal;

pub struct Keyboard {
    key_pressed_sink: StreamSink<Key>,
    pub key_pressed: Stream<Key>,
}

impl Keyboard {
    pub fn new(sodium_ctx: &SodiumCtx) -> Self {
        let key_pressed_sink = sodium_ctx.new_stream_sink();
        let key_pressed = key_pressed_sink.stream();
        Self {
            key_pressed_sink,
            key_pressed,
        }
    }

    pub fn observe_keypress(&self) -> Result<(), std::io::Error> {
        let pressed_key = Terminal::read_key()?;
        self.key_pressed_sink.send(pressed_key);
        Ok(())
    }
}
