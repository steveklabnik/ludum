extern crate rustbox;

mod console;
mod game;

use console::Console;
use game::Game;
use rustbox::Key;

fn main() {
    let console = Console::new();

    let mut game = Game::load();

    loop {
        game.render(&console);

        console.present();

        match console.get_key() {
            Some(Key::Char('q')) => { break; }
            Some(Key::Char(c)) => {
                c.to_digit(10).map(|choice| {
                    game.make_choice(choice)
                });
            }
            _ => { }
        }
    }
}
