
use std::time::Instant;
use std::time::Duration;

use termion::event::Key;
use termion::color;

use crate::Terminal;
use crate::Document;
use crate::Row;

const VERSION: &str = env!("CARGO_PKG_VERSION");
const STATUS_BG_COLOR: color::Rgb = color::Rgb(239, 239, 239);
const STATUS_FG_COLOR: color::Rgb = color::Rgb(63, 63, 63);

#[derive(Default)]
pub struct Position {
    pub x: usize,
    pub y: usize
}

struct StatusMessage {
    text: String,
    timestamp: Instant,
}

impl From<String> for StatusMessage {
    fn from(value: String) -> Self {
        Self {
            text: value,
            timestamp: Instant::now(),
        }
    }
}

pub struct Editor {
    should_quit: bool,
    terminal: Terminal,
    cursor_position: Position,
    offset: Position,
    document: Document,
    status_message: StatusMessage,
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
        let mut initial_status = "HELP: Ctrl-S = save | Ctrl-Q = quit".to_string();

        let document = if args.len() > 1 {
            let filename = &args[1];
            if let Ok(document) = Document::open(&filename) {
                document
            } else {
                initial_status = format!("ERR: Could not open file: {filename}");
                Document::default()
            }
        } else {
            Document::default()
        };

        Self { 
            should_quit: false,
            terminal: Terminal::default().expect("Failed to create the terminal"),
            document,
            offset: Position::default(),
            cursor_position: Position::default(),
            status_message: StatusMessage::from(initial_status),
        }
    }

    fn refresh_screen(&self) -> Result<(), std::io::Error>  {
        Terminal::hide_cursor();
        Terminal::cursor_position(&Position::default());
        
        self.draw_rows();
        self.draw_status_bar();
        self.draw_message_bar();
        
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
        for terminal_row in 0..height {
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

    fn draw_status_bar(&self) {
        let filename = if let Some(filename) = &self.document.filename {
            let mut filename = filename.clone();
            filename.truncate(20);
            filename
        } else {
            "[No name]".to_string()
        };

        let len = self.document.len();
        let status = format!("{filename} - {len} lines");
        let line_indicator = format!("{}/{}", self.cursor_position.y.saturating_add(1), len);
        let spaces = " ".repeat(self.terminal.size().width as usize - status.len() - line_indicator.len());

        Terminal::set_bg_color(STATUS_BG_COLOR);
        Terminal::set_fg_color(STATUS_FG_COLOR);
        println!("{status}{spaces}{line_indicator}\r");
        Terminal::reset_bg_color();
        Terminal::reset_fg_color();
    }

    fn draw_message_bar(&self) {
        Terminal::clear_current_line();

        let message = &self.status_message;
        if Instant::now() - message.timestamp < Duration::new(5, 0) {
            let mut text = message.text.clone();
            text.truncate(self.terminal.size().width as usize);
            print!("{text}\r");
        }
    }

    fn process_keypress(&mut self) -> Result<(), std::io::Error> {
        let pressed_key = Terminal::read_key()?;
        match pressed_key {
            Key::Ctrl('q') => self.should_quit = true,
            Key::Ctrl('s') => {
                if self.document.save().is_ok() {
                    self.status_message = StatusMessage::from("File saved successfully.".to_string());
                } else {
                    self.status_message = StatusMessage::from("Error writting file!".to_string());
                }
            },
            Key::Up | Key::Down | Key::Left | Key::Right 
            | Key::PageUp | Key::PageDown | Key::Home | Key::End => self.move_cursor(pressed_key),
            
            Key::Delete => self.document.delete(&self.cursor_position),
            Key::Backspace => {
                if self.cursor_position.x > 0 || self.cursor_position.y > 0{
                    self.move_cursor(Key::Left);
                    self.document.delete(&self.cursor_position);
                }
            }
            Key::Char(c) => {
                self.document.insert(&self.cursor_position, c);
                self.move_cursor(Key::Right);
            },
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
            Key::Up         => {
                if y == 0 {
                    x = 0;
                } else {
                    y = y.saturating_sub(1)
                }
            },
            Key::Down       => {
                if y < height.saturating_sub(1) {
                    y = y.saturating_add(1);
                } else if let Some(row) = self.document.row(y) {
                    x = row.len();
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
                } else if y < height.saturating_sub(1) {
                    y += 1;
                    x = 0;
                }
            },
            Key::PageUp     => y = y.saturating_sub(self.terminal.size().height as usize),
            Key::PageDown   => y = std::cmp::min(y.saturating_add(self.terminal.size().height as usize), height.saturating_sub(1)),
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