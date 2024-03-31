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

    if command == "cat" || command == "echo" {
        for line in buffer.lines() {
            if let Ok(line) = line {
                print!("{}", line);
            } else {
                eprint!("Error reading output");
                break;
            }
        }
    } else {
        for line in buffer.lines() {
            match line {
                Ok(line) => {
                    let colored_line = colorize_output(line);
                    print!("{} ", colored_line);
                }
                Err(err) => {
                    eprint!("Error reading output: {}", err);
                    break;
                }
            }
        }
    }
    print!("\n");
}
