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
    let brace_lst = find_match_loop(&lst);
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
                let j = find_second_for_first(&brace_lst, &i);
                let tmp = Vec::from(&lst[i+1..=j-1]);
                output.push(BF::Loop(Box::new(parser(&tmp))));
                i = j;
            },
            Lexer::ELoop => {},
        }
        i += 1;
    }
    output
}

fn find_second_for_first(lst: &Vec<(usize, usize)>, num: &usize) -> usize {
    let mut output = 0;
    for (i,j) in lst.iter() {
        if i == num {
            output = j.clone();
            break;
        }
    }
    output
}

fn find_match_loop(lst: &Vec<Lexer>) -> Vec<(usize, usize)> {
    let mut _count = 0;
    let mut end = 0;
    let mut lst_eloop: Vec<usize> = Vec::new();
    let mut output: Vec<(usize, usize)> = Vec::new();
    for (e,i) in lst.iter().enumerate() {
        if *i == Lexer::BLoop {
            _count += 1;
            end += 1;
            for (f,j) in lst[e+1..].iter().enumerate() {
                if end == 0 {
                    break;
                } else if *j == Lexer::BLoop {
                    _count += 1;
                    end += 1;
                } else if *j == Lexer::ELoop {
                    end -= 1;
                    lst_eloop.push(e+f+1);
                }
            }
            output.push((e, lst_eloop.last().unwrap().clone()));
            _count = 0;
            end = 0;
            lst_eloop = Vec::new();
        }
    }
    output
}
