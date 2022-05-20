use std::io::{self, Write}; 
use std::process::Command;

// func to run the command that was passed in 
fn _perfrom_command(command: String, args: Vec<&str>) {
    
    let mut child = Command::new(command)
        .args(args)
        .spawn()
        .expect("not a valid command");

   let s_code = child.wait()
       .expect("Failed to wait on child process.");

   // assert that the command succeded 
   assert!(s_code.success());
}

fn _print_type_of<T>(_: &T) {
    println!("{}", std::any::type_name::<T>())
}


// parse the command and split it into parts 
fn seperate_command(command: &String) -> (String, Vec<&str>) {

    // split the array by whitespace and than collet into &str array
    let mut parts: Vec<&str> = command.trim().split_whitespace().collect();

    // first index of parts == command
    let cmd = parts[0];

    // remove the first index and than assign the array to args
    parts.retain(|&x| x != cmd);
    let args = parts;
    
    return (cmd.to_string(), args);
    
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

        let mut cmd = String::new();
        let mut args = Vec::new();

        // call perfrom_cammand() with the correct params;
        let command_string = command.to_string();    
        (cmd, args) = seperate_command(&command_string);

        _perfrom_command(cmd, args);

    }
}
