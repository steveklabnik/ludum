extern crate rustbox;

mod console;

use console::Console;
use rustbox::Key;

fn main() {
    let console = Console::new();

    console.clear_screen();

    console.print(0, 0, "Hello, world!");

    loop {
        console.present();

        match console.get_key() {
            Some(Key::Char('q')) => { break; }
            _ => { }
        }
    }
}
