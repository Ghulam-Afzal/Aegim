use std::io::{self, Write}; 
use std::process::Command; 

fn main() {
    
    // main loop for the program 
    loop {

        print!("> ");
        io::stdout().flush().unwrap();

        let mut user_input  = String::new();

        io::stdin().read_line(&mut user_input).unwrap();

        let command = user_input.trim(); 
        
        // if the command that was entered was exit that break out of the loop 
        if command == "exit" { 
            break 
        }

        let mut child = Command::new(command)
            .spawn()
            .unwrap();

        // to make sure that only one command is accecpted at a time
        child.wait(); 

    }
}
