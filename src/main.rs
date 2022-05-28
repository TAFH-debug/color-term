use std::collections::HashMap;
use std::ops;
mod commands;
use commands::*;

type Check<T> = core::option::Option<T>;

enum Color {
    Black,
    Red,
    Green,
    Yellow,
    Blue,
    Purple,
    Cyan,
    White
}

static ENDING: &'static str = "\x1b[0m";

pub struct StringBuilder {
    vec: Vec<char>
}

impl StringBuilder {
    pub fn new() -> StringBuilder {
        Self {
            vec: Vec::new()
        }
    }

    pub fn from(vec: Vec<char>) -> StringBuilder {
        Self {
            vec
        }
    }

    pub fn add<S: AsRef<str>>(&mut self, string: S) {
        self.vec.append(&mut string.as_ref().chars().collect::<Vec<char>>());
    }

    pub fn append(&mut self, c: char) {
        self.vec.push(c);
    }

    pub fn build(&mut self) -> String {
        self.vec.clone().into_iter().collect()
    }
}

impl ops::Add<StringBuilder> for StringBuilder {
    type Output = StringBuilder;

    fn add(self, mut rhs: StringBuilder) -> Self::Output {
        let mut temp = self.vec.clone();
        temp.append(&mut rhs.vec);
        StringBuilder::from(temp)
    }
}

#[derive(Debug)]
struct Option {
    flag: String,
    value: String
}

impl Option {
    pub fn new(flag: String, value: String) -> Option {
        Self {
            flag,
            value
        }
    }
}

fn get_prefix(color: Color, is_background: bool) -> String {
    let mut temp = match color {
        Color::Black => 0,
        Color::Red => 1,
        Color::Green => 2,
        Color::Yellow => 3,
        Color::Blue => 4,
        Color::Purple => 5,
        Color::Cyan => 6,
        Color::White => 7
    };
    if is_background { temp += 40 }
    else { temp += 30 }
    "\x1b[".to_owned() + &*temp.to_string() + "m"
}

fn background<S: AsRef<str>>(color: Color, text: S) -> String {
    let prefix = get_prefix(color, true);
    prefix + text.as_ref() + ENDING
}

fn font<S: AsRef<str>>(color: Color, text: S) -> String {
    let prefix = get_prefix(color, false);
    prefix + text.as_ref() + ENDING
}

fn set_font_background<S: AsRef<str>>(fontc: Color, back: Color, text: S) -> String {
    font(fontc, background(back, text))
}

/**
Replaces **sym** with a **color**
*/
fn paint_sym<S: AsRef<str>>(src: S, color: Color, sym: char) -> String {
    let mut res = StringBuilder::new();
    let prefix = get_prefix(color, true);
    for i in src.as_ref().chars() {
        if i == sym {
            res.add(prefix.clone());
            res.append(' ');
            res.add(ENDING);
            continue;
        }
        res.append(i);
    }
    res.build()
}

fn print_logo() {
    println!();
    println!("{}", set_font_background(Color::Black, Color::White, " -----  ----   \\   /  -----"));
    println!("{}", set_font_background(Color::Black, Color::White, "   |    |--     \\ /     |  "));
    println!("{}", set_font_background(Color::Black, Color::White, "   |    ----    / \\     |  "));
    println!();
}

fn print_help() {
    print_logo();
    println!("Author: TAFH-debug");
    println!("Beautiful text formatting utility. \nUsage: ");
    println!("      textf [<options>] <text>");
    println!("Options: ");
    println!("{}", "        --help - Shows this text.\n".to_owned() +
        "        -f, --font <color> - Set texts font color.\n" +
        "        -b, --background <color> - Set texts background color.\n" +
        "        -r, --random - Generate and show random image.\n" +
        "        -p, --print <color | style> - Prints info.\n");

}

fn get_color(text: String) -> Check<Color> {
    match &*text {
        "black" => Some(Color::Black),
        "red" => Some(Color::Red),
        "green" => Some(Color::Green),
        "yellow" => Some(Color::Yellow),
        "blue" => Some(Color::Blue),
        "purple" => Some(Color::Purple),
        "cyan" => Some(Color::Cyan),
        "white" => Some(Color::White),
        _ => None
    }
}

fn main() {
    let args = std::env::args().collect::<Vec<String>>();
    let mut options = HashMap::new();

    //Soon
    options.insert("-f", "");
    options.insert("-b", "");
    options.insert("-r", "");
    options.insert("--font", "");
    options.insert("--background", "");
    options.insert("--random", "");

    if args.len() == 1 {
        print_help();
        return;
    }
    if args[1] == "--help" {
        print_help();
        return;
    }
    let mut is_option = false;
    let mut uoptions = Vec::new();
    let mut prev_flag = String::new();
    let mut text = String::new();
    for i in args {
        if i.starts_with("-") {
            if !options.contains_key(i.as_str()) {
                println!("{}", font(Color::Red, "Error: invalid flag"));
                return;
            }
            if prev_flag != "" {
                uoptions.push(Option::new(prev_flag, "".to_string()));
            }
            prev_flag = i;
            is_option = true;
            continue;
        }
        if is_option {
            uoptions.push(Option::new(prev_flag.clone(), i));
            prev_flag = "".to_string();
            is_option = false;
            continue;
        }
        text = i;
    }
    let mut backgroundc = Color::Black;
    let mut fontc = Color::White;
    for i in uoptions {
        if i.flag == "-r" {
            random();
            continue;
        }
        else if i.flag == "-b" {
            backgroundc = match get_color(i.value) {
                Some(n) => n,
                None => {
                    println!("{}", font(Color::Red, "Error: this color is not supported."));
                    return;
                },
            }
        }
        else if i.flag == "-f" {
            fontc = match get_color(i.value) {
                Some(n) => n,
                None => {
                    println!("{}", font(Color::Red, "Error: this color is not supported."));
                    return;
                },
            }
        }
        else if i.flag == "-p" {
            match &*i.value {
                "color" => {/*TODO*/},
                "style" => {/*TODO*/},
                _ => println!("{}", font(Color::Red, "Error: undefined info type")),
            }
        }
    }
    println!("{}", set_font_background(fontc, backgroundc, text));
}