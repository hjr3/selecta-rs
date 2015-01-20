
use std::os::unix::prelude::AsRawFd;
use std::io::{File, FileMode, FileAccess, IoResult, standard_error, ResourceUnavailable};
use std::io::stdio::{stdin, StdinReader, stdout_raw, StdWriter};
use std::io::pipe::{PipePair, PipeStream};
use std::io::Command;
use std::io::process::StdioContainer;

use winsize;

pub struct TTY {
    in_file: File,
    out_file: File,
}

impl TTY {
    pub fn new() -> IoResult<TTY> {
        let path = Path::new("/dev/tty");
        let in_file = File::open_mode(&path, FileMode::Open, FileAccess::Write).unwrap();
        let out_file = File::open(&path).unwrap();

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

    pub fn winsize(&mut self) -> IoResult<(u16, u16)> {
        winsize::winsize()
    }

    pub fn stty(&mut self, arg: &str) -> String {
        let path = Path::new("/dev/tty");
        let file = File::open(&path).unwrap();
        let tty_fd = file.as_raw_fd();
        let output = match Command::new("stty").arg(arg).stdin(StdioContainer::InheritFd(tty_fd)).output() {
            Ok(o) => o,
            Err(e) => panic!("failed to execute process: {}", e),
        };

        String::from_utf8(output.output).unwrap()
    }
}
