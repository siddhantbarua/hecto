use crossterm::event::{read, Event::Key, KeyCode::Char};
use crossterm::terminal::{enable_raw_mode, disable_raw_mode};

pub struct Editor {

}

impl Editor {

    pub fn default() -> Self {
        Editor{}
    }

    pub fn run(&self) {
        // terminal starts in canonical/cooked mode by default
        if let Err(err) = self.repl() {
            panic!("{err:#?}")
        }

        println!("Goodbye!");
    }

    fn repl(&self) -> Result<(), std::io::Error> {

        enable_raw_mode()?;     // rep() signature allows us to unwrap without handling Err()

        loop {
            if let Key(event) = read()? {
                println!("{event:?} \r");

                if let Char(c) = event.code{
                    if c == 'q' {
                        break;
                    }
                }
            }
        }

        disable_raw_mode()?;
        Ok(())
    }
}