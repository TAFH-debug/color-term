use std::ops;

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
    println!("{}", set_font_background(Color::Black, Color::White, "   |    |--      \\/     |  "));
    println!("{}", set_font_background(Color::Black, Color::White, "   |    ----    / \\     |  "));
    println!();
}

fn main() {
    let args = std::env::args().collect::<Vec<String>>();
    if args.len() == 1 {
        print_logo();
        return;
    }
    if args[1] == "--help" {
        print_logo();
    }
}