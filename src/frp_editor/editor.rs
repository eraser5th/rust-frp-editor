use std::io::Stdout;
use std::process;
use std::sync::Arc;

use sodium_rust::Cell;
use sodium_rust::Operational;
use sodium_rust::SodiumCtx;
use sodium_rust::Stream;
use termion::event::Key;
use termion::raw::RawTerminal;

use super::printer;
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
    c_cursor_position: Cell<Position>,
    s_quit: Stream<()>,
    stdout: Arc<RawTerminal<Stdout>>,
}

// Not FRP
impl Editor {
    /**
     * Run application.
     */
    pub fn run(&self) -> Result<(), std::io::Error> {
        printer::clear_screen();
        printer::cursor_position(&Position::default());

        Operational::updates(&self.c_cursor_position).listen(printer::cursor_position);
        let stdout = self.stdout.clone();
        self.s_quit.listen(move |_: &()| Self::quit(stdout.clone()));

        loop {
            printer::flush()?;
            self.keyboard.observe_keypress()?;
        }
    }

    fn quit(stdout: Arc<RawTerminal<Stdout>>) -> ! {
        printer::clear_screen();
        printer::cursor_position(&Position::default());
        printer::flush().unwrap();

        stdout.suspend_raw_mode().unwrap();

        println!("Bye!!!\r\n");

        process::exit(0);
    }
}

// FRP
impl Editor {
    /**
     * Build the whole of FRP Network of application
     */
    pub fn new(sodium_ctx: &SodiumCtx, stdout: &Arc<RawTerminal<Stdout>>) -> Self {
        let keyboard = Keyboard::new(&sodium_ctx);
        let terminal = Terminal::new(&sodium_ctx).expect("Failed to initialize terminal");
        let s_quit = command(&keyboard.s_key_pressed)
            .filter(|c: &Command| *c == Command::Quit)
            .map(|_: &Command| ());

        let c_cursor_position = cursor_position(&keyboard.s_arrow_key_pressed, &terminal.c_size);

        Self {
            keyboard,
            c_cursor_position,
            s_quit,
            stdout: stdout.clone(),
        }
    }
}

fn command(s_key_pressed: &Stream<Key>) -> Stream<Command> {
    s_key_pressed.map(|k: &Key| match k {
        Key::Ctrl('q') => Command::Quit,
        Key::Ctrl('s') => Command::Save,
        Key::Ctrl('z') => Command::Undo,
        Key::Ctrl('Z') => Command::Redo,
        _ => Command::NOP,
    })
}

fn cursor_position(
    s_arrow_key_pressed: &Stream<Direction>,
    c_terminal_size: &Cell<Size>,
) -> Cell<Position> {
    s_arrow_key_pressed
        .snapshot(c_terminal_size, |d: &Direction, s: &Size| {
            (d.clone(), s.clone())
        })
        .accum(
            Position::default(),
            |(d, s): &(Direction, Size), current_p: &Position| {
                let next = current_p.move_to(d);
                if s.is_in(&next) {
                    next
                } else {
                    current_p.clone()
                }
            },
        )
}
