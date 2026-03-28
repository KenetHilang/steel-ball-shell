pub mod utilities;
pub mod error;
mod builtin_commands;
use anyhow::{Result, Context};


use crate::{
    builtin_commands::{BuiltinCommand, builtin_type::builtin_type, echo::echo}, 
    error::CustomError, 
    utilities::{get_command, get_path, print_error, print_prompt}
};

pub fn run() -> Result<()> {

    let path = get_path().context("Getting Path")?;

    loop {
        print_prompt();        
        
        let command = get_command().context("getting command")?;
        
        match command {
            BuiltinCommand::Echo(command) => echo(command.as_slice()),
            BuiltinCommand::Type(arguments) => builtin_type(arguments, &path),
            BuiltinCommand::Exit => break,
            BuiltinCommand::NotFound(command) => {
                let error_message = CustomError::CommandNotFound(command);
                print_error(error_message);
            }
        }
    }

    Ok(())
}