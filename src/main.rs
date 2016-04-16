extern crate rustbox;

mod console;

use console::Console;
use rustbox::Key;

struct Choice(i32, String, i32);

impl Choice {
    fn new(number: i32, description: &str, goto: i32) -> Choice {
        Choice(number, description.to_string(), goto)
    }
}

fn main() {
    let console = Console::new();

    console.clear_screen();

    let description = "This is the first description.

no one can ask me or try to tell me what to Instagram... It's my art... In Roman times the artist would contemplate proportions and colors. Now there is only one important color... Green Tribe changed music forever.

Called I Miss the Old Kanye I love this new A$AP FERG album!!! Wes That’s all it was Kanye
";

    console.print_description(description);

    let choices = vec![Choice::new(1, "go to next screen", 2), Choice::new(2, "leave", 2)];

    print_choices(&console, &choices);

    loop {
        console.present();

        match console.get_key() {
            Some(Key::Char('q')) => { break; }
            _ => { }
        }
    }
}

fn print_choices(console: &Console, choices: &[Choice]) {
    let choices: Vec<(i32, &str)> = choices.into_iter()
                                           .map(|c| (c.0, &c.1[..]))
                                           .collect();
    console.print_choices(&choices);
}
