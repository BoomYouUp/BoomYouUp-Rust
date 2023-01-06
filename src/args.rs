use std::path::PathBuf;

use clap::{Parser, Subcommand};

static DEFAULT_CONFIG_PATH: &str = "config.yaml";

#[derive(Debug, Parser)]
#[command(version)]
#[command(about = "一个简单的定时任务程序")]
#[command(
    long_about = "一个简单的定时任务程序，可以在指定时间执行指定命令，支持使用内置播放器播放音频，还能提前发送系统通知提醒。"
)]
pub struct Args {
    #[command(subcommand)]
    pub action: Actions,
}

#[derive(Debug, Subcommand)]
pub enum Actions {
    /// 开始运行
    Run {
        /// 指定自定义配置文件
        #[arg(short, long, value_name = "PATH", default_value = DEFAULT_CONFIG_PATH)]
        config: PathBuf,
    },

    /// 进行配置
    Configure {
        /// 指定自定义配置文件
        #[arg(short, long, value_name = "PATH", default_value = DEFAULT_CONFIG_PATH)]
        config: PathBuf,
    },

    /// 测试功能
    Test {
        #[command(subcommand)]
        function: Functions,
    },
}

#[derive(Debug, Subcommand)]
pub enum Functions {
    /// 执行命令或打开文件
    Execute {
        /// 要执行的命令
        command: String,

        /// 要传递给命令的参数
        parameters: Option<Vec<String>>,
    },

    /// 播放音频
    PlayAudio {
        /// 要播放的音频文件
        path: PathBuf,
    },

    /// 发送系统通知
    SendNotification,
}
