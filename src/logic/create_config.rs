use std::fs;
use std::io::Error;

use crate::logic::start::start;
use crate::structs::item::{Command, Item, Time};
use crate::{add_command, CONFIG_PATH};

pub fn create_config() -> Result<(), Error> {
    println!("配置为空或读取失败，请填上它！");
    println!();
    println!("接下来请在窗口中输入时间与文件路径完善配置，每行一个，格式：");
    println!();
    println!("时 分 秒 文件路径");
    println!();
    println!("时间为 24 小时制，注意用空格分开");
    println!("文件路径，能打开就行");
    println!();
    println!("例子：");
    println!("12 00 00 C:\\Users\\Administrator\\Desktop\\午夜凶铃.mp5");
    println!("在中午 12 点整打开 Administrator 桌面上的午夜凶铃.mp5 文件");
    println!();
    println!("11 45 14 Z:\\只因你太美.mp4");
    println!("在 11 点 45 分 14 秒打开 Z 盘下的只因你太美.mp4 文件并发现其内容为 Rick Roll");
    println!();
    println!("添加完毕后请输入 114514 并按回车");
    print!(
        "如果你不知道文件路径是什么，可以在添加文件时将文件拖进这个控制台窗口（一般是一个黑框框）"
    );
    println!("或者选中文件并按 Ctrl+Shift+C 复制");
    println!();
    println!("小提示：按 Ctrl+C 中止程序");
    println!("小提示：你可以在同一个时间指定多个任务");
    println!();
    println!("下面请开始你的表演：");

    let mut config: Vec<Item> = Vec::new();

    loop {
        let mut input = String::new();
        std::io::stdin().read_line(&mut input)?;

        let input = input.trim();

        if input == "114514" {
            break;
        }

        let mut input = input.split_whitespace();
        let mut error = CreateConfigError::None;

        let mut time = Time {
            hour: 0,
            minute: 0,
            second: 0,
        };

        let mut command = Command {
            command: String::from(""),
            parameters: String::from(""),
            audio: false,
            notify: -1,
        };

        match input.next() {
            None => error = CreateConfigError::StdIn,
            Some(h) => match h.parse::<u8>() {
                Err(_) => error = CreateConfigError::NumberFormat,
                Ok(h) => {
                    if h >= 24 {
                        error = CreateConfigError::NumberFormat;
                    } else {
                        time.hour = h;
                    }
                }
            },
        }

        if error == CreateConfigError::None {
            match input.next() {
                None => error = CreateConfigError::StdIn,
                Some(m) => match m.parse::<u8>() {
                    Err(_) => error = CreateConfigError::NumberFormat,
                    Ok(m) => {
                        if m >= 60 {
                            error = CreateConfigError::NumberFormat;
                        } else {
                            time.minute = m;
                        }
                    }
                },
            }
        }

        if error == CreateConfigError::None {
            match input.next() {
                None => error = CreateConfigError::StdIn,
                Some(s) => match s.parse::<u8>() {
                    Err(_) => error = CreateConfigError::NumberFormat,
                    Ok(s) => {
                        if s >= 60 {
                            error = CreateConfigError::NumberFormat;
                        } else {
                            time.second = s;
                        }
                    }
                },
            }
        }

        if error == CreateConfigError::None {
            match input.next() {
                None => error = CreateConfigError::StdIn,
                Some(cmd) => command.command = String::from(cmd),
            }
        }

        if error == CreateConfigError::None {
            command.parameters = input.collect::<Vec<_>>().join(" ");
        }

        match error {
            CreateConfigError::StdIn => {
                eprintln!("读取输入错误，请重新输入");
                continue;
            }
            CreateConfigError::NumberFormat => {
                eprintln!("数字格式错误，请重新输入");
                continue;
            }
            _ => {}
        }

        let len = config.len();
        if len != 0 && config[len - 1].time == time {
            config[len - 1].commands.push(command);
            continue;
        }

        add_command!(&mut config, time, command);
    }

    config.sort_unstable_by(|a, b| a.time.cmp(&b.time));

    fs::write(
        CONFIG_PATH,
        serde_yaml::to_string(&config).expect("配置序列化错误"),
    )
    .expect("配置写入错误");

    println!();
    println!("很好，配置已写入文件，正在尝试读取...");
    println!();

    start()
}

#[derive(PartialEq)]
enum CreateConfigError {
    None,
    StdIn,
    NumberFormat,
}
