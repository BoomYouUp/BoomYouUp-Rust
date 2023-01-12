use chrono::{Local, Timelike};
use std::path::PathBuf;
use std::thread;

use notify_rust::Notification;
use opener::open;
use soloud::{AudioExt, LoadExt, Soloud, Wav};

use crate::error::NormalResult;
use crate::structs::config::Time;
use crate::APP_NAME;

pub fn execute(command: &String, parameters: Option<Vec<String>>) -> NormalResult {
    open(command, parameters.unwrap_or_default().join(" "))?;

    Ok(())
}

pub fn play_audio(path: PathBuf) -> NormalResult {
    let player = Soloud::default()?;
    let mut wav = Wav::default();
    wav.load(path)?;
    player.play(&wav);

    while player.active_voice_count() > 0 {}

    Ok(())
}

pub fn send_notification(command: &str) -> NormalResult {
    Notification::new()
        .appname(APP_NAME)
        .summary("任务提醒")
        .body(&format!(
            "你为命令 {} 设置的提醒触发了\n来自 {}",
            command, APP_NAME
        ))
        .show()?;

    Ok(())
}

pub fn time(hour: u8, minute: u8, second: u8) -> NormalResult {
    let now = Local::now();

    println!(
        "现在是 {} 时 {} 分 {} 秒 {} 毫秒",
        now.hour(),
        now.minute(),
        now.second(),
        now.nanosecond() / 1_000_000
    );

    let duration = Time {
        hour,
        minute,
        second,
    }
    .duration_from(Local::now());

    println!(
        "等待 {} 时 {} 分 {} 秒 {} 毫秒",
        duration.as_secs() / 3600,
        duration.as_secs() % 3600 / 60,
        duration.as_secs() % 60,
        duration.subsec_millis()
    );

    thread::sleep(duration);

    let now = Local::now();

    println!(
        "现在是 {} 时 {} 分 {} 秒 {} 毫秒",
        now.hour(),
        now.minute(),
        now.second(),
        now.nanosecond() / 1_000_000
    );

    Ok(())
}
