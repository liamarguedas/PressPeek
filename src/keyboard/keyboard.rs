use crossterm::event::{read, Event, KeyCode};

pub fn listen_keyboard(array_size: usize) -> String {
    let mut current_keys: Vec<char> = Vec::new();

     loop {
        if let Ok(Event::Key(event)) = read() {
            if let KeyCode::Char(c) = event.code {
                current_keys.push(c);
                if current_keys.len() == array_size {
                   current_keys.clear(); 
                } else {
                    let keys: String = current_keys.iter().collect();
                    return keys;
                }
            }
        }
    }
}
