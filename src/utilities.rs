use std::{io::{self, Write}, fmt::Display, process};
use anyhow::{Context, Ok, Result};

use crate::command::Command;

pub fn print_prompt() {
    print!("ᯓ★ ");
    io::stdout().flush().unwrap();
}

pub fn get_input() -> Result<String> {
    let mut user_input: String = String::new();

    io::stdin().read_line(&mut user_input).context("Reading User Input")?;

    Ok(user_input.trim().to_owned())
}

pub fn get_command() -> Result<Command> {
    let user_input: String = get_input()?;
    let command = Command::from(user_input.as_str());

    Ok(command)
}

pub fn print_error(message: impl Display) {
    eprintln!("{}", message);
}

pub fn exit(code: i32){
    process::exit(code)
}