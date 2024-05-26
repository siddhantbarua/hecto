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
        enable_raw_mode().unwrap(); // unwrap uses the Ok value. Can cause panic.

        loop {
            match read() {
                Ok(Key(event)) => {
                    println!("{event:?} \r");

                    if let Char(c) = event.code{
                        if c == 'q' {
                            break;
                        }
                    }
                },
                Err(err) => println!("Error: {err}"),
                _ => ()
            };
        };
        disable_raw_mode().unwrap();
    }
}