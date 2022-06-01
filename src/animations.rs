use std::fs::File;
use std::io::{BufRead, BufReader, stdout, Write};
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

pub fn show_animation(filename: String) {
    let mut file = BufReader::new(File::open(filename).unwrap());
    let mut frames = Vec::new();
    let mut cur_frame = StringBuilder::new();
    let mut reading_frame = false;
    for i in file.lines() {
        if i.as_ref().unwrap() == "#" {
            if reading_frame {
                frames.push(cur_frame.build());
                cur_frame = StringBuilder::new();
                reading_frame = false;
            }
            else {
                reading_frame = true;
            }
            continue;
        }
        if reading_frame {
            cur_frame.append_s(i.unwrap());
        }
    }
    animate(frames);
}

pub fn animate(frames: Vec<String>) -> ! {
    print!("\x1b[2J");
    loop {
        for i in &frames {
            print!("{}", goto(0, 0));
            print!("{}", i);
            stdout().flush();
            thread::sleep(Duration::from_millis(200));
        }
    }
}