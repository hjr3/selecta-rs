
use tty::TTY;
use ansi::ANSI;

use std::io::stdio::stdout_raw;

pub struct Screen {
    tty: TTY,
    ansi: ANSI,
    original_tty_state: String,
}

impl Screen {
    pub fn with_screen() -> Screen {
        let tty = TTY::new().unwrap();
        let ansi = ANSI::new(stdout_raw());
        Screen::new(tty, ansi)
    }

    pub fn new(tty: TTY, ansi: ANSI) -> Screen {

        // TODO this is bonkers, but ok
        let mut s = Screen { tty: tty, ansi: ansi, original_tty_state: String::from_str("") };
        let tty_state = s.tty.stty("-g");
        s.original_tty_state = tty_state;
        s
    }

    pub fn configure_tty(&mut self) {
        // -echo: terminal doesn't echo typed characters back to the terminal
        //# -icanon: terminal doesn't interpret special characters (like backspace)
        self.tty.stty("-echo -icanon");
    }

    pub fn restore_tty(&mut self) {
        let arg = format!("{}", self.original_tty_state);
        self.tty.stty(arg.as_slice());
    }

    pub fn move_cursor(&mut self, line: u16, column: u16) {
        self.ansi.setpos(line, column);
    }

    pub fn height(&mut self) -> u16 {
        let (_, height) = self.size();
        height
    }

    pub fn width(&mut self) -> u16 {
        let (width, _) = self.size();
        width
    }
    
    fn size(&mut self) -> (u16, u16) {
        self.tty.winsize().unwrap()
    }
}
