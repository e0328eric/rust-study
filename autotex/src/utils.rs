use std::fs::{self, DirEntry};

pub fn get_files_recur(dir: &Option<&'static str>) -> Vec<DirEntry> {
    let dir_d = if let Some(string) = dir {
        string
    } else {
        "./"
    };
    fs::read_dir(dir_d).unwrap().map(|x| x.unwrap()).collect()
}

pub fn get_file_times(dir: &Option<&'static str>) -> Vec<std::time::SystemTime> {
    let dir_d = if let Some(string) = dir {
        string
    } else {
        "./"
    };
    fs::read_dir(dir_d).unwrap()
        .map(|n| n.unwrap().metadata().unwrap().modified().unwrap())
        .collect()
}

