use console::Console;

pub struct Game {
    rooms: Vec<Room>,
    current_room: usize,
}

impl Game {
    pub fn make_choice(&mut self, choice: char) {
        choice.to_digit(10).map(|choice| {
            // we choose in 1-indexed, but vectors are 0-indexed
            let choice = choice - 1;

            let current_room = self.current_room;

            self.rooms[current_room]
                .choices.get(choice as usize)
                .map(|c| c.1)
                .map(|next| self.current_room = next);
        });
    }

    pub fn render(&self, console: &Console) {
        let current_room = self.current_room;
        let room = &self.rooms[current_room];

        console.clear_screen();
        console.print_description(&room.description);

        let choices: Vec<&str> = room.choices.iter()
                                             .map(|c| (&c.0[..]))
                                             .collect();
        console.print_choices(&choices);

        console.present();
    }

    pub fn load() -> Game {
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

        Game {
            rooms: vec![room1, room2],
            current_room: 0,
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

