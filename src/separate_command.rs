pub fn separate_command(command: &String) -> (String, Vec<&str>) {
    let mut parts: Vec<&str> = command.trim().split_whitespace().collect();

    let cmd = parts[0];

    parts.retain(|&x| x != cmd);
    let args = parts;

    return (cmd.to_string(), args);
}
