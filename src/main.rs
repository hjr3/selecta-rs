#![allow(unstable)]

extern crate libc;

use std::io::{IoResult, standard_error, ResourceUnavailable};
use std::io::stdio::{stdin, StdinReader, stdout_raw, StdWriter};

mod winsize;

struct TTY {
    in_file: StdinReader,
    out_file: StdWriter
}

impl TTY {
    fn new() -> IoResult<TTY> {
        let in_file = stdin();
        let out_file = stdout_raw();

        if ! out_file.isatty() {
            return Err(standard_error(ResourceUnavailable));
        }

        Ok(TTY { in_file: in_file, out_file: out_file })
    }

    fn puts(&mut self) {
        let _ = self.out_file.write_line("");
    }

    fn winsize(&mut self) -> IoResult<(isize, isize)> {
        winsize::winsize()
    }

    fn get_char(&mut self) -> char {
        match self.in_file.read_byte() {
            Ok(c) => c as char,
            Err(e) => {
                // log
                ' '
            }
        }
    }
}

fn main() {
    let mut tty = match TTY::new() {
        Ok(tty) => tty,
        Err(r) => {
            println!("Failed to open a tty: {}", r);
            return;
        }
    };
    println!("I read in {}", tty.get_char());

    let (width, height) = tty.winsize().unwrap();
    println!("Window size is {} x {}", width, height);
    tty.puts();
}
