pub const RED: &str = "\x1b[31m";
pub const BRIGHT_RED: &str = "\x1b[1;31m";
pub const GREEN: &str = "\x1b[32m";
pub const YELLOW: &str = "\x1b[33m";
pub const BLUE: &str = "\x1b[34m";
pub const MAGENTA: &str = "\x1b[35m";
pub const CYAN: &str = "\x1b[36m";
pub const WHITE: &str = "\x1b[37m";
pub const RESET: &str = "\x1b[0m";

pub const BOLD: &str = "\x1b[1m";
pub const UNDERLINE: &str = "\x1b[4m";
pub const ITALIC: &str = "\x1b[3m";

pub fn bold(text: &str) -> String {
    format!("{}{}{}", BOLD, text, RESET)
}

pub fn underline(text: &str) -> String {
    format!("{}{}{}", UNDERLINE, text, RESET)
}

pub fn italic(text: &str) -> String {
    format!("{}{}{}", ITALIC, text, RESET)
}

// You can also provide functions for convenience
pub fn red(text: &str) -> String {
    format!("{}{}{}", RED, text, RESET)
}

pub fn green(text: &str) -> String {
    format!("{}{}{}", GREEN, text, RESET)
}

pub fn cyan(text: &str) -> String {
    format!("{}{}{}", CYAN, text, RESET)
}

pub fn white(text: &str) -> String {
    format!("{}{}{}", WHITE, text, RESET)
}