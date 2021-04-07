use crate::config::{Reader, TokenType};
use std::collections::HashMap;
use unicode_width::UnicodeWidthStr;

#[derive(Debug, Clone)]
pub struct Token {
    pub span: (usize, usize),
    pub data: String,
    pub kind: String,
    pub priority: usize,
}

pub fn cine(token: &Token, hashmap: &mut HashMap<usize, Token>) {
    if let Some(t) = hashmap.get(&token.span.0) {
        if t.priority > token.priority {
            return;
        }
    }
    hashmap.insert(token.span.0, token.clone());
}

fn bounds(reg: &regex::Match, line: &str) -> (usize, usize) {
    let unicode_width = UnicodeWidthStr::width(reg.as_str());
    let pre_length = UnicodeWidthStr::width(&line[..reg.start()]);
    (pre_length, pre_length + unicode_width)
}

fn multi_to_single(doc: &str, m: &regex::Match) -> ((usize, usize), (usize, usize)) {
    let b = bounds(&m, &doc);
    let start_y = doc[..m.start()].matches('\n').count();
    let end_y = doc[..m.end()].matches('\n').count();
    let start_x = b.0
        - UnicodeWidthStr::width(&doc.split('\n').take(start_y).collect::<Vec<_>>().join("\n")[..]);
    let end_x = b.1
        - UnicodeWidthStr::width(&doc.split('\n').take(end_y).collect::<Vec<_>>().join("\n")[..]);
    ((start_x, start_y), (end_x, end_y))
}

pub fn highlight(
    row: &str,
    doc: &str,
    index: usize,
    regex: &[TokenType],
    highlights: &HashMap<String, (u8, u8, u8)>,
) -> HashMap<usize, Token> {
    let mut syntax = HashMap::<usize, Token>::new();
    if regex.is_empty() {
        return syntax;
    }
    for exps in regex {
        match exps {
            TokenType::SingleLine(name, regex) => {
                if name == "keywords" {
                    for kw in regex {
                        for cap in kw.captures_iter(row) {
                            let cap = cap.get(cap.len().saturating_sub(1)).unwrap();
                            let boundaries = bounds(&cap, &row);
                            cine(
                                &Token {
                                    span: boundaries,
                                    data: cap.as_str().to_string(),
                                    kind: Reader::rgb_fg(highlights["keywords"]).to_string(),
                                    priority: 0,
                                },
                                &mut syntax,
                            );
                        }
                    }
                }
            }
        }
    }

    syntax
}
