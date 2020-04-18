#![allow(unused)]
#[derive(Debug, Clone)]
enum Exp {
    X,
    INT(isize),
    ADD(Box<Exp>, Box<Exp>),
    SUB(Box<Exp>, Box<Exp>),
    MUL(Box<Exp>, Box<Exp>),
    DIV(Box<Exp>, Box<Exp>),
    SIGMA(Box<Exp>, Box<Exp>, Box<Exp>),
}

fn fnt_exp(exp: Exp) -> Box<dyn FnOnce(isize) -> isize> {
    match exp {
        Exp::X => Box::new(|x| x),
        Exp::INT(n) => Box::new(move |_x| n),
        Exp::ADD(e1, e2) => Box::new(move |x| fnt_exp(*e1)(x) + fnt_exp(*e2)(x)),
        Exp::SUB(e1, e2) => Box::new(move |x| fnt_exp(*e1)(x) - fnt_exp(*e2)(x)),
        Exp::MUL(e1, e2) => Box::new(move |x| fnt_exp(*e1)(x) * fnt_exp(*e2)(x)),
        Exp::DIV(e1, e2) => Box::new(move |x| fnt_exp(*e1)(x) / fnt_exp(*e2)(x)),
        Exp::SIGMA(e1, e2, ex) => Box::new(move |x| {
            let mut sum = 0;
            for i in fnt_exp(*e1)(x)..(fnt_exp(*e2)(x)+1) {
                let ex_ = ex.clone();
                sum += fnt_exp(*ex_)(i)
            }
            sum
        }),
    }
}

fn calculator(exp: Exp) -> isize {
    fnt_exp(exp)(0)
}

fn main() {
    let sample = Exp::SIGMA(
        Box::new(Exp::INT(1)),
        Box::new(Exp::INT(10)),
        Box::new(Exp::SIGMA(
                Box::new(Exp::INT(1)),
                Box::new(Exp::INT(5)),
                Box::new(Exp::SUB(
                        Box::new(Exp::MUL(
                                Box::new(Exp::X),
                                Box::new(Exp::X)
                                            )),
                        Box::new(Exp::X))
                        )
    )));
    let print = calculator(sample);
    println!("{}", print);
}
