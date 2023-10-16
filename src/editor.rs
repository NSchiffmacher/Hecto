
use termion::event::Key;

use crate::terminal::Terminal;

pub struct Editor {
    should_quit: bool,
    terminal: Terminal,
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
        Terminal::clear_screen();
        Terminal::cursor_position(0, 0);
        
        self.draw_rows();
        
        Terminal::cursor_position(0, 0);
        Terminal::flush()

        // Ok(())
    }

    fn draw_rows(&self) {
        for _ in 0..self.terminal.size().height {
            println!("~\r");
        }
    }

    fn process_keypress(&mut self) -> Result<(), std::io::Error> {
        let pressed_key = Terminal::read_key()?;
        match pressed_key {
            Key::Ctrl('q') => self.should_quit = true,
            _ => (),
        }

        Ok(())
    }

    fn exit_on_error(&self, error: std::io::Error) {
        print!("{}", termion::clear::All);
        panic!("{error:?}");
    }

    fn exit(&self) {
        Terminal::clear_screen();
        Terminal::cursor_position(0, 0);
        print!("Salut!");
        Terminal::flush().unwrap();
    }


    pub fn default() -> Self {
        Self { 
            should_quit: false,
            terminal: Terminal::default().expect("Failed to create the terminal"),
        }
    }
}