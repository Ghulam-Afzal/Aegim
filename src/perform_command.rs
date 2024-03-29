use std::process::Command;

pub fn perform_command(command: String, args: Vec<&str>) {
    let mut child = Command::new(command)
        .args(args)
        .spawn()
        .expect("not a valid command");

    match child.try_wait() {
        Ok(Some(status)) => println!("exited with: {status}"),
        Ok(None) => {
            let _ = child.wait();
        }
        Err(err) => {
            eprintln!("A error occured trying to perform the command {:?}", err);
        }
    }
}
