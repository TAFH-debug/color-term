use std::io::{stdout, Write};
use std::thread;
use std::time::Duration;
use crate::*;

fn goto(x: u32, y: u32) -> String {
    return format!("\x1b[{};{}H", y, x);
}

fn loading_line(count: i32, len: i32) -> String {
    "#".repeat(count as usize) + &*"*".repeat((len - count) as usize)
}

fn animate_loading() {
    print!("\x1b[2J");
    let mut counter = 0;
    let mut len = 10;
    while counter <= len {
        print!("{}", goto(0, 0));
        print!("{}", font(Color::Yellow, "["));
        print!("{}", font(Color::Green, loading_line(counter, len)));
        print!("{}", font(Color::Yellow, "]"));
        counter += 1;
        stdout().flush().expect("TODO: panic message");
        thread::sleep(Duration::from_secs(1));
    }
}

pub fn animate<S: AsRef<str>>(text1: S, text2: S) {
    animate_loading();
}