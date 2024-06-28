use core::panic;
use std::io::stdout;
use std::io::Stdout;
use std::process;
use std::sync::Arc;
use std::sync::Mutex;

use sodium_rust::Cell;
use sodium_rust::CellLoop;
use sodium_rust::Operational;
use sodium_rust::SodiumCtx;
use sodium_rust::Stream;
use termion::event::Key;
use termion::raw::RawTerminal;

use super::terminal::Size;
use super::Direction;
use super::Keyboard;
use super::Position;
use super::Terminal;

#[derive(Clone, PartialEq, Eq)]
enum Command {
    NOP,
    Quit,
    Save,
    Undo,
    Redo,
}

pub struct Editor {
    keyboard: Keyboard,
}

impl Editor {
    pub fn new(sodium_ctx: &SodiumCtx, stdout: &Arc<Mutex<RawTerminal<Stdout>>>) -> Self {
        Terminal::clear_screen();

        let keyboard = Keyboard::new(&sodium_ctx);
        let terminal = Terminal::new(&sodium_ctx).expect("Failed to initialize terminal");
        let c_stdout = sodium_ctx.new_cell(stdout.clone());
        let s_quit = command(&keyboard.key_pressed)
            .filter(|c: &Command| *c == Command::Quit)
            .snapshot1(&c_stdout);
        s_quit.listen(|stdout: &Arc<Mutex<RawTerminal<Stdout>>>| Self::quit(stdout));

        sodium_ctx.transaction(|| {
            let cursor_position: CellLoop<Position> = sodium_ctx.new_cell_loop();
            Operational::value(&cursor_position.cell())
                .listen(|p: &Position| Terminal::cursor_position(p));

            let next_position = keyboard
                .arrow_key_pressed
                .snapshot3(
                    &cursor_position.cell(),
                    &terminal.size,
                    |d: &Direction, p: &Position, s: &Size| {
                        let next_position = p.move_to(d);
                        s.is_in(&next_position).then_some(next_position)
                    },
                )
                .filter_option();

            cursor_position.loop_(&next_position.hold(Position::default()));
        });

        Self { keyboard }
    }
}

impl Editor {
    pub fn run(&self) -> Result<(), std::io::Error> {
        loop {
            Terminal::flush()?;
            self.keyboard.observe_keypress()?;
        }
    }

    fn quit(stdout: &Arc<Mutex<RawTerminal<Stdout>>>) {
        Terminal::clear_screen();
        let stdout = stdout.lock().expect("ahoy");
        stdout.suspend_raw_mode().expect("peko");
        println!("Bye!!!\r\n");
        process::exit(0);
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

fn command(key_pressed: &Stream<Key>) -> Stream<Command> {
    key_pressed.map(|k: &Key| match k {
        Key::Ctrl('q') => Command::Quit,
        Key::Ctrl('s') => Command::Save,
        Key::Ctrl('z') => Command::Undo,
        Key::Ctrl('Z') => Command::Redo,
        _ => Command::NOP,
    })
}

impl Editor {}
