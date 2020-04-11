use std::ffi::OsStr;
use std::process::{self, Command};
use std::path::PathBuf;

use crate::utils;

pub struct TeXEngine<'a, 'b, 'c>(pub &'a str, pub &'b Vec<&'c PathBuf>);

impl<'a, 'b, 'c> TeXEngine<'a, 'b, 'c> {
    pub fn run_once(&self) -> bool {
        let mut succ_or_fail: Vec<bool> = Vec::new();
        for file in self.1 {
            match Command::new(&self.0).arg(file).status() {
                Ok(a) => succ_or_fail.push(!(a.success())),
                Err(e) => {
                    eprintln!("autotex Error : {}", e);
                    succ_or_fail.push(true);
                },
            }
        }

        succ_or_fail.iter().all(|&x| x)
    }
}

pub fn run_engine(args: &Vec<&String>, fname: &str) {
    let engine = take_engine(&args[1..]).unwrap_or_else(|err| {
        eprintln!("autotex Error: {}", err);
        process::exit(1);
    });
    let fname_ = PathBuf::from(fname);
    let filename = vec![&fname_];
    // Yet it does not support the option -cd
    if utils::is_extension_exists(&None, "bib") {
        let bibfn = PathBuf::from(fname_.file_stem().unwrap());
        let bibfilename = vec![&bibfn];
        let run_engine = vec![
            TeXEngine(engine, &filename),
            TeXEngine("bibtex", &bibfilename),
            TeXEngine(engine, &filename),
            TeXEngine(engine, &filename)
        ];
        for i in run_engine {
            let j = i.run_once();
            if j { break; }
        }
    } else if utils::is_extension_exists(&None, "idx") {
        let mkindfn1 = utils::get_files(&None);
        let mkindfn = mkindfn1.iter()
            .filter(|x| x.extension() == Some(OsStr::new("idx"))).collect();
        let run_engine = vec![
            TeXEngine(engine, &filename),
            TeXEngine("makeindex", &mkindfn),
            TeXEngine(engine, &filename),
            TeXEngine(engine, &filename)
        ];
        for i in run_engine {
            let j = i.run_once();
            if j { break; }
        }
    } else {
        for _i in 0..2 {
            let j = TeXEngine(engine, &filename).run_once();
            if j { break; }
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

pub fn take_engine<'a>(args: &'a[&'a String]) -> Result<&'a str, &'a str> {
    let engines_list = vec![
        "pdftex", "tex", "xetex", "luatex",
        "pdflatex", "latex", "xelatex", "lualatex"
    ];
    let use_latex = String::from("-la");
    match args.len() {
        0 => Ok("pdftex"),
        1 => {
            if *args[0] == String::from("-la") {
                Ok("pdflatex")
            } else {
                Ok(engines_list[is_engine_args(&args[0])?])
            }
        }
        2 => {
            if *args[0] == use_latex || *args[1] == use_latex {
                let other_option = args.iter()
                    .filter(|x| ***x != String::from("-la")).next().unwrap();
                let n = is_engine_args(&other_option)?;
                Ok(engines_list[n+4])
            } else {
                Err("Cannot use two distinct TeX options")
            }
        },
        _ => Err("Too Many Arguments"),
    }
}
