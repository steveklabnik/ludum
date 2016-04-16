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

    pub fn print(&self, col: usize, row: usize, msg: &str) {
        self.rustbox.print(col, row, rustbox::RB_NORMAL, Color::White, Color::Black, msg);
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
