use std::io::{stdout, Write};
use std::path::PathBuf;
use std::{fs, thread};

use crate::error::{FinalResult, PrintingArgs, ResultPrinting};
use crate::logic::functions::{execute, play_audio, send_notification};
use crate::structs::config::{Config, Item};

pub fn run(config_path: &PathBuf) -> FinalResult {
    let mut config = Config::new(serde_yaml::from_str::<Vec<Item>>(&fs::read_to_string(
        config_path,
    )?)?);

    match fs::write(config_path, serde_yaml::to_string(&config.items)?) {
        Ok(_) => {}
        Err(e) => {
            eprintln!("配置重新写入时遇到了错误: {}", e);
        }
    }

    config.print();
    config.parse_notification();

    println!();

    loop {
        let (next, duration) = config.next();
        println!("下一次执行时间：{}", next.time);
        thread::sleep(duration);

        for command in &next.commands {
            if command.notify == -2 {
                let command = command.clone();
                let parameters;
                println!(
                    "为命令 {}{}发送通知",
                    command.command,
                    if command.parameters.is_empty() {
                        " "
                    } else {
                        parameters = format!("（参数：{}）", command.parameters);
                        &parameters
                    }
                );

                thread::spawn(move || {
                    send_notification(&command.command)
                        .result_println(PrintingArgs::customized("发送通知时遇到了问题"));
                });
            } else if command.audio {
                let path = command.command.clone();
                println!("播放音频 {}", path);

                thread::spawn(move || {
                    play_audio(PathBuf::from(&path))
                        .result_println(PrintingArgs::customized("播放音频时遇到了问题"));
                });
            } else {
                let command = command.clone();
                let parameters_string;
                println!(
                    "执行命令 {}{}",
                    command.command,
                    if command.parameters.is_empty() {
                        ""
                    } else {
                        parameters_string = format!("（参数：{}）", command.parameters);
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
                    .result_println(PrintingArgs::customized("执行命令时遇到了问题"));
                });
            }
            println!();
        }

        stdout().flush()?;
    }
}
