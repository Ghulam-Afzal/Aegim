use std::env;
use std::path::Path;

pub fn pwd() {
    let cur_dir =
        env::current_dir().expect("error occured trying to get the current working directory.");
    let path = Path::new(cur_dir.as_os_str());

    let current_working_directory;

    match path.to_str() {
        Some(s) => current_working_directory = s.trim_matches('"'),
        None => panic!("something went wrong"),
    }

    println!("{}", current_working_directory)
}
