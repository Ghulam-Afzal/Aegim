pub enum Command {
    Normal(String),
    Cd(Vec<String>),
    Exit,
    Pwd,
}

impl Command {
    pub fn from_str(command: &str, args: Vec<&str>) -> Self {
        match command.trim().to_lowercase().as_str() {
            "exit" => Command::Exit,
            "cd" => Command::Cd(args.iter().map(|&s| s.to_string()).collect()),
            "pwd" => Command::Pwd,
            other => Command::Normal(other.to_string()),
        }
    }
}
