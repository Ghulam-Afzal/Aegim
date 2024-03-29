use core::panic;
use std::env;
use std::fs::OpenOptions;
use std::io::{self, Write};
use std::path::Path;

#[path = "helpers/print_type.rs"]
mod print_type;

mod helpers;
mod perform_command;
mod separate_command;
mod shell_commands;

enum Command {
    Normal(String),
    Exit,
    Cd(Vec<String>),
}

impl Command {
    fn from_str(command: &str, args: Vec<&str>) -> Self {
        match command.trim().to_lowercase().as_str() {
            "exit" => Command::Exit,
            "cd" => Command::Cd(args.iter().map(|&s| s.to_string()).collect()),
            other => Command::Normal(other.to_string()),
        }
    }
}

fn main() {
    let mut command_history_file = OpenOptions::new()
        .read(true)
        .append(true)
        .create(true)
        .open("aegim_history.txt")
        .unwrap();

    let cur_dir = env::current_dir().expect("could not get the current dir");
    let path = Path::new(cur_dir.as_os_str());

    let name;

    match path.file_name() {
        Some(n) => match n.to_str() {
            Some(nme) => name = nme.trim_matches('"'),
            None => panic!("unable to convert dir name to str"),
        },
        None => panic!("unable to get the currect directory"),
    };

    loop {
        print!("{:?}: ", name);

        io::stdout().flush().unwrap();
        let mut user_input = String::new();

        io::stdin().read_line(&mut user_input).unwrap();

        let command = user_input.trim();

        if let Err(err) = command_history_file.write_all(format!("{}\n", command).as_bytes()) {
            eprintln!("a error occured writing command history: {}", err);
        };

        let command_string = command.to_string();
        let (cmd, args) = separate_command::separate_command(&command_string);

        match Command::from_str(cmd.clone().as_str(), args.clone()) {
            Command::Exit => {
                break;
            }
            Command::Cd(args) => {
                shell_commands::cd::cd(args);
            }
            Command::Normal(cmd) => perform_command::perform_command(cmd, args),
        }
    }
}
