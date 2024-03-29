use std::env;
use std::fs::OpenOptions;
use std::io::{self, Write};

#[path = "helpers/print_type.rs"]
mod print_type;

mod helpers;
mod perform_command;
mod separate_command;
mod shell_commands;

enum Command {
    Normal(String),
    Exit,
    Cd,
}

impl Command {
    fn from_str(command: &str) -> Self {
        match command.trim().to_lowercase().as_str() {
            "exit" => Command::Exit,
            "cd" => Command::Cd,
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
    let dir = cur_dir.to_string_lossy();

    loop {
        print!("{:}: ", dir.trim_matches(&['\n', '"']));

        io::stdout().flush().unwrap();
        let mut user_input = String::new();

        io::stdin().read_line(&mut user_input).unwrap();

        let command = user_input.trim();

        if let Err(err) = command_history_file.write_all(format!("{}\n", command).as_bytes()) {
            eprintln!("a error occured writing command history: {}", err);
        };

        let command_string = command.to_string();
        let (cmd, args) = separate_command::separate_command(&command_string);

        let cmd_enum = Command::from_str(cmd.clone().as_str());

        match cmd_enum {
            Command::Exit => {
                break;
            }
            Command::Cd => {
                shell_commands::cd::cd();
            }
            Command::Normal(cmd) => perform_command::perform_command(cmd, args),
        }
    }
}
