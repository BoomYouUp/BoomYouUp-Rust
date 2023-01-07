use clap::CommandFactory;
use clap_complete::generate_to;
use clap_complete::Shell::{Bash, Elvish, Fish, PowerShell, Zsh};
use std::{env, fs};
use std::io::Error;
use std::path::Path;
include!("src/args.rs");

fn main() -> Result<(), Error> {
    let manifest_dir = env::var("CARGO_MANIFEST_DIR").unwrap();
    let build_type = env::var("PROFILE").unwrap();
    let output_path = Path::new(&manifest_dir)
        .join("target")
        .join(build_type)
        .join("completion_scripts");

    println!(
        "cargo:warning=Generating completions into {}",
        output_path.display()
    );
    fs::create_dir_all(&output_path)?;

    let mut args = Args::command();
    generate_to(Bash, &mut args, "boom_you_up_r", &output_path)?;
    generate_to(Elvish, &mut args, "boom_you_up_r", &output_path)?;
    generate_to(Fish, &mut args, "boom_you_up_r", &output_path)?;
    generate_to(PowerShell, &mut args, "boom_you_up_r", &output_path)?;
    generate_to(Zsh, &mut args, "boom_you_up_r", &output_path)?;

    println!(
        "cargo:warning=Generated completion scripts in {}.",
        output_path.display()
    );

    Ok(())
}
