pub mod utilities;
pub mod error;
mod commands;
use anyhow::{Result, Context};


use crate::{
    commands::{Command, builtin_type::builtin_type, echo::echo}, 
    error::CustomError, 
    utilities::{get_command, print_error, print_prompt}
};

pub fn run() -> Result<()> {
    loop {
        print_prompt();        
        
        let command = get_command().context("getting command")?;
        
        match command {
            Command::Echo(command) => echo(&command),
            Command::Type(arguments) => builtin_type(arguments),
            Command::Exit => break,
            Command::NotFound(command) => {
                let error_message = CustomError::CommandNotFound(command);
                print_error(error_message);
            }
        }
    }

    Ok(())
}