#![allow(unstable)]

extern crate libc;
extern crate test;

use std::io::stdin;

//mod winsize;
//mod tty;
//mod ansi;
//mod screen;
mod config;
mod score;
mod search;


#[cfg(not(test))]
fn main() {

    //let mut screen = screen::Screen::with_screen();
    //let mut _choices = stdin_readlines();

    let score = score::score("a", "b");
    println!("score = {}", score);

    //let config = config::Configuration::from_inputs(choices, screen.height());

    //s.configure_tty();
    //s.move_cursor(0, 5);
    //s.restore_tty();

    //let width = s.width();
    //let height = s.height();
    //println!("Window size is {} x {}", width, height);

    //let mut tty = match tty::TTY::new() {
    //    Ok(tty) => tty,
    //    Err(r) => {
    //        println!("Failed to open a tty: {}", r);
    //        return;
    //    }
    //};
    //println!("I read in {}", tty.get_char());

    //tty.puts();
}

#[cfg(not(test))]
fn stdin_readlines() -> Vec<String> {
    let mut choices: Vec<String> = Vec::new();

    for line in stdin().lock().lines() {
        choices.push(line.unwrap());
    }

    choices
}
