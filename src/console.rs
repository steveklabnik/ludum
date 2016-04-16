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

    fn clear(&self) {
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

    fn print(&self, col: usize, row: usize, msg: &str) {
        self.rustbox.print(col, row, rustbox::RB_NORMAL, Color::White, Color::Black, msg);
    }

    fn print_char(&self, col: usize, row: usize, c: char) {
        self.rustbox.print_char(col, row, rustbox::RB_NORMAL, Color::White, Color::Black, c);
    }

    pub fn print_room(&self, description: &str, choices: &[&str]) {
        self.clear();
        self.print_description(description);
        self.print_choices(choices);
        self.present();
    }

    // oh god this is sloppy
    fn print_description(&self, description: &str) {
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
            self.print(2, line, &format!("{})", i + 1));
            self.print(5, line, choice);
            line += 1;
        }
    }
    
    pub fn present(&self) {
        self.rustbox.present();
    }

    pub fn get_key(&self) -> Option<Key> {
        match self.rustbox.poll_event(false) {
            Ok(rustbox::Event::KeyEvent(key)) => {
                Some(key)
            },
            Err(e) => panic!("{}", e),
            _ => { None }
        }
    }
}
