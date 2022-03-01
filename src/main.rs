use std::io::{self, Write}; 


fn main() {
    
    // main loop for the program 
    loop {

        print!("> ");
        io::stdout().flush().unwrap();

        let mut user_input  = String::new();

        io::stdin().read_line(&mut user_input).unwrap();

        let command = user_input.trim(); 

        if command == "exit" {
            break
        }
    }
}
