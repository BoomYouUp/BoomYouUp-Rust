use fs::remove_file;
use std::io::Write;
use std::{fs, io};

use crate::error::{FinalResult, UnexpectedError};
use crate::logic::create_config::create_config;
use crate::logic::run::run;
use crate::structs::item::Item;
use crate::CONFIG_PATH;
use crate::utils::stdio::print_and_readln;

pub fn enter() -> FinalResult {
    let config_str = fs::read_to_string(CONFIG_PATH);

    if let Err(e) = config_str {
        return match e.kind() {
            io::ErrorKind::NotFound => create_config(),
            _ => {
                eprintln!("配置读取错误: {}", e);
                Err(UnexpectedError::from(e))
            }
        };
    }

    let config_str = config_str?;

    match serde_yaml::from_str::<Vec<Item>>(&config_str) {
        Ok(config) => {
            if config.is_empty() {
                return create_config();
            }
        }
        Err(e) => {
            eprintln!("配置读取错误：{}", e);
            return create_config();
        }
    }

    println!("请问你想作甚?");
    println!("1. 开始运行");
    println!("2. 重新配置");
    match print_and_readln("请输入：")?.as_str() {
        "1" => run(),
        "2" => reconfigure(),
        _ => enter(),
    }
}

fn reconfigure() -> FinalResult {
    print!(
        "你的所有配置都会被清空。如需更改配置，请打开 {} 。确认重新配置？（y/N）",
        CONFIG_PATH
    );
    io::stdout().flush()?;

    let mut input = String::new();
    io::stdin().read_line(&mut input)?;

    match input.trim() {
        "Y" | "y" => {
            remove_file(CONFIG_PATH)?;
            create_config()
        }
        _ => enter(),
    }
}
