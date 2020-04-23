use crate::parser::BF;
use crate::tape::Tape;
use crate::modint::ModInt;

pub fn interpreter(lst: &Vec<BF>, mut tape: &mut Tape, input: &(Option<&String>, usize)) -> String {
    let mut output = String::new();
    let empty = String::new();
    let input_str = input.0.unwrap_or_else(|| &empty);
    for i in lst {
        match i {
            BF::Add => tape.change(tape.take_val() + ModInt(1)),
            BF::Sub => tape.change(tape.take_val() - ModInt(1)),
            BF::Left => tape.move_left(),
            BF::Right => tape.move_right(),
            BF::Input => tape.change(ModInt(input_str.bytes().next().unwrap())),
            BF::Output => output.push_str(&tape.take_val().to_string()),
            BF::Loop(l) => {
                while tape.take_val() != ModInt(0) {
                    output.push_str(&interpreter(&l, &mut tape, &input));
                    if tape.take_val() == ModInt(0) {
                        break;
                    }
                }
            },
        }
    }
    output
}
