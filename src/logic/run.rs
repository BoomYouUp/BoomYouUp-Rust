use std::{fs, thread};
use std::ffi::OsStr;
use std::io::Write;
use std::path::Path;
use std::time::Duration;

use chrono::{Local, Timelike};
use notify_rust::Notification;
use opener::open;
use soloud::{AudioExt, LoadExt, Soloud, Wav};

use crate::{APP_NAME, CONFIG_PATH};
use crate::error::FinalResult;
use crate::structs::item::{AddCommand, Command, Item, Time};

pub fn run() -> FinalResult {
    let mut config = serde_yaml::from_str::<Vec<Item>>(&fs::read_to_string(CONFIG_PATH)?)?;

    config.sort_unstable_by(|a, b| a.time.cmp(&b.time));
    match fs::write(CONFIG_PATH, serde_yaml::to_string(&config)?) {
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

                    let result = Notification::new()
                        .appname(APP_NAME)
                        .summary("任务提醒")
                        .body(&format!(
                            "你为命令 {} 设置的提醒触发了\n来自 {}",
                            command.command, APP_NAME
                        ))
                        .show();

                    if let Err(e) = result {
                        eprintln!("通知发送失败：{}", e);
                    }
                } else if command.audio {
                    println!("播放音频：{}", command.command);
                    let clone = command.command.clone();
                    thread::spawn(move || match Soloud::default() {
                        Ok(player) => {
                            let mut wav = Wav::default();
                            match wav.load(Path::new(&clone)) {
                                Ok(_) => {
                                    player.play(&wav);

                                    while player.active_voice_count() > 0 {
                                        thread::sleep(Duration::from_millis(100));
                                    }
                                }
                                Err(e) => eprintln!("音频解码失败：{}", e),
                            }
                        }
                        Err(e) => eprintln!("音频播放器创建失败：{}", e),
                    });
                } else {
                    let parameters;
                    println!(
                        "执行命令：{}{}",
                        command.command,
                        if command.parameters.is_empty() {
                            ""
                        } else {
                            parameters = format!(" 参数：{}", command.parameters);
                            &parameters
                        }
                    );

                    if let Err(e) = open(
                        OsStr::new(&command.command),
                        OsStr::new(&command.parameters),
                    ) {
                        eprintln!("命令执行错误：{}", e);
                    }
                }
                println!();
            }

            std::io::stdout().flush()?;
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
