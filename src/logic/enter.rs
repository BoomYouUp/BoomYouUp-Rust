use crate::logic::create_config::create_config;
use crate::logic::start::start;
use crate::structs::item::Item;
use crate::CONFIG_PATH;
use std::io::{Error, ErrorKind, Write};
use std::{fs, io};

pub fn enter() -> Result<(), Error> {
    let config_str = fs::read_to_string(CONFIG_PATH);

    if let Err(e) = config_str {
        match e.kind() {
            ErrorKind::NotFound => return create_config(),
            _ => panic!("配置读取错误：{:?}", e),
        }
    }

    let config_str = config_str.expect("未知错误");

    match serde_yaml::from_str::<Vec<Item>>(&config_str) {
        Err(e) => {
            eprintln!("配置读取错误：{:?}", e);
            return create_config();
        }
        Ok(config) => {
            if config.is_empty() {
                return create_config();
            }
        }
    }

    println!("请问你想作甚？");
    println!("1. 开始运行");
    println!("2. 重新配置");
    print!("请输入：");
    io::stdout().flush()?;

    let mut input = String::new();
    io::stdin().read_line(&mut input)?;

    let input = input.trim();

    match input {
        "1" => start(),
        "2" => reconfigure(),
        _ => {
            println!("输入错误，请重新输入！");
            enter()
        }
    }
}

fn reconfigure() -> Result<(), Error> {
    print!(
        "确认重新配置？你的所有配置都会被清空。如需更改配置，请打开 {} [Y/n] ",
        CONFIG_PATH
    );
    io::stdout().flush()?;

    let mut input = String::new();
    io::stdin().read_line(&mut input)?;

    let input = input.trim();

    match input {
        "Y" | "y" => {
            fs::remove_file(CONFIG_PATH)?;
            create_config()
        }
        _ => reconfigure(),
    }
}
