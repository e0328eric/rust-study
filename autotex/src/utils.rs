use std::path::{Path, PathBuf};
use std::process;
use std::fs;
use std::ffi::OsStr;

type Time = std::time::SystemTime;

// Handle autotex options
pub fn is_valid_args(args: Vec<String>) -> Vec<String>{
    if args.len() < 2 {
        eprintln!("autotex Error: There is no file to compile");
        process::exit(1);
    } else {
        args
    }
}

// File name and modified time collector
pub fn get_files(dir: &Option<PathBuf>) -> Vec<PathBuf> {
    let dir_d = if let Some(dirdir) = dir {
        dirdir
    } else {
        PathBuf::from("./")
    };
    let mut lst_files: Vec<PathBuf> = Vec::new();
    for file in fs::read_dir(dir_d.to_str().unwrap()).unwrap().map(|x| x.unwrap()) {
        if file.path().is_dir() {
            lst_files.append(&mut get_files(&Some(file.path())));
        } else {
            lst_files.push(file.path());
        }
    }

    lst_files
}

pub fn get_file_times(dir: &Option<&'static str>) -> Vec<Time> {
    let dir_d = dir.unwrap_or_else(|| "./");
    fs::read_dir(dir_d).unwrap()
        .map(|n| n.unwrap().metadata().unwrap().modified().unwrap())
        .collect()
}

// Check the given extension exists
pub fn is_extension_exists(dir: &Option<PathBuf>, ext: &'static str) -> bool {
    let lst = get_files(&dir);
    lst.iter().any(|x| x.extension() == Some(OsStr::new(ext)))
}

#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn testing_exists_files() {
        assert_eq!(true, is_extension_exists(&None, "rs"));
        assert_eq!(true, is_extension_exists(&None, "tex"));
        assert_eq!(true, is_extension_exists(&None, "pdf"));
    }

    #[test]
    fn testing_non_exists_files() {
        assert_eq!(false, is_extension_exists(&None, "tdt"));
        assert_eq!(false, is_extension_exists(&None, "hwp"));
        assert_eq!(false, is_extension_exists(&None, "bib"));
    }
}
