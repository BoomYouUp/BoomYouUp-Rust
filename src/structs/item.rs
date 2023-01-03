use std::cmp::Ordering;
use std::fmt::Display;
use std::ops::{Add, RangeBounds, Sub};

use serde::{Deserialize, Serialize};

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

pub trait AddCommand {
    fn _add_command<R: RangeBounds<usize> + Iterator<Item=usize> + DoubleEndedIterator>(
        &mut self,
        time: Time,
        command: Command,
        range: R,
    );

    fn add_command_with_index(&mut self, time: Time, command: Command, index: usize);

    fn add_command(&mut self, time: Time, command: Command) {
        self.add_command_with_index(time, command, 0);
    }

    fn _add_command_reverse<R: RangeBounds<usize> + Iterator<Item=usize> + DoubleEndedIterator>(
        &mut self,
        time: Time,
        command: Command,
        range: R,
    );

    fn add_command_reverse_with_index(&mut self, time: Time, command: Command, index: usize) {
        self._add_command_reverse(time, command, 0..index);
    }

    fn add_command_reverse(&mut self, time: Time, command: Command);
}

impl AddCommand for Vec<Item> {
    fn _add_command<R: RangeBounds<usize> + IntoIterator<Item=usize> + DoubleEndedIterator>(
        &mut self,
        time: Time,
        command: Command,
        range: R,
    ) {
        for i in range {
            match self[i].time.cmp(&time) {
                Ordering::Greater => {
                    self.insert(
                        i,
                        Item {
                            time,
                            commands: vec![command],
                        },
                    );
                    return;
                }
                Ordering::Equal => {
                    self[i].commands.push(command);
                    return;
                }
                Ordering::Less => {}
            }
        }
        self.push(Item {
            time,
            commands: vec![command],
        });
    }

    fn add_command_with_index(&mut self, time: Time, command: Command, index: usize) {
        self._add_command(time, command, index..self.len());
    }

    fn _add_command_reverse<
        R: RangeBounds<usize> + Iterator<Item=usize> + DoubleEndedIterator,
    >(
        &mut self,
        time: Time,
        command: Command,
        range: R,
    ) {
        for i in range.rev() {
            match self[i].time.cmp(&time) {
                Ordering::Less => {
                    self.insert(
                        i + 1,
                        Item {
                            time,
                            commands: vec![command],
                        },
                    );
                    return;
                }
                Ordering::Equal => {
                    self[i].commands.push(command);
                    return;
                }
                Ordering::Greater => {}
            }
        }
        self.insert(
            0,
            Item {
                time,
                commands: vec![command],
            },
        );
    }

    fn add_command_reverse(&mut self, time: Time, command: Command) {
        self.add_command_reverse_with_index(time, command, self.len());
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
