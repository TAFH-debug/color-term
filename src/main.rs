extern crate core;

mod structs;
mod animations;
mod console;
mod png;

use std::collections::HashMap;
use structs::*;
use console::*;
use crate::animations::show_animation;

type Check<T> = core::option::Option<T>;


fn font_background<S: AsRef<str>>(fontc: Color, back: Color, text: S) -> String {
    font(fontc, background(back, text))
}

fn style_font_background<S: AsRef<str>>(stylet: Style, fontc: Color, back: Color, text: S) -> String {
    style(stylet, font(fontc, background(back, text)))
}

fn print_help() {
    println!("Author: TAFH-debug");
    println!("Colorful terminal utility. \nUsage: ");
    println!("      textf [<options>] <text>");
    println!("Options: ");
    println!("{}", "        --help - Shows this text.\n".to_owned() +
        "        -f, --font <color> - Set texts font color.\n" +
        "        -b, --background <color> - Set texts background color.\n" +
        "        -r, --random - Generate and show random image.\n" +
        "        -p, --print <color | style> - Prints info.\n" +
        "        --png <path> - Shows png file in console");

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

fn get_style(text: String) -> Check<Style> {
    match &*text {
        "bold" => Some(Style::Bold),
        "italic" => Some(Style::Italic),
        "normal" => Some(Style::Normal),
        "faded" => Some(Style::Faded),
        "strikethrough" => Some(Style::Strikethrough),
        "underlined" => Some(Style::Underlined),
        "flashing" => Some(Style::Flashing),
        _ => None
    }
}

fn main() {
    let args = std::env::args().collect::<Vec<String>>();
    if args.len() == 1 {
        print_help();
        return;
    }
    if args[1] == "--help" {
        print_help();
        return;
    }

    let mut options: HashMap<&str, fn(String) -> OptionType> = HashMap::new();

    fn style_flag(arg: String) -> OptionType {
        match get_style(arg) {
            Some(n) => OptionType::Style(n),
            None => {
                println!("{}", font(Color::Red, "Error: this style is not supported."));
                OptionType::NoPrint
            },
        }
    }
    fn animation_flag(arg: String) -> OptionType {
        show_animation(arg);
        return OptionType::NoPrint;
    }
    fn print_flag(arg: String) -> OptionType {
        match &*arg {
            "color" => {
                println!("Colors:");
                println!("    black, red, green, yellow, blue, purple, cyan, white.");
            },
            "style" => {
                println!("Styles:");
                println!("    normal, bold, flashing, italic, underlined, faded, strikethrough.");
            },
            _ => {
                println!("{}", font(Color::Red, "Error: undefined info type"));
                return OptionType::NoPrint;
            },
        };
        OptionType::NoPrint
    }
    fn font_flag(arg: String) -> OptionType {
        match get_color(arg) {
            Some(n) => OptionType::Font(n),
            None => {
                println!("{}", font(Color::Red, "Error: this color is not supported."));
                OptionType::NoPrint
            },
        }
    }
    fn png_flag(arg: String) -> OptionType {
        png::show_in_console(arg);
        OptionType::NoPrint
    }
    fn background_flag(arg: String) -> OptionType {
        match get_color(arg) {
            Some(n) => OptionType::Background(n),
            None => {
                println!("{}", font(Color::Red, "Error: this color is not supported."));
                OptionType::NoPrint
            },
        }
    }

    options.insert("-a", animation_flag);
    options.insert("--animation", animation_flag);
    options.insert("--print", print_flag);
    options.insert("-p", print_flag);
    options.insert("-s", style_flag);
    options.insert("-f", font_flag);
    options.insert("-b", background_flag);
    options.insert("--png", png_flag);
    options.insert("--style", style_flag);
    options.insert("--font", font_flag);
    options.insert("--background", background_flag);

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
    if prev_flag != "" {
        uoptions.push(Option::new(prev_flag, "".to_string()));
    }
    let mut background_c = Color::Black;
    let mut font_c = Color::White;
    let mut style_t = Style::Normal;

    for i in uoptions {
        match (options.get(&*i.flag.clone()).unwrap())(i.value) {
            OptionType::Background(n) => background_c = n,
            OptionType::Font(n) => font_c = n,
            OptionType::Style(n) => style_t = n,
            OptionType::NoPrint => return,
        }
    }
    println!("{}", style_font_background(style_t, font_c, background_c, text));
}