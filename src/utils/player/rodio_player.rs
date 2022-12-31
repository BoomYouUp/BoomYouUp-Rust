pub fn play(path: String) {
    std::thread::spawn(move || match rodio::OutputStream::try_default() {
        Ok((_stream, handle)) => match rodio::Sink::try_new(&handle) {
            Ok(sink) => match std::fs::File::open(path) {
                Ok(file) => match rodio::Decoder::new(std::io::BufReader::new(file)) {
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
}
