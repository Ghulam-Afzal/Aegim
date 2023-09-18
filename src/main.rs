use std::io::{self, Write}; 
use std::fs::OpenOptions;

#[path = "helpers/print_type.rs"] mod print_type;

mod perform_command;
mod separate_command;

fn main() {
    
    // create a file if not exists to store the command history
    let mut command_history_file = OpenOptions::new().read(true).append(true).create(true).open("aegim_history.txt").unwrap();

    // main loop for the program 
    loop {

        print!("> ");
        io::stdout().flush().unwrap();

        let mut user_input = String::new();

        io::stdin().read_line(&mut user_input).unwrap();

        let command = user_input.trim(); 
        
        // write the command to the history file
        let _ = command_history_file.write_all(&command.as_bytes());
        let _ = command_history_file.write_all("\n".as_bytes());

        // if the command that was entered was exit that break out of the loop 
        if command == "exit" { 
            break 
        }

        #[allow(unused)]
        let mut cmd = String::new();
        
        #[allow(unused)]
        let mut args = Vec::new();

        // call perform_command() with the correct params;
        let command_string = command.to_string();    
        (cmd, args) = separate_command::separate_command(&command_string);

        perform_command::perform_command(cmd, args);

    }
}
