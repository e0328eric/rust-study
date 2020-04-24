use crate::lexer::Lexer;
#[derive(Debug)]
pub enum BF {
    Add,
    Sub,
    Left,
    Right,
    Input,
    Output,
    Loop(Box<Vec<BF>>),
}

pub fn parser(lst: &Vec<Lexer>) -> Vec<BF> {
    let mut output: Vec<BF> = Vec::new();
    let mut i = 0;
    while i < lst.len() {
        match lst[i] {
            Lexer::Add => output.push(BF::Add),
            Lexer::Sub => output.push(BF::Sub),
            Lexer::Left => output.push(BF::Left),
            Lexer::Right => output.push(BF::Right),
            Lexer::Input => output.push(BF::Input),
            Lexer::Output => output.push(BF::Output),
            Lexer::BLoop => {
                let j = find_match_loop(&lst, &i);
                output.push(BF::Loop(Box::new(parser(&lst[i+1..=j-1].to_vec()))));
                i = j;
            },
            Lexer::ELoop => {},
        }
        i += 1;
    }
    output
}

fn find_match_loop(lst: &Vec<Lexer>, num: &usize) -> usize {
    let mut end = 1;
    let mut lst_eloop: Vec<usize> = Vec::new();
    if *num < lst.len() {
        for (e,i) in lst[*num+1..].iter().enumerate() {
            if end == 0 {
                break;
            } else if *i == Lexer::BLoop {
                end += 1;
            } else if *i == Lexer::ELoop {
                end -= 1;
                lst_eloop.push(num+e+1);
            }
        }
        if end != 0 {
            panic!("There is no matched closed loop symbol");
        }
        return lst_eloop.last().unwrap().clone();
    }
    panic!("There is no matched closed loop symbol");
}
