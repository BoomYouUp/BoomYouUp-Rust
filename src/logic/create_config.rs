use crate::error::NormalError::{Cancelled, Input, NumberFormat};
use crate::error::{DetailedResult, FinalResult};
use crate::structs::item::{Command, Item, Time};
use crate::{add_command, normal_unwrap, CONFIG_PATH};
use std::io::Write;
use std::str::SplitWhitespace;
use std::{fs, io};
use crate::logic::enter::enter;

pub fn create_config() -> FinalResult {
    println!("请选择配置方式");
    println!("1. 输入所有参数进行配置");
    println!("2. 交互式配置");
    print!("请输入: ");
    io::stdout().flush()?;

    let mut input = String::new();
    io::stdin().read_line(&mut input)?;
    let input = input.trim();

    let result = match input {
        "1" => create_with_all_parameters(),
        "2" => create_config_by_interactive(),
        _ => { Ok(Err(Input)) },
    }?;

    match result {
        Ok(()) => return enter(),
        Err(_) => normal_unwrap!(result),
    }

    create_config()
}

fn create_with_all_parameters() -> DetailedResult {
    println!("接下来请在窗口中输入所有参数进行配置, 每行一个, 格式: ");
    println!();
    println!("时间 是否使用内置播放器 发送通知 文件路径 参数");
    println!();
    println!("时间");
    println!("  24 小时制时间, 格式 (注意使用空格分开): ");
    println!("  时 分 秒");
    println!();
    println!("  示例");
    println!("    7 30 0");
    println!("    07 30 00");
    println!();
    println!("是否使用内置播放器");
    println!("  接受 true/false yes/no y/n 1/0 的任意一种输入 (包括任意大小写组合)");
    println!();
    println!("发送通知");
    println!("  若不发送, 接受 false no n 0 的任意一种输入 (包括任意大小写组合)");
    println!("  若发送, 请输入以秒为单位的发送通知提前的时间");
    println!();
    println!("文件路径");
    println!("  接受任意文件或命令路径, 注意使用相对路径时的起始路径 (一般为程序所在目录)");
    println!();
    println!("参数 (可选)");
    println!("  要传入的参数, 使用空格分开");
    println!();
    println!("示例");
    println!("12 00 00 n n C:\\Users\\Administrator\\Desktop\\午夜凶铃.mp5");
    println!("在中午 12 点整使用合适的程序打开 Administrator 桌面上的午夜凶铃.mp5 文件");
    println!();
    println!("11 45 14 n 0 Z:\\只因你太美.mp4");
    println!("在 11 点 45 分 14 秒打开 Z 盘下的只因你太美.mp4 文件, 同时发送一则通知, 并发现其内容为 Rick Roll");
    println!();
    print!(
        "小提示: 如果你不知道文件路径是什么, 可以在添加文件时将文件拖进这个控制台窗口 (一般是一个黑框框) "
    );
    println!("或者选中文件并按 Ctrl+Shift+C 复制 (Windows)");
    println!("小提示: 按 Ctrl+C 中止程序");
    println!("小提示: 你可以在同一个时间指定多个任务");
    println!();
    println!();
    println!("控制命令");
    println!("`114514    保存并退出");
    println!("`1919810   重新选择配置方式");
    println!("`d [index] 删除指定序号的项目");
    println!("`e [index] 编辑指定序号的项目");
    println!();
    println!("下面请开始你的表演");
    println!();

    let mut index: usize = 0;
    let mut config: Vec<Item> = Vec::new();

    loop {
        println!("index: {index}");

        let mut input = String::new();
        io::stdin().read_line(&mut input)?;

        let input = input.trim();

        if input.starts_with('`') {
            let mut input = input.split_whitespace();
            let command = &input.next().unwrap_or("`")[1..];

            match command {
                "114514" => break,
                "1919810" => return Ok(Err(Cancelled)),
                "d" => {
                    match input.next() {
                        Some(index) => match index.parse::<usize>() {
                            Ok(index) => normal_unwrap!(delete_item(&mut config, index)?),
                            Err(_) => eprintln!("输入的序号不是一个有效数字"),
                        },
                        None => eprintln!("缺少参数：序号"),
                    }
                    continue;
                }
                "e" => {
                    match input.next() {
                        Some(index) => match index.parse::<usize>() {
                            Ok(index) => normal_unwrap!(edit_item(&mut config, index)?),
                            Err(_) => eprintln!("输入的序号不是一个有效数字"),
                        },
                        None => eprintln!("缺少参数: 序号"),
                    }
                    continue;
                }
                _ => eprintln!("未知的命令"),
            }
        }

        let input = input.split_whitespace();

        let mut time = Time::default();
        let mut command = Command::default();

        normal_unwrap!(parse_item(input, &mut time, &mut command)?);

        let len = config.len();
        if len != 0 && config[len - 1].time == time {
            config[len - 1].commands.push(command);
            continue;
        }

        add_command!(&mut config, time, command);
    }

    fs::write(CONFIG_PATH, serde_yaml::to_string(&config)?)?;

    println!();
    println!("很好, 配置已写入文件, 正在尝试读取...");
    println!();

    Ok(Ok(()))
}

