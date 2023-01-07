use clap::CommandFactory;
use clap_complete::generate_to;
use clap_complete::Shell::{Bash, Elvish, Fish, PowerShell, Zsh};
use std::env;
use std::io::Error;

include!("src/args.rs");

fn main() -> Result<(), Error> {
    let output_path = &match env::var("OUT_DIR") {
        Ok(out_dir) => out_dir,
        Err(_) => return Ok(()),
    };

    let mut args = Args::command();
    generate_to(Bash, &mut args, "boom_you_up_r", output_path)?;
    generate_to(Elvish, &mut args, "boom_you_up_r", output_path)?;
    generate_to(Fish, &mut args, "boom_you_up_r", output_path)?;
    generate_to(PowerShell, &mut args, "boom_you_up_r", output_path)?;
    generate_to(Zsh, &mut args, "boom_you_up_r", output_path)?;

    println!(
        "cargo:warning=Generated completion scripts in {}.",
        output_path
    );

    Ok(())
}
