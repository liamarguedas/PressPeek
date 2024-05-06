use crossterm::event::{read, Event, KeyCode};

pub fn listen_keyboard(array_size: usize) {
    let mut current_keys: Vec<char> = Vec::new();

    loop {
        if current_keys.len() == array_size {
            let keys: String = current_keys.iter().collect();
            println!("Array is full: {}", keys);
            current_keys.clear();
        }

        if let Ok(Event::Key(event)) = read() {
            if let KeyCode::Char(c) = event.code {
                current_keys.push(c);
            }
        }
    }
}


