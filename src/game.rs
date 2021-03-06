use console::Console;
use std::fs::File;
use std::io::prelude::*;
use toml;

pub struct Game {
    rooms: Vec<Room>,
    current_room: usize,
    ending: Option<Ending>,
    player: Player,
    splash: String,
    win: String,
    lose: String,
}

impl Game {
    pub fn make_choice(&mut self, choice: char) {
        choice.to_digit(10).map(|choice| {
            // we choose in 1-indexed, but vectors are 0-indexed
            let choice_num = choice as usize - 1;

            let current_room = self.current_room;

            let mut okay = false;
            if let Some(choice) = self.rooms[current_room].choices.get(choice_num) {
                okay = if choice.requires.is_some() {
                    let item = Item { name: choice.requires.clone().unwrap() };
                    self.player.items.contains(&item)
                } else {
                    true
                };
            }

            if okay {
                let action = self.rooms[current_room]
                                 .choices
                                 .get(choice_num)
                                 .map(|c| c.action.clone()); // FIXME: this is suspicious

                if let Some(action) = action {
                    match action {
                        Action::Goto(next) => self.current_room = next,
                        Action::Win => self.ending = Some(Ending::Win),
                        Action::Lose => self.ending = Some(Ending::Lose),
                        Action::Aquire(item) => {
                            self.player.aquire(item);
                            self.rooms[current_room].choices.remove(choice_num);
                        }
                    }
                }
            }
        });
    }

    pub fn render(&self, console: &Console) {
        let current_room = self.current_room;
        let room = &self.rooms[current_room];

        let choices: Vec<&str> = room.choices
                                     .iter()
                                     .filter(|c| {
                                         if c.requires.is_some() {
                                             let item = Item { name: c.requires.clone().unwrap() };
                                             self.player.items.contains(&item)
                                         } else {
                                             true
                                         }
                                     })
                                     .map(|c| &c.description[..])
                                     .collect();
        let items: Vec<&str> = self.player
                                   .items
                                   .iter()
                                   .map(|i| &i.name[..])
                                   .collect();

        console.print_room(&room.description, &choices, &items);
    }

    pub fn render_ending(&self, console: &Console) {
        if let Some(ending) = self.ending {
            console.clear();

            match ending {
                Ending::Win => console.print_text(&self.win),
                Ending::Lose => console.print_text(&self.lose),
            }

            console.present();
        }
    }

    pub fn render_splash(&self, console: &Console) {
        console.clear();
        console.print_text(&self.splash);
        console.present();
    }

    pub fn is_over(&self) -> bool {
        self.ending.is_some()
    }

    pub fn load() -> Game {
        let mut file = File::open("Game.toml").unwrap();
        let mut toml = String::new();

        file.read_to_string(&mut toml).unwrap();

        let toml = toml::Parser::new(&toml).parse().unwrap();

        let mut rooms = Vec::new();
        let game_data = toml.get("game").unwrap();
        let game_data = game_data.as_table().unwrap();
        let splash = game_data.get("splash").unwrap().as_str().unwrap().to_string();
        let win = game_data.get("win").unwrap().as_str().unwrap().to_string();
        let lose = game_data.get("lose").unwrap().as_str().unwrap().to_string();

        let toml_rooms = toml.get("rooms").unwrap().as_slice().unwrap();
        for room in toml_rooms {
            let room = room.as_table().unwrap();
            let description = room.get("description").unwrap().as_str().unwrap();

            let toml_choices = room.get("choices").unwrap().as_slice().unwrap();
            let mut choices = Vec::new();

            for choice in toml_choices {
                let toml_choice = choice.as_table().unwrap();
                let description = toml_choice.get("description").unwrap().as_str().unwrap();
                let toml_action = toml_choice.get("action").unwrap();
                let action = match toml_action {
                    &toml::Value::String(ref s) => {
                        if s == "win" {
                            Action::Win
                        } else if s == "lose" {
                            Action::Lose
                        } else {
                            Action::Aquire(Item { name: s.clone() })
                        }
                    }
                    &toml::Value::Integer(i) => Action::Goto(i as usize),
                    _ => panic!("couldn't parse action"),
                };

                let choice = Choice::new(description, action);

                if let Some(item) = toml_choice.get("requires") {
                    let item = item.as_str().unwrap().to_string();
                    choices.push(choice.requires(item));
                } else {
                    choices.push(choice);
                }
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
            player: Player { items: Vec::new() },
            splash: splash,
            win: win,
            lose: lose,
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

#[derive(Clone,PartialEq)]
struct Item {
    name: String,
}

struct Choice {
    description: String,
    action: Action,
    requires: Option<String>,
}

impl Choice {
    fn new(description: &str, action: Action) -> Choice {
        Choice {
            description: description.to_string(),
            action: action,
            requires: None,
        }
    }

    fn requires(mut self, item: String) -> Choice {
        self.requires = Some(item);
        self
    }
}
