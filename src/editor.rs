
use termion::event::Key;

use crate::terminal::Terminal;

const VERSION: &str = env!("CARGO_PKG_VERSION");

pub struct Position {
    pub x: usize,
    pub y: usize
}

pub struct Editor {
    should_quit: bool,
    terminal: Terminal,
    cursor_position: Position,
}
impl Editor {
    pub fn run(&mut self) {
        
        while !self.should_quit {
            if let Err(error) = self.refresh_screen() {
                self.exit_on_error(error);
            }
            if let Err(error) = self.process_keypress() {
                self.exit_on_error(error);
            }
        }

        self.exit();
    }

    fn refresh_screen(&self) -> Result<(), std::io::Error>  {
        Terminal::hide_cursor();
        Terminal::cursor_position(&Position{ x:0, y: 0 });
        
        self.draw_rows();
        
        Terminal::cursor_position(&self.cursor_position);
        Terminal::show_cursor();
        Terminal::flush()

        // Ok(())
    }

    fn draw_rows(&self) {
        let height = self.terminal.size().height;
        for line_number in 0..(height-1) {
            Terminal::clear_current_line();
            if line_number == height / 3 {
                self.draw_welcome_message();
            } else {
                println!("~\r");
            }
        }
    }

    fn draw_welcome_message(&self) {
        let mut welcome_message = format!("Hecto editor -- version {VERSION}");
        let width = self.terminal.size().width as usize;

        let len = welcome_message.len();
        let padding = width.saturating_sub(len) / 2;
        let spaces = " ".repeat(padding.saturating_sub(1));

        welcome_message = format!("~{spaces}{welcome_message}");
        welcome_message.truncate(width);

        println!("{}\r", welcome_message);
    }

    fn process_keypress(&mut self) -> Result<(), std::io::Error> {
        let pressed_key = Terminal::read_key()?;
        match pressed_key {
            Key::Ctrl('q') => self.should_quit = true,
            Key::Up | Key::Down | Key::Left | Key::Right 
              | Key::PageUp | Key::PageDown | Key::Home | Key::End => self.move_cursor(pressed_key),
            _ => (),
        }

        Ok(())
    }

    fn move_cursor(&mut self, key: Key) {
        let Position { mut x, mut y } = self.cursor_position;
        let size = self.terminal.size();
        let height = size.height.saturating_sub(1) as usize;
        let width = size.width.saturating_sub(1) as usize;

        match key {
            Key::Up         => y = y.saturating_sub(1),
            Key::Down       => {
                if y < height {
                    y = y.saturating_add(1);
                }
            },
            Key::Left       => x = x.saturating_sub(1),
            Key::Right      => {
                if x < width {
                    x = x.saturating_add(1);
                }
            },
            Key::PageUp     => y = 0,
            Key::PageDown   => y = height,
            Key::Home       => x = 0,
            Key::End        => x = width,
            _ => {},
        }
        self.cursor_position = Position { x, y };
    }

    fn exit_on_error(&self, error: std::io::Error) {
        print!("{}", termion::clear::All);
        panic!("{error:?}");
    }

    fn exit(&self) {
        Terminal::clear_screen();
        Terminal::cursor_position(&Position { x: 0, y: 0 });
        print!("Salut!");
        Terminal::flush().unwrap();
    }


    pub fn default() -> Self {
        Self { 
            should_quit: false,
            terminal: Terminal::default().expect("Failed to create the terminal"),
            cursor_position: Position { x: 0, y: 0 },
        }
    }
}