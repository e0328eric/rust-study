mod default;

use crossterm::style::{Color, SetBackgroundColor, SetForegroundColor};
use regex::Regex;
use ron::de::from_str;
use serde::Deserialize;
use std::collections::HashMap;
use std::fs;

#[derive(Clone)]
pub enum TokenType {
    MultiLine(String, Vec<Regex>),
    SingleLine(String, Vec<Regex>),
}

#[derive(Debug)]
pub enum Status {
    Parse(String),
    File,
    Success,
    Empty,
}

#[derive(Debug, Copy, Clone, Hash, Eq, PartialEq, Deserialize)]
pub enum KeyBinding {
    Ctrl(RawKey),
    Alt(RawKey),
    Shift(RawKey),
    F(u8),
    Unsupported,
}

#[derive(Debug, Copy, Clone, Hash, Eq, PartialEq, Deserialize)]
pub enum RawKey {
    Char(char),
    Up,
    Down,
    Left,
    Right,
    Backspace,
    Enter,
    Tab,
    Home,
    End,
    PageUp,
    PageDown,
    Delete,
    Insert,
    Null,
    Esc,
}

#[derive(Debug, Clone, Deserialize)]
pub struct Reader {
    pub general: General,
    pub theme: Theme,
    pub macros: HashMap<String, Vec<String>>,
    pub highlights: HashMap<String, HashMap<String, (u8, u8, u8)>>,
    pub keys: HashMap<KeyBinding, Vec<String>>,
    pub languages: Vec<Language>,
}

#[derive(Debug, Clone, Deserialize)]
pub struct General {
    pub line_number_padding_right: usize,
    pub line_number_padding_left: usize,
    pub tab_width: usize,
    pub undo_period: u64,
    pub status_left: String,
    pub status_right: String,
    pub tab: String,
    pub warp_cursor: bool,
}

#[derive(Debug, Deserialize, Clone)]
pub struct Theme {
    pub transparent_editor: bool,
    pub editor_bg: (u8, u8, u8),
    pub editor_fg: (u8, u8, u8),
    pub status_bg: (u8, u8, u8),
    pub status_fg: (u8, u8, u8),
    pub line_number_fg: (u8, u8, u8),
    pub line_number_bg: (u8, u8, u8),
    pub inactive_tab_fg: (u8, u8, u8),
    pub inactive_tab_bg: (u8, u8, u8),
    pub active_tab_fg: (u8, u8, u8),
    pub active_tab_bg: (u8, u8, u8),
    pub warning_fg: (u8, u8, u8),
    pub error_fg: (u8, u8, u8),
    pub info_fg: (u8, u8, u8),
    pub default_theme: String,
    pub fallback: bool,
}

#[derive(Debug, Deserialize, Clone)]
pub struct Language {
    pub name: String,
    pub icon: String,
    pub extensions: Vec<String>,
    pub keywords: Vec<String>,
    pub definitions: HashMap<String, Vec<String>>,
}

fn default() -> String {
    default::DEFAULT_CONFIG.to_string()
}

impl Reader {
    pub fn read(config: &str) -> (Self, Status) {
        let config = if let Ok(config) = shellexpand::full(config) {
            (*config).to_string()
        } else {
            config.to_string()
        };
        if let Ok(file) = fs::read_to_string(config) {
            let result: (Self, Status) = if let Ok(contents) = from_str(&file) {
                (contents, Status::Success)
            } else if file.is_empty() {
                (from_str(&default()).unwrap(), Status::Empty)
            } else {
                let result: Result<Self, ron::Error> = from_str(&file);
                (
                    from_str(&default()).unwrap(),
                    Status::Parse(format!("{:?}", result)),
                )
            };
            result
        } else {
            (from_str(&default()).unwrap(), Status::File)
        }
    }

    pub fn get_syntax_regex(config: &Self, extension: &str) -> Vec<TokenType> {
        let mut result = vec![];
        for lang in &config.languages {
            if lang.extensions.contains(&extension.to_string()) {
                for (name, reg) in &lang.definitions {
                    let mut single = vec![];
                    let mut multi = vec![];
                    for expr in reg {
                        if expr.starts_with("(?ms)") || expr.starts_with("(?sm)") {
                            if let Ok(regex) = Regex::new(&expr) {
                                multi.push(regex);
                            }
                        } else {
                            if let Ok(regex) = Regex::new(&expr) {
                                single.push(regex);
                            }
                        }
                    }
                    if !single.is_empty() {
                        result.push(TokenType::SingleLine(name.clone(), single));
                    }
                    if !multi.is_empty() {
                        result.push(TokenType::MultiLine(name.clone(), multi));
                    }
                }
                result.push(TokenType::SingleLine(
                    "keywords".to_string(),
                    lang.keywords
                        .iter()
                        .map(|x| Regex::new(&format!(r"\b({})\b", x)).unwrap())
                        .collect(),
                ))
            }
        }

        result
    }

    pub fn rgb_fg(color: (u8, u8, u8)) -> SetForegroundColor {
        SetForegroundColor(Color::Rgb {
            r: color.0,
            g: color.1,
            b: color.2,
        })
    }

    pub fn rgb_bg(color: (u8, u8, u8)) -> SetBackgroundColor {
        SetBackgroundColor(Color::Rgb {
            r: color.0,
            g: color.1,
            b: color.2,
        })
    }
}
