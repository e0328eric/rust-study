use std::process::Command;

pub struct TeXEngine(pub String);

#[derive(PartialEq)]
pub enum TeXorLaTeX {
    TeX,
    LaTeX,
}

impl TeXEngine {
    pub fn run_once(&self, filename: &str) -> bool {
        match Command::new(&self.0).arg(filename).status() {
            Ok(a) => !(a.success()),
            Err(e) => {
                eprintln!("autotex Error : {}", e);
                true
            },
        }
    }

    pub fn run(vec: &Vec<Self>, filename: &str) {
        for i in vec {
            let j = i.run_once(filename);
            if j {
                break;
            }
        }
    }
}

pub fn is_engine_args(arg: &str) -> Result<usize, &str> {
    let engine_args: Vec<String> = vec![
        String::from("-pdf"), String::from("-plain"),
        String::from("-xe"), String::from("-lua")
    ];
    match engine_args.iter().position(|x| x == arg) {
        Some(n) => Ok(n),
        None => Err("Wrong engine options"),
    }
}

pub fn take_engine(args: &[String]) -> Result<(TeXorLaTeX, &str), &str> {
    let engines_list = vec![
        "pdftex", "tex", "xetex", "luatex",
        "pdflatex", "latex", "xelatex", "lualatex"
    ];
    let use_latex = String::from("-la");
    match args.len() {
        0 => Ok((TeXorLaTeX::TeX, "pdftex")),
        1 => {
            if args[0] == String::from("-la") {
                Ok((TeXorLaTeX::LaTeX, "pdflatex"))
            } else {
                let n = is_engine_args(&args[0])?;
                Ok((TeXorLaTeX::TeX, engines_list[n]))
            }
        }
        2 => {
            if args[0] == use_latex || args[1] == use_latex {
                let other_option = args.iter()
                    .filter(|x| **x != String::from("-la")).next().unwrap();
                let n = is_engine_args(&other_option)?;
                Ok((TeXorLaTeX::LaTeX, engines_list[n+4]))
            } else {
                Err("Cannot use two distinct TeX options")
            }
        },
        _ => Err("Too Many Arguments"),
    }
}
