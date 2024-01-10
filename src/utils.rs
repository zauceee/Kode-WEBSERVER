use std::io;
use std::fs;
use std::env;
use std::{thread, time};

/*
*******

  _    _ _______ _____ _       _____ 
 | |  | |__   __|_   _| |     / ____|
 | |  | |  | |    | | | |    | (___  
 | |  | |  | |    | | | |     \___ \ 
 | |__| |  | |   _| |_| |____ ____) |
  \____/   |_|  |_____|______|_____/ 
                                     
                                     
Files used for useful fuctions such as sleep, and for getting files from a dir.

*******

*/

pub fn get_files_in_dir(path: &str) -> io::Result<Vec<String>> {
    let canonical_path = fs::canonicalize(path)?;
    let entries = fs::read_dir(canonical_path)?;

    let mut file_names: Vec<String> = entries
        .filter_map(|entry| {
            let path = entry.ok()?.path();
            if path.is_file() {
                path.file_name()?.to_str().map(|s| s.to_owned())
            } else {
                None
            }
        })
        .collect();

    Ok(file_names)
}

pub fn remove_suffix<'a>(s: &'a str, suffix: &str) -> &'a str {

    match s.strip_suffix(suffix) {
        Some(s) => s,
        None => s
    }
}

pub fn remove_prefix<'a>(s: &'a str, prefix: &str) -> &'a str {

    match s.strip_prefix(prefix) {
        Some(s) => s,
        None => s
    }
}
