mod utilities;
pub(crate) mod error;
mod command;

use std::io::{self, Write};
use std::process;

use anyhow::Result;

use crate::{command::Command, error::CustomError, utilities::{get_input, print_error}};

pub fn run() -> Result<()> {
    loop {
        print!("ᯓ★ ");

        io::stdout().flush().unwrap();

        let user_input: String = get_input()?;
        let command = Command::from(user_input.as_str());
        
        
        match command {
            Command::Exit => process::exit(0),
            Command::NotFound(command_string) => {
                let error_message = CustomError::CommandNotFound(command_string);
                print_error(error_message);
            }
        }
    }

    #[allow(unreachable_code)]
    Ok(())
}