use std::io::{stdin, stdout, Write};

use crate::error::FinalResult;

pub fn print(message: &str) -> FinalResult {
    print!("{message}");
    stdout().flush()?;

    Ok(())
}

pub fn print_and_readln(message: &str) -> FinalResult<String> {
    print(message)?;

    let mut input = String::new();
    stdin().read_line(&mut input)?;

    Ok(input.trim().to_string())
}
