use crossterm::event::{read, Event, Event::Key, KeyCode::Char, KeyEvent, KeyModifiers};

mod terminal;
use terminal::Terminal;

pub struct Editor {
    should_quit: bool,
}

impl Editor {

    pub const fn default() -> Self {
        Self{should_quit: false}
    }

    pub fn run(&mut self) {
        Terminal::initialize().unwrap();
        let result = self.repl();
        Terminal::terminate().unwrap();
        result.unwrap(); // Enables repl() response to panic

    }

    fn repl(&mut self) -> Result<(), std::io::Error> {

        loop {
            self.refresh_screen()?;
            if self.should_quit {
                break;
            }
            let event = read()?;
            self.evaluate_event(&event);
        }

        Ok(())
    }

    fn refresh_screen(&self) -> Result<(), std::io::Error>{
        if self.should_quit { 
            Terminal::clear_screen()?;
            print!("Goodbye!\r\n");
        } else {
            Self::draw_rows()?;
            Terminal::move_cursor_to(0, 0)?;
        }

        Ok(())
    }

    fn evaluate_event(&mut self, event: &Event){
        if let Key(KeyEvent{
            code, modifiers, ..
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
        
        let (_, rows) = Terminal::size()?;

        for i in 0..rows {
            print!("~");
            if i < rows - 1 {
                print!("\r\n");
            }
        }

        Ok(())
    }

}