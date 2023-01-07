use clap::Parser;

use error::PrintingArgs;
use logic::create_config::create_config;
use logic::functions::{execute, play_audio, send_notification};
use logic::run::run;

use crate::args::{Actions, Args, Functions};
use crate::error::ResultPrinting;
use crate::logic::functions::time;

mod args;
mod error;
mod logic;
mod structs;
mod utils;

static APP_NAME: &str = "BoomYouUpR";

fn main() {
    match Args::parse().action {
        Actions::Run { config } => run(&config),
        Actions::Configure { config } => create_config(&config),
        Actions::Test { function } => {
            match function {
                Functions::Execute {
                    command,
                    parameters,
                } => execute(&command, parameters),
                Functions::PlayAudio { path } => play_audio(path),
                Functions::SendNotification => send_notification("测试"),
                Functions::Time {
                    hour,
                    minute,
                    second,
                } => time(hour, minute, second),
            }
            .result_println(PrintingArgs::normal().ok_message("测试成功"));
            Ok(())
        }
    }
    .result_println(PrintingArgs::unexpected());
}
