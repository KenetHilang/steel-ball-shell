pub enum Command {
    Exit,
    Echo(Vec<String>),
    NotFound(String),
}

impl From<(String, Vec<String>)> for Command {
    fn from((command, arguments): (String, Vec<String>)) -> Self {
        match command.as_str() {
            "echo" => Self::Echo(arguments),
            "exit" => Self::Exit,
            _ => Self::NotFound(command.to_owned())
        }
    }
}

