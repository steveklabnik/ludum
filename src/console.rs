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
        for row in 0..25 {
            for col in 0..80 {
                self.rustbox.print_char(col, row, rustbox::RB_NORMAL, Color::White, Color::Black, ' ');
            }
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
