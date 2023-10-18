
use termion::event::Key;

use crate::Terminal;
use crate::Document;
use crate::Row;

const VERSION: &str = env!("CARGO_PKG_VERSION");

#[derive(Default)]
pub struct Position {
    pub x: usize,
    pub y: usize
}

pub struct Editor {
    should_quit: bool,
    terminal: Terminal,
    cursor_position: Position,
    document: Document,
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
        Terminal::cursor_position(&Position::default());
        
        self.draw_rows();
        
        Terminal::cursor_position(&self.cursor_position);
        Terminal::show_cursor();
        Terminal::flush()

        // Ok(())
    }

    fn draw_rows(&self) {
        let height = self.terminal.size().height;
        for terminal_row in 0..height - 1 {
            Terminal::clear_current_line();
            if let Some(row) = self.document.row(terminal_row as usize) {
                self.draw_row(row);
            } else if terminal_row == height / 3 && self.document.is_empty() {
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

        println!("{welcome_message}\r");
    }

    pub fn draw_row(&self, row: &Row) {
        let start = 0;
        let end = self.terminal.size().width as usize;
        let row = row.render(start, end);
        println!("{row}\r");
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
        Terminal::cursor_position(&Position::default());
        print!("Salut!");
        Terminal::flush().unwrap();
    }


    pub fn default() -> Self {
        let args: Vec<_> = std::env::args().collect();
        let document = if args.len() > 1 {
            let filename = &args[1];
            Document::open(&filename).unwrap_or_default()
        } else {
            Document::default()
        };

        Self { 
            should_quit: false,
            terminal: Terminal::default().expect("Failed to create the terminal"),
            document,
            cursor_position: Position::default(),
        }
    }
}