pub use std::process::exit;
use std::{env::{self, split_paths}, path::PathBuf, io::{self, Write}, fmt::Display};
use anyhow::{Context, Ok, Result, bail};
use crate::builtin_commands::{BuiltinCommand};

pub fn print_prompt() {
    print!("ᯓ★ ");
    io::stdout().flush().unwrap();
}

pub fn get_input() -> Result<String> {
    let mut user_input = String::new();
    io::stdin().read_line(&mut user_input).context("Reading User Input")?;
    Ok(user_input.trim().to_owned())
}

pub fn get_command() -> Result<BuiltinCommand> {
    let user_input = get_input()?;

    let mut parsed_user_input = user_input.split_whitespace();
    let command = parsed_user_input.next().unwrap_or(" ").to_owned();
    let arguments = parsed_user_input.map(ToOwned:: to_owned).collect::<Vec<String>>();

    let command = BuiltinCommand::from((command, arguments));
    Ok(command)
}

pub fn print_error(message: impl Display) {
    eprintln!("{}", message);
}

pub fn get_path() -> Result<Vec<PathBuf>> {
    let path = env::var("PATH").context("Getting Path Variable")?;
    let split_paths = split_paths(&path).map(|path| {
        if path.is_file() {
            bail!("PATH from environment variable is a file");
        } else {
            Ok(path)
        }
    });

    split_paths.collect()
}