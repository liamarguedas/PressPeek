use std::io::{self, Write};
use crossterm::event::{read, Event, KeyCode};

pub fn listen(clean_limit: i32) -> io::Result<()> {
    
    let current_keys = Vec<char>;

    loop {
        if let Event::Key(event) = read()? {
            if let KeyCode::Char(c) = event.code {
            }
        }
    }
}
