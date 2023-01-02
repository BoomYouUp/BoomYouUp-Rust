use crate::error::ResultPrinting;

mod error;
mod logic;
mod structs;
mod utils;

static CONFIG_PATH: &str = "config.yaml";

static APP_NAME: &str = "BoomYouUpR";
static APP_CHINESE_NAME: &str = "炸你起床R";

fn main() {
    println!(
        "{} {} 版本 {} 南科大附中 胡睿邈 于 2020-12-25 编写",
        APP_NAME,
        APP_CHINESE_NAME,
        env!("CARGO_PKG_VERSION")
    );

    logic::enter::enter().result_println(error::PrintingArgs::new());
}