fn parse_item(mut input: SplitWhitespace, time: &mut Time, command: &mut Command) -> DetailedResult {
    normal_unwrap!(parse_time(&mut input, time)?);

    match input.next() {
        Some(audio) => match audio.to_lowercase().as_str() {
            "y" | "1" | "yes" | "true" => command.audio = true,
            _ => {}
        },
        None => return Ok(Err(Input)),
    }

    match input.next() {
        Some(notify) => {
            if let Ok(notify) = notify.parse::<isize>() {
                command.notify = notify;
            }
        }
        None => return Ok(Err(Input)),
    }

    match input.next() {
        Some(cmd) => command.command = String::from(cmd),
        None => return Ok(Err(Input)),
    }

    command.parameters = input.collect::<Vec<_>>().join(" ");

    Ok(Ok(()))
}

fn parse_time(input: &mut SplitWhitespace, time: &mut Time) -> DetailedResult {
    match input.next() {
        Some(h) => match h.parse::<u8>() {
            Ok(h) => {
                if h >= 24 {
                    return Ok(Err(NumberFormat));
                } else {
                    time.hour = h;
                }
            }
            Err(_) => return Ok(Err(NumberFormat)),
        },
        None => return Ok(Err(Input)),
    }

    match input.next() {
        Some(m) => match m.parse::<u8>() {
            Ok(m) => {
                if m >= 60 {
                    return Ok(Err(NumberFormat));
                } else {
                    time.minute = m;
                }
            }
            Err(_) => return Ok(Err(NumberFormat)),
        },
        None => return Ok(Err(Input)),
    }

    match input.next() {
        Some(s) => match s.parse::<u8>() {
            Ok(s) => {
                if s >= 60 {
                    return Ok(Err(NumberFormat));
                } else {
                    time.second = s;
                }
            }
            Err(_) => return Ok(Err(NumberFormat)),
        },
        None => return Ok(Err(Input)),
    }

    Ok(Ok(()))
}

fn delete_item(config: &mut Vec<Item>, index: usize) -> DetailedResult {
    Ok(Ok(()))
}

fn edit_item(config: &mut Vec<Item>, index: usize) -> DetailedResult {
    Ok(Ok(()))
}

fn create_config_by_interactive() -> DetailedResult {
    println!("欢迎使用交互式配置创建器");
    println!();

    let mut index: usize = 0;
    let mut config: Vec<Item> = Vec::new();

    loop {
        let mut input = String::new();

        println!("index: {index}");
        print!("请输入时间 (时 分 秒): ");
        io::stdout().flush()?;

        io::stdin().read_line(&mut input)?;
    }

    Ok(Ok(()))
}
