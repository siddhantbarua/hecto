use crossterm::event::{read, Event::Key, KeyCode::Char, KeyEvent, KeyModifiers};
use crossterm::terminal::{enable_raw_mode, disable_raw_mode};

pub struct Editor {
    should_quit: bool
}

impl Editor {

    pub fn default() -> Self {
        Editor{should_quit: false}
    }

    pub fn run(&mut self) {
        // terminal starts in canonical/cooked mode by default
        if let Err(err) = self.repl() {
            panic!("{err:#?}")
        }

        println!("Goodbye!");
    }

    fn repl(&mut self) -> Result<(), std::io::Error> {

        enable_raw_mode()?;     // rep() signature allows us to unwrap without handling Err()

        loop {
            if let Key(KeyEvent {
                code, modifiers, kind, state
            }) = read()? {
                println!("Code: {code:?}, Modifiers: {modifiers:?}, Kind: {kind:?}, State: {state:?} \r");

                match code {
                    Char('q') if modifiers == KeyModifiers::CONTROL => {
                        self.should_quit = true;
                    },
                    _ => ()
                }
            }

            if self.should_quit {
                break;
            }
        }

        disable_raw_mode()?;
        Ok(())
    }
}