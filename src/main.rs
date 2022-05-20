use std::io::{self, Write}; 

#[path = "helpers/print_type.rs"] mod print_type;

mod perform_command;
mod seperate_command;

fn main() {
    
    // main loop for the program 
    loop {

        print!("> ");
        io::stdout().flush().unwrap();

        let mut user_input = String::new();

        io::stdin().read_line(&mut user_input).unwrap();

        let command = user_input.trim(); 
        
        // if the command that was entered was exit that break out of the loop 
        if command == "exit" { 
            break 
        }

        let mut cmd = String::new();
        let mut args = Vec::new();

        // call perfrom_cammand() with the correct params;
        let command_string = command.to_string();    
        (cmd, args) = seperate_command::seperate_command(&command_string);

        perform_command::perform_command(cmd, args);

    }
}
