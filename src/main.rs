use crate::logic::enter::enter;

mod logic {
    pub mod create_config;
    pub mod enter;
    pub mod start;
}
mod structs {
    pub mod item;
}
mod utils {
    pub mod player;
}

static CONFIG_PATH: &str = "config.yaml";

static APP_NAME: &str = "BoomYouUpR";
static APP_CHINESE_NAME: &str = "炸你起床R";

fn main() -> Result<(), std::io::Error> {
    println!(
        "{} {} 版本 {} 南科大附中 胡睿邈 于 2020-12-25 编写",
        APP_NAME,
        APP_CHINESE_NAME,
        env!("CARGO_PKG_VERSION")
    );

    enter()
}
