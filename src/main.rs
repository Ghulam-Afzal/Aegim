use std::io::{self, Write}; 
use std::process::Command;

// func to run the command that was passed in 
fn perfrom_command(command: &String) {
    
    let mut child = Command::new(command)
        .spawn()
        .expect("not a valid command");

   let s_code = child.wait()
       .expect("Failed to wait on child process.");

   // assert that the command succeded 
   assert!(s_code.success());
}


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

        // call perfrom_cammand() with the correct params;
        let command_string = command.to_string();    
        perfrom_command(&command_string);

    }
}
