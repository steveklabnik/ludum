use rustbox;
use rustbox::Color;
use rustbox::RustBox;
use rustbox::Key;

pub struct Console {
    rustbox: RustBox,
}

impl Console {
    pub fn new() -> Console {
        let rustbox = match RustBox::init(Default::default()) {
            Result::Ok(v) => v,
            Result::Err(e) => panic!("{}", e),
        };

        Console {
            rustbox: rustbox,                
        }
    }

    pub fn clear_screen(&self) {
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

    // unclear i'm gonna actually use this but i'm lazy so leaving it in for now
    pub fn print(&self, col: usize, row: usize, msg: &str) {
        self.rustbox.print(col, row, rustbox::RB_NORMAL, Color::White, Color::Black, msg);
    }

    fn print_char(&self, col: usize, row: usize, c: char) {
        self.rustbox.print_char(col, row, rustbox::RB_NORMAL, Color::White, Color::Black, c);
    }

    // oh god this is sloppy
    pub fn print_description(&self, description: &str) {
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

    pub fn print_choices(&self, choices: &[(i32, &str)]) {
        let mut line = 18;
        self.print(2, line, "What do you do?");

        line += 2;

        for choice in choices.iter() {
            self.print(2, line, &format!("{})", choice.0));
            self.print(5, line, choice.1);
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
