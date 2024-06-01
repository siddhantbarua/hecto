use std::io::stdout;

use crossterm::event::Event;
use crossterm::event::{read, Event::Key, KeyCode::Char, KeyEvent, KeyModifiers};
use crossterm::{execute};
use crossterm::terminal::{disable_raw_mode, enable_raw_mode, Clear, ClearType};

pub struct Editor {
    should_quit: bool,
}

impl Editor {

    pub fn default() -> Self {
        Editor{should_quit: false}
    }

    pub fn run(&mut self) {
        Self::initialize().unwrap();
        Self::draw_rows().unwrap();
        let result = self.repl();
        Self::terminate().unwrap();

        result.unwrap(); // Enables repl() response to panic

    }

    fn initialize() -> Result<(), std::io::Error> {
        // terminal starts in canonical/cooked mode by default
        enable_raw_mode()?;
        Self::clear_screen()?;
        let mut stdout = stdout();

        execute!(stdout, crossterm::cursor::MoveTo(0, 0))
    }

    fn terminate() -> Result<(), std::io::Error> {
        disable_raw_mode()
    }

    fn clear_screen() -> Result<(), std::io::Error> {
        let mut stdout = stdout();
        execute!(stdout, Clear(ClearType::All)) // Write immediately, don't wait for buffer to fill
    }

    fn repl(&mut self) -> Result<(), std::io::Error> {

        loop {
            let event = read()?;
            self.evaluate_event(&event);
            self.refresh_screen()?;
        
            if self.should_quit {
                break;
            }
        }

        Ok(())
    }

    fn refresh_screen(&self) -> Result<(), std::io::Error>{
        if self.should_quit { 
            Self::clear_screen()?;
            print!("Goodbye!\r\n");
        }
        Ok(())
    }

    fn evaluate_event(&mut self, event: &Event){
        if let Key(KeyEvent{
            code,
            modifiers,
            ..
        }) = event {
            match code {
                Char('q') if *modifiers == KeyModifiers::CONTROL => {
                    self.should_quit = true;
                }
                _ => ()
            }
        }
    }

    fn draw_rows() -> Result<(), std::io::Error>{
        let mut stdout = stdout();
        
        let (_, rows) = crossterm::terminal::size()?;

        for i in 0..rows {
            print!("~");
            if i < rows - 1 {
                print!("\n\r");
            }
        }

        execute!(stdout, crossterm::cursor::MoveTo(0, 0))?;

        Ok(())
    }

}