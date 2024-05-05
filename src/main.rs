use std::io::{self, Write};
use crossterm::event::{read, Event, KeyCode};

fn main() -> io::Result<()> {
    loop {
        if let Event::Key(event) = read()? {
            if let KeyCode::Char(c) = event.code {
                println!("Key pressed: {}", c);
            }
        }
    }
}
