use crate::zip_command::ZipCommand;
use fs_extra::dir::{CopyOptions as DirCopyOptions, copy as copy_dir};
use fs_extra::file::{CopyOptions as FileCopyOptions, copy as copy_file};
use std::fs::{self, Metadata};
use std::io;
use std::path::{Path, PathBuf};
use tempfile::TempDir;

pub struct ArchiveFiles {
    pub files: Vec<ArchiveFile>,
}

impl ArchiveFiles {
    pub fn new(files: Vec<ArchiveFile>) -> Result<Self, ArchiveError> {
        Ok(ArchiveFiles { files })
    }
    pub fn zip_with<P: AsRef<Path>>(
        &self,
        zip_command: &ZipCommand,
        to: P,
    ) -> Result<(), ArchiveError> {
        let tmp_dir = TempDir::new()?;
        let staging_root = tmp_dir.path();

        // creat directory structure in temporary dir
        for file in &self.files {
            file.copy_to(staging_root)?
        }

        zip_command.zip_dir(to.as_ref(), staging_root)?;

        Ok(())
    }

    pub fn zip_and_rename<P: AsRef<Path>>(
        &self,
        zip_command: &ZipCommand,
        to: P,
    ) -> Result<(), ArchiveError> {
        let zip_to = to.as_ref().with_extension("zip");
        self.zip_with(zip_command, &zip_to)?;

        std::fs::rename(&zip_to, &to).inspect_err(|_| {
            let _ = std::fs::remove_file(&zip_to);
        })?;

        Ok(())
    }
}

#[derive(Debug, PartialEq)]
pub enum FileType {
    Dir,
    File,
    Other,
}

impl From<&Metadata> for FileType {
    fn from(metadata: &Metadata) -> Self {
        let file_type = metadata.file_type();
        if file_type.is_dir() {
            FileType::Dir
        } else if file_type.is_file() {
            FileType::File
        } else {
            FileType::Other
        }
    }
}

pub struct ArchiveFile {
    /// 压缩包中的相对路径（如果为 None，则使用原始文件名）
    pub position: Option<String>,

    /// 原始文件系统路径
    pub path: PathBuf,

    /// 文件类型（文件、目录或其他）
    pub file_type: FileType,

    /// 是否将文件拷贝到临时目录用于打包,目前当不复制文件时，不会在压缩包中创建文件结构
    should_copy: bool,
}

impl ArchiveFile {
    pub fn new(position: Option<String>, path: &Path) -> Result<ArchiveFile, ArchiveError> {
        let metadata = fs::metadata(path).map_err(ArchiveError::Io)?;
        let file_type = FileType::from(&metadata);

        Ok(ArchiveFile {
            position,
            path: path.to_path_buf(),
            file_type,
            should_copy: true,
        })
    }

    pub fn not_copy(mut self) -> Self {
        self.should_copy = false;
        self
    }

    fn copy_file_to(&self, target_path: &Path) -> Result<(), ArchiveError> {
        if let Some(parent) = target_path.parent() {
            if !parent.exists() {
                std::fs::create_dir_all(parent)?;
            }
        }
        copy_file(&self.path, target_path, &FileCopyOptions::new())?;
        Ok(())
    }

    fn copy_dir_to(&self, target_path: &Path) -> Result<(), ArchiveError> {
        if !target_path.exists() {
            std::fs::create_dir_all(target_path)?;
        }
        copy_dir(
            &self.path,
            target_path,
            &DirCopyOptions::new().content_only(true),
        )?;
        Ok(())
    }

    fn copy_to<P: AsRef<Path>>(&self, dir_path: P) -> Result<(), ArchiveError> {
        let target_path = if let Some(pos) = &self.position {
            dir_path.as_ref().join(pos)
        } else {
            dir_path.as_ref().join(self.path.file_name().ok_or_else(|| {
                ArchiveError::InvalidPath(io::Error::new(
                    io::ErrorKind::InvalidInput,
                    format!("文件名为空: {:?}", self.path),
                ))
            })?)
        };

        match self.file_type {
            FileType::File => self.copy_file_to(&target_path),
            FileType::Dir => self.copy_dir_to(&target_path),
            FileType::Other => Err(ArchiveError::InvalidPath(io::Error::new(
                io::ErrorKind::InvalidInput,
                format!("不支持的文件类型:: {:?}", self.path),
            ))),
        }
    }

    pub fn zip_with<P: AsRef<Path>>(
        &self,
        zip_command: &ZipCommand,
        to: P,
    ) -> Result<(), ArchiveError> {
        if self.should_copy {
            // 创建临时目录并拷贝后再压缩
            let tmp_dir = TempDir::new()?;
            let staging_root = tmp_dir.path();

            self.copy_to(staging_root)?;
            zip_command.zip_dir(to.as_ref(), staging_root)?;
        } else {
            // 直接压缩原始路径
            match self.file_type {
                FileType::File => {
                    zip_command.zip_file(to.as_ref(), &self.path)?;
                }
                FileType::Dir => {
                    zip_command.zip_dir(to.as_ref(), &self.path)?;
                }
                FileType::Other => {
                    return Err(ArchiveError::InvalidPath(io::Error::new(
                        io::ErrorKind::InvalidInput,
                        format!("不支持的文件类型: {:?}", self.path),
                    )));
                }
            }
        }

        Ok(())
    }
    pub fn zip_and_rename<P: AsRef<Path>>(
        &self,
        zip_command: &ZipCommand,
        to: P,
    ) -> Result<(), ArchiveError> {
        let zip_to = to.as_ref().with_extension("zip");
        self.zip_with(zip_command, &zip_to)?;

        std::fs::rename(&zip_to, &to).inspect_err(|_| {
            let _ = std::fs::remove_file(&zip_to);
        })?;

        Ok(())
    }
}

#[derive(Debug)]
pub enum ArchiveError {
    InvalidPath(std::io::Error),
    Io(std::io::Error),
}

impl std::fmt::Display for ArchiveError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            ArchiveError::InvalidPath(path) => write!(f, "{path}"),
            ArchiveError::Io(e) => write!(f, "{e}"),
        }
    }
}

impl From<std::io::Error> for ArchiveError {
    fn from(err: std::io::Error) -> Self {
        ArchiveError::Io(err)
    }
}

impl From<fs_extra::error::Error> for ArchiveError {
    fn from(err: fs_extra::error::Error) -> Self {
        match err.kind {
            fs_extra::error::ErrorKind::Io(io_err) => ArchiveError::Io(io_err),
            fs_extra::error::ErrorKind::StripPrefix(e) => {
                ArchiveError::InvalidPath(std::io::Error::new(std::io::ErrorKind::InvalidInput, e))
            }
            fs_extra::error::ErrorKind::OsString(s) => {
                ArchiveError::InvalidPath(std::io::Error::new(
                    std::io::ErrorKind::InvalidInput,
                    format!("Invalid OsString: {s:?}"),
                ))
            }
            _ => ArchiveError::InvalidPath(std::io::Error::other(format!(
                "fs_extra error: {:?}",
                err.kind
            ))),
        }
    }
}
