mod keyboard;


fn main() {
    let clean_at = 5;
    keyboard::keyboard::listen_keyboard(clean_at);
}
