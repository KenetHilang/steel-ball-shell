pub mod utilities;
pub mod error;
mod command;
pub mod echo;
use anyhow::{Result, Context};

use echo::echo;
use crate::{command::Command, error::CustomError, utilities::{get_command, print_error, print_prompt}};

pub fn run() -> Result<()> {
    loop {
        print_prompt();        
        
        let command = get_command().context("getting command")?;
        
        match command {
            Command::Echo(command) => echo(command),
            Command::Exit => break,
            Command::NotFound(command_string) => {
                let error_message = CustomError::CommandNotFound(command_string);
                print_error(error_message);
            }
        }
    }

    Ok(())
}