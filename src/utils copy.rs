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

fn get_files_in_dir(path: &str) -> io::Result<Vec<String>> {
    // Get a list of all entries in the folder
    let entries = fs::read_dir(path)?;

    // Extract the filenames from the directory entries and store them in a vector
    let file_names: Vec<String> = entries
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

fn sleep(time: u64) {
    let secs = time::Duration::from_secs(time);
    //let now = time::Instant::now();

    thread::sleep(secs);
}