pub mod utilities;
pub(crate) mod error;
mod command;
use anyhow::{Result, Context};

use crate::{command::Command, error::CustomError, utilities::{exit, get_command, print_error, print_prompt}};

pub fn run() -> Result<()> {
    loop {
        print_prompt();        
        
        let command = get_command().context("getting command")?;
        
        match command {
            Command::Exit => exit(0),
            Command::NotFound(command_string) => {
                let error_message = CustomError::CommandNotFound(command_string);
                print_error(error_message);
            }
        }
    }

    #[allow(unreachable_code)]
    Ok(())
}