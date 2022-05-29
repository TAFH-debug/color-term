use std::ops;

pub enum OptionType {
    Background(Color),
    Style(Style),
    Font(Color),
    Print,
    Random,
    Error
}
pub enum Color {
    Black,
    Red,
    Green,
    Yellow,
    Blue,
    Purple,
    Cyan,
    White
}

pub enum Style {
    Normal,
    Bold,
    Faded,
    Italic,
    Underlined,
    Flashing,
    Strikethrough
}

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
pub struct Option {
    pub flag: String,
    pub value: String
}

impl Option {
    pub fn new(flag: String, value: String) -> Option {
        Self {
            flag,
            value
        }
    }
}