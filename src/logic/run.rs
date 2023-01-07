use chrono::{Local, Timelike};
use std::io::{stdout, Write};
use std::path::PathBuf;
use std::time::Duration;
use std::{fs, thread};

use crate::error::{FinalResult, PrintingArgs, ResultPrinting};
use crate::logic::functions::{execute, play_audio, send_notification};
use crate::structs::item::{AddCommand, Command, Item, Time};

pub fn run(config_path: &PathBuf) -> FinalResult {
    let mut config = serde_yaml::from_str::<Vec<Item>>(&fs::read_to_string(config_path)?)?;

    config.sort_unstable_by(|a, b| a.time.cmp(&b.time));
    match fs::write(config_path, serde_yaml::to_string(&config)?) {
        Ok(_) => {}
        Err(e) => {
            eprintln!("配置重新写入时遇到了错误: {}", e);
        }
    }

    print_config(&config);
    parse_config(&mut config);

    println!();

    let mut time = Time::default();
    let mut next: &Item = &config[0];

    update_system_time(&mut time);
    update_next(&config, time, &mut next);

    loop {
        if time == next.time {
            for command in &next.commands {
                if command.notify == -2 {
                    let command = command.clone();
                    let parameters;
                    println!(
                        "为命令：{}{} 发送通知",
                        command.command,
                        if command.parameters.is_empty() {
                            ""
                        } else {
                            parameters = format!(" 参数：{}", command.parameters);
                            &parameters
                        }
                    );

                    thread::spawn(move || {
                        send_notification(&command.command)
                            .result_println(PrintingArgs::message("发送通知时遇到了问题"));
                    });
                } else if command.audio {
                    let path = command.command.clone();
                    println!("播放音频：{}", path);

                    thread::spawn(move || {
                        play_audio(PathBuf::from(&path))
                            .result_println(PrintingArgs::message("播放音频时遇到了问题"));
                    });
                } else {
                    let command = command.clone();
                    let parameters_string;
                    println!(
                        "执行命令：{}{}",
                        command.command,
                        if command.parameters.is_empty() {
                            ""
                        } else {
                            parameters_string = format!(" 参数：{}", command.parameters);
                            &parameters_string
                        }
                    );

                    thread::spawn(move || {
                        execute(
                            &command.command,
                            Some(
                                command
                                    .parameters
                                    .split_whitespace()
                                    .map(|s| s.to_string())
                                    .collect(),
                            ),
                        )
                            .result_println(PrintingArgs::message("执行命令时遇到了问题"));
                    });
                }
                println!();
            }

            stdout().flush()?;
            update_next(&config, time, &mut next);
        }

        thread::sleep(Duration::from_secs(1));
        update_system_time(&mut time);
    }
}

fn print_config(config: &Vec<Item>) {
    println!("配置解析中，配置如下：");

    for item in config {
        print!("{} ", item.time);

        let mut width = 0;
        for command in &item.commands {
            if width != 0 {
                println!();
            }

            println!("{:>width$}命令：{}", "", command.command, width = width);

            if width == 0 {
                width = 9;
            }

            println!(
                "{:>width$}参数：{}",
                "",
                if command.parameters.is_empty() {
                    "无"
                } else {
                    &command.parameters
                },
                width = width
            );

            println!(
                "{:>width$}音频：{}",
                "",
                if command.audio { "是" } else { "否" },
                width = width
            );

            let notify_str;
            println!(
                "{:>width$}发送通知：{}",
                "",
                match command.notify {
                    ..=-1 => "否",
                    0 => "开始运行时",
                    _ => {
                        let time = item.time - Time::second(command.notify as usize);
                        notify_str = format!("开始运行的 {} 秒之前，即 {}", command.notify, time);
                        &notify_str
                    }
                },
                width = width
            );
        }
    }
}

fn parse_config(config: &mut Vec<Item>) {
    let mut result: Vec<Item> = Vec::new();

    for item in &mut *config {
        let reference = &mut result;

        reference.push(item.clone());

        for j in 0..item.commands.len() {
            if item.commands[j].notify != -1 {
                reference.add_command_reverse(
                    item.time - Time::second(item.commands[j].notify as usize),
                    Command {
                        command: item.commands[j].command.clone(),
                        parameters: item.commands[j].parameters.clone(),
                        notify: -2,
                        ..item.commands[j]
                    },
                )
            }
        }
    }

    *config = result;
}

fn update_next<'a>(config: &'a Vec<Item>, time: Time, next: &mut &'a Item) {
    for i in 0..config.len() {
        if config[i].time > time {
            *next = &config[i];
            break;
        }

        if i == config.len() - 1 {
            *next = &config[0];
        }
    }

    println!("下一次执行时间：{}", next.time);
}

fn update_system_time(time: &mut Time) {
    let system_time = Local::now();

    *time = Time {
        hour: system_time.hour() as u8,
        minute: system_time.minute() as u8,
        second: system_time.second() as u8,
    };
}
