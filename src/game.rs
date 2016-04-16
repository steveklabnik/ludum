use console::Console;
use rustbox::Key;

pub struct Game {
    rooms: Vec<Room>,
    current_room: usize,
    ending: Option<Ending>,
}

impl Game {
    pub fn make_choice(&mut self, choice: char) {
        choice.to_digit(10).map(|choice| {
            // we choose in 1-indexed, but vectors are 0-indexed
            let choice = choice - 1;

            let current_room = self.current_room;

            let action = self.rooms[current_room]
                             .choices.get(choice as usize)
                             .map(|c| c.action);

            if let Some(action) = action {
                match action {
                    Action::Goto(next) => self.current_room = next,
                    Action::Win => self.ending = Some(Ending::Win),
                }
            }
        });
    }

    pub fn render(&self, console: &Console) {
        let current_room = self.current_room;
        let room = &self.rooms[current_room];

        let choices: Vec<&str> = room.choices.iter()
                                             .map(|c| &c.description[..])
                                             .collect();

        console.print_room(&room.description, &choices);
    }

    pub fn render_ending(&self, console: &Console) {
        console.clear();
        console.print(5, 5, "ITS OVER!");
        console.print(2, 23, "Press 'q' to quit");
        console.present();
        loop {
            match console.get_keypress() {
                Some(Key::Char('q')) => { break; },
                _ => {},
            }
        }
    }

    pub fn is_over(&self) -> bool {
        self.ending.is_some()
    }

    pub fn load() -> Game {
        let description = "This is the first description.

no one can ask me or try to tell me what to Instagram... It's my art... In Roman times the artist would contemplate proportions and colors. Now there is only one important color... Green Tribe changed music forever.

Called I Miss the Old Kanye I love this new A$AP FERG album!!! Wes That’s all it was Kanye
";

        let choices = vec![Choice::new("go to next screen", Action::Goto(1)), Choice::new("leave", Action::Goto(1))];

        let room1 = Room {
            description: description.to_string(),
            choices: choices,
        };

        let description = "This is the second description.

And now I look and look around and there’s so many Kanyes When companies doubt me they doubt us. I have a dream.

I also wanted to point out that it’s the first album to go number 1 off of streaming!!! Utah has eliminated homelessness by 91% and also my number one design rule of anything I do from shoes to music to homes is that Kim has to like it.... Don't be scared of the truth because we need to restart the human foundation in truth

Pablo in blood Don't hide from the truth because it is the only light. I love you. Thank you to everybody who made The Life of Pablo the number 1 album in the world!!! 
";

        let choices = vec![Choice::new("go to previous screen", Action::Goto(0)), Choice::new("win the game", Action::Win)];

        let room2 = Room {
            description: description.to_string(),
            choices: choices,
        };

        Game {
            rooms: vec![room1, room2],
            current_room: 0,
            ending: None,
        }
    }
}

struct Room {
    description: String,
    choices: Vec<Choice>,
}

enum Ending {
    Win,
}

#[derive(Clone,Copy)]
enum Action {
    Goto(usize),
    Win,
}

struct Choice {
    description: String,
    action: Action,
}

impl Choice {
    fn new(description: &str, action: Action) -> Choice {
        Choice {
            description: description.to_string(),
            action: action,
        }
    }
}
