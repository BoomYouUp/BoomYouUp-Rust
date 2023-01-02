use crate::error::FinalResult;
use std::io::{stdin, stdout, Write};

pub fn print_and_readln(message: &str) -> FinalResult<String> {
    print!("{message}");
    stdout().flush()?;

    let mut input = String::new();
    stdin().read_line(&mut input)?;

    Ok(input)
}
