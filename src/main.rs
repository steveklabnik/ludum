extern crate rustbox;
extern crate toml;

mod console;
mod game;

use console::Console;
use game::Game;
use rustbox::Key;

fn main() {

    let console = Console::new();
    let mut game = Game::load();

    game.render_splash(&console);

    loop {
        match console.get_keypress() {
            Some(_) => { break; },
            _ => {},
        }
    }

    'gameloop: loop {
        game.render(&console);

        match console.get_keypress() {
            Some(Key::Char('q')) => {
                break 'gameloop;
            },
            Some(Key::Char(choice)) => {
                game.make_choice(choice)
            },
            _ => {}
        }

        if game.is_over() {
            game.render_ending(&console);

            loop {
                match console.get_keypress() {
                    Some(_) => { break 'gameloop; },
                    _ => {},
                }
            }
        }
    }
}
