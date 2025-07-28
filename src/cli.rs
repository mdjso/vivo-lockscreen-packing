// src/cli.rs
use clap::Parser;
use std::io;
use std::path::{Path, PathBuf};

use crate::archivefiles::ArchiveError;

/// VIVO锁屏打包工具
#[derive(Parser, Debug)]
#[command(author = "mdjso", version = "1.1", about = "用于打包锁屏主题的工具")]
pub struct Args {
    /// 锁屏包路径（必须包含 preview、description.xml、lockscreen/manifest.xml）
    #[arg(value_name = "锁屏包路径", value_hint = clap::ValueHint::DirPath)]
    pub input_path: Option<PathBuf>,

    /// 执行注册功能
    #[arg(long, help = "注册右键菜单项")]
    pub register: bool,

    /// 执行注册功能
    #[arg(long, help = "取消注册右键菜单项")]
    pub unregister: bool,

    /// 指定 zip 可执行文件路径
    #[arg(short, long, value_name = "ZIP可执行文件", value_hint = clap::ValueHint::FilePath)]
    pub zip_path: Option<PathBuf>,

    /// 输出目录路径（默认为输入路径的上一级目录）
    #[arg(short, long, value_name = "输出目录", value_hint = clap::ValueHint::DirPath)]
    pub output: Option<PathBuf>,
}

impl Args {
    pub fn get_input_path(&self) -> Result<&Path, ArchiveError> {
        self.input_path.as_deref().ok_or_else(|| {
            ArchiveError::InvalidPath(io::Error::new(
                io::ErrorKind::InvalidInput,
                "未提供锁屏包路径, 请拖拽或指定锁屏包路径\n用法: vlp <输入路径>\n获取帮助信息, 尝试 'vlp --help'.",
            ))
        })
    }

    pub fn validate_input(&self) -> Result<(), ArchiveError> {
        let input_path = self.get_input_path()?;
        let required = ["preview", "description.xml", "lockscreen/manifest.xml"];
        let mut missing = Vec::new();

        for entry in &required {
            let path = input_path.join(entry);
            if !path.exists() {
                missing.push(entry.to_string());
            }
        }

        if !missing.is_empty() {
            return Err(ArchiveError::InvalidPath(io::Error::new(
                io::ErrorKind::NotFound,
                format!("锁屏包缺少以下必要文件: {}", missing.join(", ")),
            )));
        }

        Ok(())
    }

    pub fn resolved_output_dir(&self) -> Result<PathBuf, ArchiveError> {
        if let Some(output) = &self.output {
            return Ok(output.clone());
        }

        let input_path = self.get_input_path()?;

        if let Some(parent) = input_path.parent() {
            return Ok(parent.to_path_buf());
        }

        Err(ArchiveError::InvalidPath(std::io::Error::new(
            std::io::ErrorKind::InvalidInput,
            format!("无法从路径 {:?} 推导父目录", self.input_path),
        )))
    }
}
