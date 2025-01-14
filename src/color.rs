use std::fmt;
use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize, Debug, PartialEq, Clone, PartialOrd, Eq, Ord)]
pub enum Color {
    Red,
    Yellow,
    Green,
    Blue,
    Purple,
}

impl fmt::Display for Color {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match self {
            Color::Red => write!(f, "Red"),
            Color::Yellow => write!(f, "Yellow"),
            Color::Green => write!(f, "Green"),
            Color::Blue => write!(f, "Blue"),
            Color::Purple => write!(f, "Purple"),
        }
    }
}

pub trait Colorize {
    fn red_fg(&self)    -> String;
    fn yellow_fg(&self) -> String;
    fn green_fg(&self)  -> String;
    fn blue_fg(&self)   -> String;
    fn purple_fg(&self) -> String;
    fn red_bg(&self)    -> String;
    fn yellow_bg(&self) -> String;
    fn green_bg(&self)  -> String;
    fn blue_bg(&self)   -> String;
    fn purple_bg(&self) -> String;
}

impl Colorize for str {
    fn red_fg(&self)    -> String { add_color(String::from("\x1b[31m"), self) }
    fn yellow_fg(&self) -> String { add_color(String::from("\x1b[33m"), self) }
    fn green_fg(&self)  -> String { add_color(String::from("\x1b[32m"), self) }
    fn blue_fg(&self)   -> String { add_color(String::from("\x1b[34m"), self) }
    fn purple_fg(&self) -> String { add_color(String::from("\x1b[35m"), self) }
    fn red_bg(&self)    -> String { add_color(String::from("\x1b[41m"), self) }
    fn yellow_bg(&self) -> String { add_color(String::from("\x1b[43m"), self) }
    fn green_bg(&self)  -> String { add_color(String::from("\x1b[42m"), self) }
    fn blue_bg(&self)   -> String { add_color(String::from("\x1b[44m"), self) }
    fn purple_bg(&self) -> String { add_color(String::from("\x1b[45m"), self) }
}

// Color the string
fn add_color(mut color: String, text: &str) -> String {
    let reset = "\x1b[0m";
    color.push_str(text);
    color.push_str(reset);
    color
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_colors() {
        let s = String::from("test");

        assert_eq!(s.red_fg(), String::from("\x1b[31mtest\x1b[0m"));
        assert_eq!(s.yellow_fg(), String::from("\x1b[33mtest\x1b[0m"));
        assert_eq!(s.green_fg(), String::from("\x1b[32mtest\x1b[0m"));
        assert_eq!(s.blue_fg(), String::from("\x1b[34mtest\x1b[0m"));
        assert_eq!(s.purple_fg(), String::from("\x1b[35mtest\x1b[0m"));
        assert_eq!(s.red_bg(), String::from("\x1b[41mtest\x1b[0m"));
        assert_eq!(s.yellow_bg(), String::from("\x1b[43mtest\x1b[0m"));
        assert_eq!(s.green_bg(), String::from("\x1b[42mtest\x1b[0m"));
        assert_eq!(s.blue_bg(), String::from("\x1b[44mtest\x1b[0m"));
        assert_eq!(s.purple_bg(), String::from("\x1b[45mtest\x1b[0m"));
    }
}
