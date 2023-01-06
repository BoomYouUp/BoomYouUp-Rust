use clap::Parser;

use args::Actions::{Configure, Run, Test};
use error::PrintingArgs;
use logic::create_config::create_config;
use logic::functions::{execute, play_audio, send_notification};
use logic::run::run;
use Functions::{Execute, PlayAudio, SendNotification};

use crate::args::{Args, Functions};
use crate::error::ResultPrinting;

mod args;
mod error;
mod logic;
mod structs;
mod utils;

static APP_NAME: &str = "BoomYouUpR";

fn main() {
    match Args::parse().action {
        Run { config } => run(&config),
        Configure { config } => create_config(&config),
        Test { function } => {
            match function {
                Execute {
                    command,
                    parameters,
                } => execute(&command, parameters),
                PlayAudio { path } => play_audio(path),
                SendNotification => send_notification("测试"),
            }
            .result_println(PrintingArgs::normal());
            Ok(())
        }
    }
    .result_println(PrintingArgs::new());
}
