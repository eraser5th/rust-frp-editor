use std::io::Stdout;
use std::process;
use std::sync::Arc;

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
    c_cursor_position: Cell<Position>,
    s_quit: Stream<Arc<RawTerminal<Stdout>>>,
}

// Not FRP
impl Editor {
    /**
     * Observe keyboard and run application
     */
    pub fn run(&self) -> Result<(), std::io::Error> {
        Terminal::clear_screen();
        Terminal::cursor_position(&Position::default());

        Operational::updates(&self.c_cursor_position).listen(Terminal::cursor_position);
        self.s_quit
            .listen(|stdout: &Arc<RawTerminal<Stdout>>| Self::quit(stdout));

        loop {
            Terminal::flush()?;
            self.keyboard.observe_keypress()?;
        }
    }

    fn quit(stdout: &Arc<RawTerminal<Stdout>>) {
        Terminal::clear_screen();
        Terminal::cursor_position(&Position::default());
        Terminal::flush().unwrap();

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
        let c_stdout = sodium_ctx.new_cell(stdout.clone());
        let s_quit = command(&keyboard.key_pressed)
            .filter(|c: &Command| *c == Command::Quit)
            .snapshot1(&c_stdout);

        let c_cursor_position =
            cursor_position(sodium_ctx, &keyboard.arrow_key_pressed, &terminal.size);

        Self {
            keyboard,
            c_cursor_position,
            s_quit,
        }
    }
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

fn cursor_position(
    sodium_ctx: &SodiumCtx,
    arrow_key_pressed: &Stream<Direction>,
    terminal_size: &Cell<Size>,
) -> Cell<Position> {
    sodium_ctx.transaction(|| {
        let cl_cursor_position: CellLoop<Position> = sodium_ctx.new_cell_loop();

        let next_position = arrow_key_pressed
            .snapshot3(
                &cl_cursor_position.cell(),
                &terminal_size,
                |d: &Direction, p: &Position, s: &Size| {
                    let next_position = p.move_to(d);
                    s.is_in(&next_position).then_some(next_position)
                },
            )
            .filter_option();

        cl_cursor_position.loop_(&next_position.hold(Position::default()));
        cl_cursor_position.cell()
    })
}

impl Editor {}
