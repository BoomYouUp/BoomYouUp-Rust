use crate::structs::item::{Command, Item, Time};
use crate::{add_command_reverse, pcstr, APP_NAME, CONFIG_PATH};
use notify_rust::Notification;
use opener::open;
use rodio::{Decoder, OutputStream, Sink};
use std::ffi::OsStr;
use std::fs::File;
use std::io::{BufReader, Error};
use std::{fs, thread};
use windows::core::PCSTR;
use windows::Win32::Foundation::{GetLastError, HWND, SYSTEMTIME};
use windows::Win32::System::SystemInformation::GetLocalTime;
use windows::Win32::UI::Shell::ShellExecuteA;
use windows::Win32::UI::WindowsAndMessaging::SW_SHOWNORMAL;

pub fn start() -> Result<(), Error> {
    let mut config =
        serde_yaml::from_str::<Vec<Item>>(&fs::read_to_string(CONFIG_PATH).expect("配置读取错误"))
            .expect("配置读取错误");

    config.sort_unstable_by(|a, b| a.time.cmp(&b.time));
    fs::write(
        CONFIG_PATH,
        serde_yaml::to_string(&config).expect("配置序列化错误"),
    )
    .expect("配置写入错误");

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
                    let command_copy = command.command.clone();
                    thread::spawn(|| match OutputStream::try_default() {
                        Ok((_stream, handle)) => match Sink::try_new(&handle) {
                            Ok(sink) => match File::open(command_copy) {
                                Ok(file) => match Decoder::new(BufReader::new(file)) {
                                    Ok(source) => {
                                        sink.append(source);
                                        sink.sleep_until_end();
                                    }
                                    Err(e) => eprintln!("音频解码失败：{}", e),
                                },
                                Err(e) => eprintln!("音频文件打开失败：{}", e),
                            },
                            Err(e) => eprintln!("音频播放器创建失败：{}", e),
                        },
                        Err(e) => eprintln!("音频输出流打开失败：{}", e),
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
            update_next(&config, time, &mut next);
        }

        thread::sleep(std::time::Duration::from_secs(1));
        update_system_time(&mut time);
    }
}

fn print_config(config: &Vec<Item>) {
    println!("配置解析中，配置如下：");

    for item in config {
        print!(
            "{:02}:{:02}:{:02} ",
            item.time.hour, item.time.minute, item.time.second
        );

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
                    -1 => "否",
                    0 => "开始运行时",
                    _ => {
                        let time = item.time - Time::second(command.notify as usize);
                        notify_str = format!(
                            "开始运行的 {} 秒之前，即 {:02}:{:02}:{:02}",
                            command.notify, time.hour, time.minute, time.second
                        );
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
                add_command_reverse!(
                    reference,
                    item.time - Time::second(item.commands[j].notify as usize),
                    Command {
                        command: item.commands[j].command.clone(),
                        parameters: item.commands[j].parameters.clone(),
                        notify: -2,
                        ..item.commands[j]
                    }
                );
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

    println!(
        "下一次执行时间：{:02}:{:02}:{:02}",
        next.time.hour, next.time.minute, next.time.second
    );
}

fn update_system_time(time: &mut Time) {
    let mut system_time = SYSTEMTIME {
        wYear: 2022,
        wMonth: 12,
        wDayOfWeek: 7,
        wDay: 25,
        wHour: 11,
        wMinute: 45,
        wSecond: 14,
        wMilliseconds: 0,
    };

    unsafe {
        GetLocalTime(&mut system_time);
    }

    *time = Time {
        hour: system_time.wHour as u8,
        minute: system_time.wMinute as u8,
        second: system_time.wSecond as u8,
    };
}
