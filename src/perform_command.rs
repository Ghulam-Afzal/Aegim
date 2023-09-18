use std::process::Command;

// func to run the command that was passed in 
pub fn perform_command(command: String, args: Vec<&str>) {
    
    let mut child = Command::new(command)
        .args(args)
        .spawn()
        .expect("not a valid command");

   let s_code = child.wait()
       .expect("Failed to wait on child process.");

   // assert that the command succeeded 
   assert!(s_code.success());
}