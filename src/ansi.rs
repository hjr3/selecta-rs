use std::io::stdio::StdWriter;

enum ANSIColor {
    black,
    red,
    green,
    yellow,
    blue,
    magenta,
    cyan,
    white,
    default
}

fn escape(sequence: &str) -> String {
    let ESC: char = ::std::char::from_u32(27).unwrap();
    format!("{}[{}", ESC, sequence)
}

fn clear() -> String {
    escape("23")
}

fn hide_cursor() -> String {
    escape("?25l")
}

fn show_cursor() -> String {
    escape("?25h")
}

fn setpos(line: u16, column: u16) -> String {
    let seq = format!("{};{}H", (line + 1), (column + 1));
    escape(seq.as_slice())
}

fn color(fg: ANSIColor, bg: ANSIColor) -> String {
    let fg_code = match fg {
        ANSIColor::black => 30,
        ANSIColor::red => 31,
        ANSIColor::green => 32,
        ANSIColor::yellow => 33,
        ANSIColor::blue => 34,
        ANSIColor::magenta => 35,
        ANSIColor::cyan => 36,
        ANSIColor::white => 37,
        ANSIColor::default => 39,
    };

    let bg_code = match bg {
        ANSIColor::black => 40,
        ANSIColor::red => 41,
        ANSIColor::green => 42,
        ANSIColor::yellow => 44,
        ANSIColor::blue => 44,
        ANSIColor::magenta => 45,
        ANSIColor::cyan => 46,
        ANSIColor::white => 47,
        ANSIColor::default => 49,
    };

    let seq = format!("{};{}m", fg_code, bg_code);
    escape(seq.as_slice())
}

fn inverse() -> String {
    escape("7m")
}

fn reset() -> String {
    escape("0m")
}


pub struct ANSI {
    file: StdWriter
}

impl ANSI {
    pub fn new(file: StdWriter) -> ANSI {
        ANSI { file: file }
    }

    pub fn clear(&mut self) {
        self.write(clear());
    }

    pub fn hide_cursor(&mut self) {
        self.write(hide_cursor());
    }

    pub fn show_cursor(&mut self) {
        self.write(show_cursor());
    }

    pub fn setpos(&mut self, line: u16, column: u16) {
        self.write(setpos(line, column));
    }

    pub fn color(&mut self, fg: ANSIColor, bg: ANSIColor) {
        self.write(color(fg, bg));
    }

    pub fn inverse(&mut self) {
        self.write(inverse());
    }

    pub fn reset(&mut self) {
        self.write(reset());
    }

    pub fn write(&mut self, s: String) {
        self.file.write_str(s.as_slice());
    }
}
