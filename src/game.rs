use console::Console;
use std::fs::File;
use std::io::prelude::*;
use toml;

pub struct Game {
    rooms: Vec<Room>,
    current_room: usize,
    ending: Option<Ending>,
    player: Player,
}

impl Game {
    pub fn make_choice(&mut self, choice: char) {
        choice.to_digit(10).map(|choice| {
            // we choose in 1-indexed, but vectors are 0-indexed
            let choice = choice - 1;

            let current_room = self.current_room;

            let action = self.rooms[current_room]
                             .choices.get(choice as usize)
                             .map(|c| c.action.clone()); // FIXME: this is suspicious

            if let Some(action) = action {
                match action {
                    Action::Goto(next) => self.current_room = next,
                    Action::Win => self.ending = Some(Ending::Win),
                    Action::Lose => self.ending = Some(Ending::Lose),
                    Action::Aquire(item) => { self.player.aquire(item) },
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
        let items: Vec<&str> = self.player.items.iter()
                                                .map(|i| &i.name[..])
                                                .collect();

        console.print_room(&room.description, &choices, &items);
    }

    pub fn render_ending(&self, console: &Console) {
        if let Some(ending) = self.ending {
            console.clear();

            match ending {
                Ending::Win => console.print(5, 5, "YOU WIN!!!!!!!!!!!!!!!!!!!!!!!!!!1"),
                Ending::Lose => console.print(5, 5, "Sorry, you lose :("),
            }

            console.print(2, 23, "Press 'q' to quit");
            console.present();
        }
    }

    pub fn is_over(&self) -> bool {
        self.ending.is_some()
    }

    pub fn load() -> Game {
        let mut file = File::open("Rooms.toml").unwrap();
        let mut toml = String::new();

        file.read_to_string(&mut toml).unwrap();

        let toml = toml::Parser::new(&toml).parse().unwrap();

        let mut rooms = Vec::new();

        let toml_rooms = toml.get("rooms").unwrap().as_slice().unwrap();
        for room in toml_rooms {
            let room = room.as_table().unwrap();
            let description = room.get("description").unwrap().as_str().unwrap();

            let toml_choices = room.get("choices").unwrap().as_slice().unwrap();
            let mut choices = Vec::new();

            for choice in toml_choices {
                let choice = choice.as_table().unwrap();
                let description = choice.get("description").unwrap().as_str().unwrap();
                let toml_action = choice.get("action").unwrap();
                let action = match toml_action {
                    &toml::Value::String(ref s) => {
                        if s == "win" {
                            Action::Win
                        } else if s == "lose" {
                            Action::Lose
                        } else {
                            Action::Aquire(Item { name: s.clone() })
                        }
                    },
                    &toml::Value::Integer(i) => { Action::Goto(i as usize) },
                    _ => panic!("couldn't parse action"),
                };

                choices.push(Choice::new(description, action));
            }

            let room = Room {
                description: description.to_string(),
                choices: choices,
            };

            rooms.push(room);
        }

        Game {
            rooms: rooms,
            current_room: 0,
            ending: None,
            player: Player {
                items: Vec::new(),
            },
        }
    }
}

struct Room {
    description: String,
    choices: Vec<Choice>,
}

#[derive(Clone,Copy)]
enum Ending {
    Win,
    Lose,
}

#[derive(Clone)]
enum Action {
    Goto(usize),
    Win,
    Lose,
    Aquire(Item),
}

struct Player {
    items: Vec<Item>,
}

impl Player {
    fn aquire(&mut self, item: Item) {
        self.items.push(item)
    }
}

#[derive(Clone)]
struct Item {
    name: String,
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
