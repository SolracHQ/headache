use crossterm::event::{Event, KeyCode, KeyEvent, read};
use crossterm::terminal::enable_raw_mode;

pub fn getchar() -> char {
    enable_raw_mode().unwrap();
    loop {
        match read().unwrap() {
            Event::Key(
                KeyEvent {
                    code: KeyCode::Char(char), ..
                }) => return char,
            _ => {}
        };
    }
}