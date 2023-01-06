use std::env;
use std::io::Error;
use clap::CommandFactory;
use clap_complete::generate_to;
use clap_complete::Shell::{Bash, Elvish, Fish, PowerShell, Zsh};
include!("src/args.rs");

fn main() -> Result<(), Error> {
    let out_dir = &match env::var_os("OUT_DIR") {
        Some(out_dir) => out_dir,
        None => return Ok(()),
    };

    let mut args = Args::command();
    generate_to(Bash, &mut args, "boom_you_up_r", out_dir)?;
    generate_to(Elvish, &mut args, "boom_you_up_r", out_dir)?;
    generate_to(Fish, &mut args, "boom_you_up_r", out_dir)?;
    generate_to(PowerShell, &mut args, "boom_you_up_r", out_dir)?;
    generate_to(Zsh, &mut args, "boom_you_up_r", out_dir)?;

    Ok(())
}
