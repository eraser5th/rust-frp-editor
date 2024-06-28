use core::panic;
use std::sync::Arc;

use sodium_rust::Cell;
use sodium_rust::CellLoop;
use sodium_rust::Operational;
use sodium_rust::SodiumCtx;
use termion::event::Key;

use super::terminal::Size;
use super::Direction;
use super::Keyboard;
use super::Position;
use super::Terminal;

pub struct Editor {
    cursor_position: Arc<Cell<Position>>,
    should_quit: bool,
    keyboard: Arc<Keyboard>,
    terminal: Arc<Terminal>,
}

impl Editor {
    pub fn new(sodium_ctx: &SodiumCtx) -> Self {
        Self {
            cursor_position: Arc::new(sodium_ctx.new_cell(Position::default())),
            should_quit: false,
            keyboard: Arc::new(Keyboard::new(&sodium_ctx)),
            terminal: Arc::new(Terminal::new(sodium_ctx).expect("Failed to initialize terminal")),
        }
    }
}

impl Editor {
    pub fn run(&self, sodium_ctx: &SodiumCtx) {
        Terminal::clear_screen();
        Terminal::cursor_position(&Position::default());
        Terminal::flush().unwrap();

        sodium_ctx.transaction(|| {
            let cursor_position: CellLoop<Position> = sodium_ctx.new_cell_loop();
            Operational::updates(&cursor_position.cell())
                .listen(|p: &Position| Terminal::cursor_position(p));

            let next_position = self
                .keyboard
                .arrow_key_pressed
                .snapshot(&cursor_position.cell(), |d: &Direction, p: &Position| {
                    p.move_to(d)
                });

            let update = next_position
                .snapshot(&self.terminal.size, |p: &Position, s: &Size| {
                    s.is_in(p).then(|| p.clone())
                })
                .filter_option();

            cursor_position.loop_(&update.hold(Position::default()));
        });

        self.keyboard.key_pressed.listen(|k: &Key| match k {
            Key::Char('q') => panic!("q pressed"),
            _ => (),
        });

        loop {
            Terminal::flush().unwrap();
            self.keyboard.observe_keypress().unwrap();
        }
    }

    /*
    fn process_key(&self, key: &Key) {
        match key {
            Key::Char('q') => panic!("q pressed"),
            Key::Char(c) => println!("{c}\r"),
            Key::Up | Key::Right | Key::Down | Key::Left => self.move_cursor(key),
            _ => (),
        };
    }
    */
}

impl Editor {}
