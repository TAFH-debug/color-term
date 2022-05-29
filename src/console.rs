use crate::structs::*;

static ENDING: &'static str = "\x1b[0m";

/**
Sets background color to given text.
 */
pub fn background<S: AsRef<str>>(color: Color, text: S) -> String {
    let prefix = get_prefix(color, true);
    prefix + text.as_ref() + ENDING
}

/**
Sets font to given text.
 */
pub fn font<S: AsRef<str>>(color: Color, text: S) -> String {
    let prefix = get_prefix(color, false);
    prefix + text.as_ref() + ENDING
}

/**
Sets style to given text.
 */
pub fn style<S: AsRef<str>>(style: Style, text: S) -> String {
    let prefix = get_style_prefix(style);
    prefix + text.as_ref() + ENDING
}

/**
Replaces **sym** with a **color**
 */
pub fn paint_sym<S: AsRef<str>>(src: S, color: Color, sym: char) -> String {
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