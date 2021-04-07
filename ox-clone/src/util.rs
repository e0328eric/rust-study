use regex::Regex;
use unicode_segmentation::UnicodeSegmentation;
use unicode_width::{UnicodeWidthChar, UnicodeWidthStr};

#[derive(Debug, Clone)]
pub struct Exp {
    pub ansi: Regex,
}

impl Exp {
    pub fn new() -> Self {
        Self {
            ansi: Regex::new(r"\u{1b}\[[0-?]*[ -/]*[@-~]").unwrap(),
        }
    }

    pub fn ansi_len(&self, string: &str) -> usize {
        UnicodeWidthStr::width(&*self.ansi.replace_all(string, ""))
    }
}

pub fn title(c: &str) -> String {
    c.chars().next().map_or(String::new(), |f| {
        f.to_uppercase().collect::<String>() + &c[1..]
    })
}

pub fn trim_end(text: &str, end: usize) -> String {
    let mut widths = Vec::new();
    for i in text.chars() {
        widths.push(UnicodeWidthChar::width(i).map_or(0, |i| i));
    }
    let chars: Vec<&str> = text.graphemes(true).collect();
    let mut result = vec![];
    let mut length = 0;
    for i in 0..chars.len() {
        let chr = chars[i];
        let wid = widths[i];
        if length == end {
            return result.join("");
        } else if length + wid <= end {
            result.push(chr.to_string());
            length += wid;
        } else if length + wid > end {
            result.push("".to_string());
            return result.join("");
        }
    }

    result.join("")
}

pub fn line_offset(point: usize, offset: i128, limit: usize) -> usize {
    if offset.is_negative() {
        if point as i128 + offset >= 0 {
            (point as i128 + offset) as usize
        } else {
            0
        }
    } else if point as i128 + offset < limit as i128 {
        (point as i128 + offset) as usize
    } else {
        limit.saturating_sub(1)
    }
}

pub fn spaces_to_tabs(code: &str, tab_width: usize) -> String {
    let mut result = vec![];
    for mut line in code.split('\n') {
        let mut spaces = 0;
        for c in line.chars() {
            if c == ' ' {
                spaces += 1;
            } else {
                break;
            }
        }
        let tabs = spaces / tab_width;
        line = &line[spaces..];
        result.push(format!("{}{}", "\t".repeat(tabs), line));
    }
    result.join("\n")
}

pub fn tabs_to_spaces(code: &str, tab_width: usize) -> String {
    let mut result = vec![];
    for mut line in code.split('\n') {
        let mut tabs = 0;
        for c in line.chars() {
            if c == '\t' {
                tabs += 1;
            } else {
                break;
            }
        }
        let spaces = tabs * tab_width;
        line = &line[tabs..];
        result.push(format!("{}{}", " ".repeat(spaces), line));
    }
    result.join("\n")
}

pub fn is_ansi(s: &str, chk: &Regex) -> bool {
    chk.is_match(s)
}

pub fn safe_ansi_insert(index: usize, list: &[&str], chk: &Regex) -> Option<usize> {
    let mut c = 0;
    for (ac, i) in list.iter().enumerate() {
        if !is_ansi(i, chk) {
            c += 1;
        }
        if c == index {
            return Some(ac.saturating_add(1));
        }
    }
    None
}
