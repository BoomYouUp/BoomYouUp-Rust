use std::path::PathBuf;
use std::thread;
use std::time::Duration;

use notify_rust::Notification;
use opener::open;
use soloud::{AudioExt, LoadExt, Soloud, Wav};

use crate::error::NormalResult;
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

    while player.active_voice_count() > 0 {
        thread::sleep(Duration::from_millis(100));
    }

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
