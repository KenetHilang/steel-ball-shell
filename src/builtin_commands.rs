pub mod echo;
pub mod builtin_type;

pub type CommandArguments = Vec<String>;

pub enum BuiltinCommand {
    Exit,
    Echo(CommandArguments),
    Type(CommandArguments),
    NotFound(String),
}

impl From<(String, CommandArguments)> for BuiltinCommand {
    fn from((command, arguments): (String, CommandArguments)) -> Self {
        match command.as_str() {
            "echo" => Self::Echo(arguments),
            "type" => Self::Type(arguments),
            "exit" => Self::Exit,
            _ => Self::NotFound(command.to_owned())
        }
    }
}

impl From<String> for BuiltinCommand {
    fn from(command:String) -> Self {
        let arguments = vec![];
        Self::from((command, arguments))
    }
}