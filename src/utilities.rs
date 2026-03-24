use std::{io, fmt::Display};
use anyhow::{Context, Result};

pub fn get_input() -> Result<String> {
    let mut user_input: String = String::new();

    io::stdin().read_line(&mut user_input).context("Reading User Input")?;

    Ok(user_input.trim().to_owned())
}

pub fn print_error(message: impl Display) {
    eprintln!("{}", message);
}