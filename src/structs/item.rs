use serde::{Deserialize, Serialize};
use std::cmp::Ordering;
use std::fmt::Display;
use std::ops::{Add, Sub};

#[derive(Debug, Clone, Default, Serialize, Deserialize)]
pub struct Item {
    pub time: Time,
    pub commands: Vec<Command>,
}

#[derive(Debug, Eq, PartialEq, Clone, Copy, Default, Serialize, Deserialize)]
pub struct Time {
    pub hour: u8,
    pub minute: u8,
    pub second: u8,
}

impl Display for Time {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{:02}:{:02}:{:02}", self.hour, self.minute, self.second)
    }
}

impl PartialOrd for Time {
    fn partial_cmp(&self, other: &Self) -> Option<Ordering> {
        Some(self.cmp(other))
    }
}

impl Ord for Time {
    fn cmp(&self, other: &Self) -> Ordering {
        if self.hour != other.hour {
            self.hour.cmp(&other.hour)
        } else if self.minute != other.minute {
            self.minute.cmp(&other.minute)
        } else {
            self.second.cmp(&other.second)
        }
    }
}

impl Add for Time {
    type Output = Self;

    fn add(self, other: Self) -> Self {
        let mut second = self.second as i32 + other.second as i32;
        let mut minute = self.minute as i32 + other.minute as i32;
        let mut hour = self.hour as i32 + other.hour as i32;

        while second >= 60 {
            second -= 60;
            minute += 1;
        }

        while minute >= 60 {
            minute -= 60;
            hour += 1;
        }

        while hour >= 24 {
            hour -= 24;
        }

        Time {
            hour: hour as u8,
            minute: minute as u8,
            second: second as u8,
        }
    }
}

impl Sub for Time {
    type Output = Self;

    fn sub(self, other: Self) -> Self {
        let mut second = self.second as i32 - other.second as i32;
        let mut minute = self.minute as i32 - other.minute as i32;
        let mut hour = self.hour as i32 - other.hour as i32;

        while second < 0 {
            second += 60;
            minute -= 1;
        }

        while minute < 0 {
            minute += 60;
            hour -= 1;
        }

        while hour < 0 {
            hour += 24;
        }

        Time {
            hour: hour as u8,
            minute: minute as u8,
            second: second as u8,
        }
    }
}

impl Time {
    pub fn second(second: usize) -> Self {
        let mut second = second;
        let mut minute = 0;
        let mut hour = 0;

        while second >= 60 {
            second -= 60;
            minute += 1;
        }

        while minute >= 60 {
            minute -= 60;
            hour += 1;
        }

        while hour >= 24 {
            hour -= 24;
        }

        Time {
            hour: hour as u8,
            minute: minute as u8,
            second: second as u8,
        }
    }
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Command {
    pub command: String,
    pub parameters: String,
    pub audio: bool,
    pub notify: isize,
}

impl Default for Command {
    fn default() -> Self {
        Command {
            command: String::new(),
            parameters: String::new(),
            audio: false,
            notify: -1,
        }
    }
}

pub fn add_command(config: &mut Vec<Item>, time: Time, command: Command, search_index: usize) {
    for i in search_index..config.len() {
        match config[i].time.cmp(&time) {
            Ordering::Greater => {
                config.insert(
                    i,
                    Item {
                        time,
                        commands: vec![command],
                    },
                );
                return;
            }
            Ordering::Equal => {
                config[i].commands.push(command);
                return;
            }
            Ordering::Less => {}
        }
    }
    config.push(Item {
        time,
        commands: vec![command],
    });
}

#[macro_export]
macro_rules! add_command {
    ($config:expr, $time:expr, $command:expr) => {
        $crate::structs::item::add_command($config, $time, $command, 0);
    };
}

pub fn add_command_reverse(
    config: &mut Vec<Item>,
    time: Time,
    command: Command,
    search_index: usize,
) {
    for i in (0..search_index).rev() {
        match config[i].time.cmp(&time) {
            Ordering::Less => {
                config.insert(
                    i + 1,
                    Item {
                        time,
                        commands: vec![command],
                    },
                );
                return;
            }
            Ordering::Equal => {
                config[i].commands.push(command);
                return;
            }
            Ordering::Greater => {}
        }
    }
    config.insert(
        0,
        Item {
            time,
            commands: vec![command],
        },
    );
}

#[macro_export]
macro_rules! add_command_reverse {
    ($config:expr, $time:expr, $command:expr) => {
        $crate::structs::item::add_command_reverse($config, $time, $command, $config.len());
    };
}
