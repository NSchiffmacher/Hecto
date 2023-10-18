
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
    offset: Position,
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
            offset: Position::default(),
            cursor_position: Position::default(),
        }
    }

    fn refresh_screen(&self) -> Result<(), std::io::Error>  {
        Terminal::hide_cursor();
        Terminal::cursor_position(&Position::default());
        
        self.draw_rows();
        
        Terminal::cursor_position(&Position {
            x: self.cursor_position.x.saturating_sub(self.offset.x),
            y: self.cursor_position.y.saturating_sub(self.offset.y),
        });
        Terminal::show_cursor();
        Terminal::flush()

        // Ok(())
    }

    fn draw_rows(&self) {
        let height = self.terminal.size().height;
        for terminal_row in 0..height - 1 {
            Terminal::clear_current_line();
            if let Some(row) = self.document.row(terminal_row as usize + self.offset.y) {
                self.draw_row(row);
            } else if terminal_row == height / 3 && self.document.is_empty() {
                self.draw_welcome_message();
            } else {
                println!("~\r");
            }
        }
    }

    fn draw_row(&self, row: &Row) {
        let width = self.terminal.size().width as usize;

        let start = self.offset.x;
        let end = self.offset.x + width;

        let row = row.render(start, end);
        println!("{row}\r");
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

    fn process_keypress(&mut self) -> Result<(), std::io::Error> {
        let pressed_key = Terminal::read_key()?;
        match pressed_key {
            Key::Ctrl('q') => self.should_quit = true,
            Key::Up | Key::Down | Key::Left | Key::Right 
              | Key::PageUp | Key::PageDown | Key::Home | Key::End => self.move_cursor(pressed_key),
            _ => (),
        }

        self.scroll();
        Ok(())
    }

    fn scroll(&mut self) {
        let Position { x, y } = self.cursor_position;
        let width = self.terminal.size().width as usize;
        let height = self.terminal.size().height as usize;
        let offset = &mut self.offset;

        if y < offset.y {
            offset.y = y;
        } else if y >= offset.y.saturating_add(height) {
            offset.y = y.saturating_sub(height).saturating_add(1);
        }
        
        if x < offset.x {
            offset.x = x;
        } else if x >= offset.x.saturating_add(width) {
            offset.x = x.saturating_sub(width).saturating_add(1);
        }
    }

    fn move_cursor(&mut self, key: Key) {
        let Position { mut x, mut y } = self.cursor_position;
        let height = self.document.len();
        let mut width = if let Some(row) = self.document.row(y) {
            row.len()
        } else {
            0
        };

        match key {
            Key::Up         => y = y.saturating_sub(1),
            Key::Down       => {
                if y < height {
                    y = y.saturating_add(1);
                }
            },
            Key::Left       => {
                if x > 0 {
                    x -= 1;
                } else if y > 0 {
                    y -= 1;
                    if let Some(row) = self.document.row(y) {
                        x = row.len();
                    } else {
                        x = 0;
                    }
                }
            },
            Key::Right      => {
                if x < width {
                    x += 1;
                } else if y < height {
                    y += 1;
                    x = 0;
                }
            },
            Key::PageUp     => y = y.saturating_sub(self.terminal.size().height as usize),
            Key::PageDown   => y = std::cmp::min(y.saturating_add(self.terminal.size().height as usize), height),
            Key::Home       => x = 0,
            Key::End        => x = width,
            _ => {},
        }

        width = if let Some(row) = self.document.row(y) {
            row.len()
        } else {
            0
        };
        if x > width {
            x = width;
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
}