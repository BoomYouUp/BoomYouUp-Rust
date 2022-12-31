use soloud::{AudioExt, LoadExt};

pub fn play(path: String) {
    std::thread::spawn(move || match soloud::Soloud::default() {
        Ok(soloud) => {
            let mut wav = soloud::Wav::default();
            match wav.load(std::path::Path::new(&path)) {
                Ok(_) => {
                    soloud.play(&wav);

                    while soloud.active_voice_count() > 0 {
                        std::thread::sleep(std::time::Duration::from_millis(100));
                    }
                }
                Err(e) => eprintln!("音频解码失败：{}", e),
            }
        }
        Err(e) => eprintln!("音频播放器创建失败：{}", e),
    });
}
