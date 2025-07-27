use std::io;
use std::path::{Path, PathBuf};
use std::process::Command;

pub struct ZipCommand {
    pub path: PathBuf,
}

impl ZipCommand {
    /// Attempts to resolve the path to the `zip` executable.
    ///
    /// The resolution follows these steps:
    /// 1. If a user-specified path is provided and exists, it is used.
    /// 2. If not, tries to find `zip` in the same directory as the current executable (for embedded deployment).
    /// 3. If neither is found, returns an error indicating that the `zip` command was not found.
    ///
    /// # Arguments
    ///
    /// * `zip_command_path` - An optional path to the `zip` executable specified by the user.
    ///
    /// # Returns
    ///
    /// Returns a `ZipCommand` instance with the resolved path on success, or an `io::Error` if not found.
    pub fn resolve(zip_command_path: Option<&Path>) -> io::Result<Self> {
        // customize zip path
        if let Some(path) = zip_command_path {
            if path.exists() {
                return Ok(Self {
                    path: path.to_path_buf(),
                });
            }
        }

        // find zip in exec program's parent path
        if let Ok(exec_path) = std::env::current_exe() {
            if let Some(parent) = exec_path.parent() {
                let candidate = parent.join("zip");
                if candidate.exists() {
                    return Ok(Self { path: candidate });
                }
            }
        }

        // find zip in PATH
        if let Ok(zip_in_path) = which::which("zip") {
            return Ok(Self { path: zip_in_path });
        }

        Err(io::Error::new(
            io::ErrorKind::NotFound,
            "未找到可执行的 zip 命令",
        ))
    }

    pub fn zip_dir(&self, to: &Path, from: &Path) -> io::Result<()> {
        let output = Command::new(&self.path)
            .arg("-r")
            .arg(to)
            .arg(".")
            .current_dir(from)
            .output()?;

        self.report_error(&output)
    }
    pub fn zip_file(&self, to: &Path, from: &Path) -> io::Result<()> {
        let mut cmd = Command::new(&self.path);

        if let Some(file_name) = from.file_name()
            && let Some(parent_dir) = from.parent()
        {
            let output = cmd
                .arg(to)
                .arg(file_name)
                .current_dir(parent_dir)
                .output()?;
            self.report_error(&output)
        } else {
            Err(std::io::Error::new(
                std::io::ErrorKind::InvalidInput,
                format!(
                    "无效的文件路径, 无法获取文件名或父目录: [{}]",
                    from.display()
                ),
            ))
        }
    }
    pub fn report_error(&self, output: &std::process::Output) -> io::Result<()> {
        if !output.status.success() {
            let stdout = String::from_utf8_lossy(&output.stdout);
            return Err(io::Error::other(format!("zip 命令执行失败: \n{stdout}")));
        }
        Ok(())
    }
}
