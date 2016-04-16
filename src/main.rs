extern crate rustbox;

mod console;

use console::Console;
use rustbox::Key;

fn main() {
    let console = Console::new();

    let rooms = load_rooms();
    let mut current_room = 0;

    loop {
        rooms[current_room].render(&console);

        console.present();

        match console.get_key() {
            Some(Key::Char('q')) => { break; }
            Some(Key::Char(c)) => {
                c.to_digit(10).and_then(|choice| {
                    rooms[current_room].make_choice(choice)
                }).map(|next| {
                    current_room = next;
                });
            }
            _ => { }
        }
    }
}

struct Choice(String, usize);

impl Choice {
    fn new(description: &str, goto: usize) -> Choice {
        Choice(description.to_string(), goto)
    }
}

struct Room {
    description: String,
    choices: Vec<Choice>,
}

impl Room {
    fn render(&self, console: &Console) {
        console.clear_screen();
        console.print_description(&self.description);

        let choices: Vec<&str> = self.choices.iter()
                                              .map(|c| (&c.0[..]))
                                              .collect();
        console.print_choices(&choices);
    }

    fn make_choice(&self, choice: u32) -> Option<usize> {
        self.choices.get((choice - 1) as usize).map(|c| c.1)
    }
}

fn load_rooms() -> Vec<Room> {
    let description = "This is the first description.

no one can ask me or try to tell me what to Instagram... It's my art... In Roman times the artist would contemplate proportions and colors. Now there is only one important color... Green Tribe changed music forever.

Called I Miss the Old Kanye I love this new A$AP FERG album!!! Wes That’s all it was Kanye
";

    let choices = vec![Choice::new("go to next screen", 1), Choice::new("leave", 1)];

    let room1 = Room {
        description: description.to_string(),
        choices: choices,
    };

    let description = "This is the second description.

And now I look and look around and there’s so many Kanyes When companies doubt me they doubt us. I have a dream.

I also wanted to point out that it’s the first album to go number 1 off of streaming!!! Utah has eliminated homelessness by 91% and also my number one design rule of anything I do from shoes to music to homes is that Kim has to like it.... Don't be scared of the truth because we need to restart the human foundation in truth

Pablo in blood Don't hide from the truth because it is the only light. I love you. Thank you to everybody who made The Life of Pablo the number 1 album in the world!!! 
";

    let choices = vec![Choice::new("go to previous screen", 0), Choice::new("leave", 0)];

    let room2 = Room {
        description: description.to_string(),
        choices: choices,
    };

    vec![room1, room2]
}
