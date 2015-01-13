#![allow(unstable)]

extern crate libc;

mod winsize;
mod tty;
mod ansi;

fn main() {
    let mut tty = match tty::TTY::new() {
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

