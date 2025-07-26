// src/cli.rs
use clap::Parser;
use std::io;
use std::path::PathBuf;

use crate::archivefiles::ArchiveError;

/// VIVO锁屏打包工具
#[derive(Parser, Debug)]
#[command(author = "mdjso", version = "1.1", about = "用于打包锁屏主题的工具")]
pub struct Args {
    /// 输入的锁屏目录路径（必须包含 preview、description.xml、manifest.xml）
    #[arg(value_name = "输入路径")]
    pub input_path: std::path::PathBuf,

    /// 指定 zip 可执行文件路径（可选）
    #[arg(short, long, help = "zip 可执行文件路径")]
    pub zip_path: Option<std::path::PathBuf>,

    /// 输出目录（默认为输入路径的上一级）
    #[arg(short, long, help = "输出目录路径")]
    pub output: Option<std::path::PathBuf>,
}

impl Args {
    pub fn validate_input(&self) -> io::Result<()> {
        let required = [
            "preview",
            "description.xml",
            "lockscreen",
            "lockscreen/manifest.xml",
        ];
        for entry in &required {
            let path = self.input_path.join(entry);
            if !path.exists() {
                return Err(io::Error::new(
                    io::ErrorKind::NotFound,
                    format!("缺少必要文件: {entry}"),
                ));
            }
        }
        Ok(())
    }

    pub fn resolved_output_dir(&self) -> Result<PathBuf, ArchiveError> {
        if let Some(output) = &self.output {
            return Ok(output.clone());
        }

        if let Some(parent) = self.input_path.parent() {
            return Ok(parent.to_path_buf());
        }

        Err(ArchiveError::InvalidPath(std::io::Error::new(
            std::io::ErrorKind::InvalidInput,
            format!("无法从路径 {:?} 推导父目录", self.input_path),
        )))
    }
}
