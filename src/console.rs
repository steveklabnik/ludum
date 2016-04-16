use rustbox;
use rustbox::Color;
use rustbox::RustBox;
use rustbox::Key;

pub struct Console {
    rustbox: RustBox,
}

impl Console {
    pub fn new() -> Console {
        let rustbox = RustBox::init(Default::default()).unwrap();

        Console {
            rustbox: rustbox,
        }
    }

    pub fn print_room(&self, description: &str, choices: &[&str], items: &[&str]) {
        self.clear();
        self.print_text(description);
        self.print_choices(choices);
        self.print_items(items);
        self.present();
    }

    pub fn get_keypress(&self) -> Option<Key> {
        match self.rustbox.poll_event(false) {
            Ok(rustbox::Event::KeyEvent(key)) => {
                Some(key)
            },
            Err(e) => panic!("{}", e),
            _ => { None }
        }
    }

    // oh god this is sloppy
    pub fn print_text(&self, description: &str) {
        let mut line = 2;
        let mut count = 2;

        for c in description.chars() {
            if c == '\n' {
                line += 1;
                count = 2;
                continue;
            }

            self.print_char(count, line, c);

            count += 1;

            if count > 78 {
                line += 1;
                count = 2;
            }
        }
    }

    fn print_choices(&self, choices: &[&str]) {
        let mut line = 18;

        self.print(2, line, "What do you do?");

        line += 2;

        for (i, choice) in choices.iter().enumerate() {
            // we choose in 1-indexed, but vectors are 0-indexed
            let i = i + 1;

            self.print(2, line, &format!("{})", i));
            self.print(5, line, choice);

            line += 1;
        }
    }

    fn print_items(&self, items: &[&str]) {
        let mut line = 18;

        self.print(42, line, "Items");

        line += 2;

        for (i, item) in items.iter().enumerate() {
            // we choose in 1-indexed, but vectors are 0-indexed
            let i = i + 1;

            self.print(42, line, &format!("{})", i));
            self.print(45, line, item);

            line += 1;
        }
    }

    pub fn clear(&self) {
        // yeah i could make this happen in one loop if i cared but i don't
        // i also redraw too much
        // it's 25x80 graphics, efficiency doesn't matter

        // clear
        for row in 0..26 {
            for col in 0..81 {
                self.rustbox.print_char(col, row, rustbox::RB_NORMAL, Color::White, Color::Black, ' ');
            }
        }

        // top and bottom
        for col in 0..81 {
            self.rustbox.print_char(col, 0, rustbox::RB_BOLD, Color::White, Color::Black, '*');
            self.rustbox.print_char(col, 25, rustbox::RB_BOLD, Color::White, Color::Black, '*');
        }

        // left and right
        for row in 0..26 {
            self.rustbox.print_char(0, row, rustbox::RB_BOLD, Color::White, Color::Black, '*');
            self.rustbox.print_char(80, row, rustbox::RB_BOLD, Color::White, Color::Black, '*');
        }

    }


    pub fn print(&self, col: usize, row: usize, msg: &str) {
        self.rustbox.print(col, row, rustbox::RB_NORMAL, Color::White, Color::Black, msg);
    }

    fn print_char(&self, col: usize, row: usize, c: char) {
        self.rustbox.print_char(col, row, rustbox::RB_NORMAL, Color::White, Color::Black, c);
    }
    
    pub fn present(&self) {
        self.rustbox.present();
    }
}
