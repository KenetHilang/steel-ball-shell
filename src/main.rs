use std::process;
use steel_ball_shell::run;

fn main() {
    match run() {
        Ok(()) => process::exit(0),

        Err(error) => {
            eprintln!("Error: {error}");
            process::exit(1);
        }
    }
}