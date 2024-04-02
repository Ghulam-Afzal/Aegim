use std::fs::{self, metadata};
use std::{
    io::{self, BufRead},
    process::{Command, Stdio},
};

use crate::helpers::colorized_output::colorize_output;

pub fn perform_command(command: String, args: Vec<&str>) {
    let mut child = Command::new(&command)
        .args(args)
        .stdout(Stdio::piped())
        .spawn()
        .expect("not a valid command");

    let stdout = child
        .stdout
        .take()
        .expect("failed capture output from child process");

    let buffer = io::BufReader::new(stdout);

    if command == "ls" {
        for line in buffer.lines() {
            match line {
                Ok(file) => {
                    let metadata = fs::metadata(&file).expect("expected to get file meta data");
                    if metadata.file_type().is_dir() {
                        let colored_file = colorize_output(&file);
                        print!("{} ", &colored_file);
                    } else {
                        print!("{} ", file);
                    }
                }
                Err(err) => {
                    eprintln!("Error reading output: {}", err);
                    break;
                }
            }
        }
        println!("");
        return;
    }

    for line in buffer.lines() {
        if let Ok(line) = line {
            println!("{}", line);
        } else {
            eprintln!("error reading input from child process stdio");
            break;
        }
    }
}
