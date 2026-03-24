mod utilities;
pub(crate) mod error;

use std::io::{self, Write};

use anyhow::Result;

use crate::{error::CustomError, utilities::{get_input, print_error}};

pub fn run() -> Result<()> {
    print!("ᯓ★  ");

    io::stdout().flush().unwrap();

    let user_input: String = get_input()?;
    let error_message = CustomError::CommandNotFound(user_input);
    

    print_error(error_message);

    Ok(())
}