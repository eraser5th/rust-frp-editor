use core::panic;

use sodium_rust::Cell;
use sodium_rust::CellLoop;
use sodium_rust::Operational;
use sodium_rust::SodiumCtx;
use termion::event::Key;

use super::Keyboard;
use super::Position;
use super::Terminal;

pub struct Editor {
    cursor_position: Cell<Position>,
    should_quit: bool,
    keyboard: Keyboard,
    terminal: Terminal,
}

impl Editor {
    pub fn new(sodium_ctx: &SodiumCtx) -> Self {
        Self {
            cursor_position: Cell::new(&sodium_ctx, Position::default()),
            should_quit: false,
            keyboard: Keyboard::new(&sodium_ctx),
            terminal: Terminal::default().expect("Failed to initialize terminal"),
        }
    }
}

impl Editor {
    pub fn run(&mut self, sodium_ctx: &SodiumCtx) {
        Terminal::clear_screen();
        Terminal::flush().unwrap();

        sodium_ctx.transaction(|| {
            let value: CellLoop<Position> = sodium_ctx.new_cell_loop();
            Operational::updates(&value.cell()).listen(|p: &Position| Terminal::cursor_position(p));

            let update = self.keyboard.key_pressed.snapshot(
                &value.cell(),
                |k: &Key, v: &Position| match k {
                    Key::Char('q') => panic!("q pressed"),
                    Key::Down => v.down(),
                    Key::Up => v.up(),
                    Key::Left => v.left(),
                    Key::Right => v.right(),
                    _ => v.clone(),
                },
            );
            value.loop_(&update.hold(Position::default()));
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
