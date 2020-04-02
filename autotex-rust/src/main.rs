use std::env;
use std::process::Command;

struct TeXEngine(String);

impl TeXEngine {
    fn run_once(&self, filename: &str) -> bool {
        match Command::new(&self.0).arg(filename).status() {
                Ok(a) => !(a.success()),
                Err(e) => {
                    println!("autotex Error : {}", e);
                    true
                },
        }
    }

    fn run(vec: &Vec<Self>, filename: &str) {
        for i in vec {
            let j = i.run_once(filename);
            if j {
                break;
            }
        }
    }
}

// Maximum Argument can be 5.
// Since args[0] is fixed, matching the case 2, 3, 4, 5 and 6.
// Argument number 2 : (ex) autotex filename.tex
// Argument number 3 : (ex) autotex -xe filename.tex
// Argument number 4 : (ex) autotex -xe -la filename.tex
fn is_engine_args(arg: &str) -> Result<usize, &str> {
    let engine_args: Vec<String> = vec![
        String::from("-pdf"),
        String::from("-plain"),
        String::from("-xe"),
        String::from("-lua")
    ];
    match engine_args.iter().position(|x| x == arg) {
        Some(n) => Ok(n),
        None => Err("Wrong engine options"),
    }
}

fn is_valid_arg(args: &Vec<String>) -> Result<(&str, &str), &str> {
    let engines_list = vec![
        "pdftex", "tex", "xetex", "luatex",
        "pdflatex", "latex", "xelatex", "lualatex"
    ];
    let use_latex = String::from("-la");
    match args.len() {
        0 | 1 => Err("Too Few Arguments"),
        2 => Ok(("pdftex", &args[1])),
        3 => match is_engine_args(&args[1]) {
            Ok(n) => Ok((engines_list[n], &args[2])),
            Err(e) => Err(e),
        }
        4 => {
            if args[1] == use_latex {
                match is_engine_args(&args[2]) {
                    Ok(n) => Ok((engines_list[n+4], &args[3])),
                    Err(e) => Err(e),
                }
            } else if args[2] == use_latex {
                match is_engine_args(&args[1]) {
                    Ok(n) => Ok((engines_list[n+4], &args[3])),
                    Err(e) => Err(e),
                }
            } else {
                Err("Wrong engine options")
            }
        },
        _ => Err("Too Many Arguments"),
    }
}

fn main() {
    let args: Vec<String> = env::args().collect();
    let engine = is_valid_arg(&args);
    match engine {
        Ok((eng, filename)) => {
            let x = vec![
                TeXEngine(String::from(eng)),
                TeXEngine(String::from(eng))
            ];
            TeXEngine::run(&x, &filename);
        },
        Err(e) => eprintln!("autotex Error : {}", e),
    }
}
