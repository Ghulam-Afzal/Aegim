// parse the command and split it into parts 
pub fn seperate_command(command: &String) -> (String, Vec<&str>) {

    // split the array by whitespace and than collet into &str array
    let mut parts: Vec<&str> = command.trim().split_whitespace().collect();

    // first index of parts == command
    let cmd = parts[0];

    // remove the first index and than assign the array to args
    parts.retain(|&x| x != cmd);
    let args = parts;
    
    return (cmd.to_string(), args);
    
}