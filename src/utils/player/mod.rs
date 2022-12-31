#[cfg(not(target_os = "linux"))]
mod rodio_player;
#[cfg(target_os = "linux")]
mod soloud_player;

#[cfg(not(target_os = "linux"))]
use rodio_player as audio;
#[cfg(target_os = "linux")]
use soloud_player as player;

pub fn play(path: String) {
    player::play(path);
}
