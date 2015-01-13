
use std::io::{IoResult, standard_error, ResourceUnavailable};
use std::io::stdio::{stdin, StdinReader, stdout_raw, StdWriter};

use winsize;

pub struct TTY {
    in_file: StdinReader,
    out_file: StdWriter
}

impl TTY {
    pub fn new() -> IoResult<TTY> {
        let in_file = stdin();
        let out_file = stdout_raw();

        if ! out_file.isatty() {
            return Err(standard_error(ResourceUnavailable));
        }

        Ok(TTY { in_file: in_file, out_file: out_file })
    }

    pub fn get_char(&mut self) -> char {
        match self.in_file.read_byte() {
            Ok(c) => c as char,
            Err(e) => {
                // log
                ' '
            }
        }
    }

    pub fn puts(&mut self) {
        let _ = self.out_file.write_line("");
    }

    pub fn winsize(&mut self) -> IoResult<(isize, isize)> {
        winsize::winsize()
    }

    pub fn stty() {
        // TODO implement a way to pipe to /dev/tty
    }
}
