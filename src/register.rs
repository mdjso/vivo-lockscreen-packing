use std::io;

#[cfg(windows)]
use win_right_click::{register_windows_right_click, unregister_windows_right_click};

#[cfg(windows)]
pub fn do_register() -> io::Result<()> {
    register_windows_right_click()
}

#[cfg(not(windows))]
pub fn do_register() -> io::Result<()> {
    Err(io::Error::new(
        io::ErrorKind::Unsupported,
        "当前平台不支持注册右键菜单",
    ))
}

#[cfg(windows)]
pub fn do_unregister() -> io::Result<()> {
    unregister_windows_right_click()
}

#[cfg(not(windows))]
pub fn do_unregister() -> io::Result<()> {
    Err(io::Error::new(
        io::ErrorKind::Unsupported,
        "当前平台不支持取消注册右键菜单",
    ))
}

#[cfg(windows)]
mod win_right_click {
    use std::env;
    use std::io;
    use std::path::PathBuf;
    use winreg::RegKey;
    use winreg::enums::*;

    pub fn register_windows_right_click() -> io::Result<()> {
        let hkcr = RegKey::predef(HKEY_CLASSES_ROOT);
        let shell = hkcr.open_subkey_with_flags("Directory\\shell", KEY_WRITE)?;

        let vlp = shell.create_subkey("vlp")?;

        let exe_path: PathBuf = env::current_exe()?;
        let exe_str = exe_path.to_string_lossy();

        vlp.0.set_value("", &"打包vivo锁屏包")?;
        vlp.0.set_value("Icon", &exe_str.to_string())?;

        let command_str = format!("\"{exe_str}\" \"%1\"");
        let command = vlp.0.create_subkey("command")?;
        command.0.set_value("", &command_str)?;

        Ok(())
    }

    pub fn unregister_windows_right_click() -> io::Result<()> {
        let hkcr = RegKey::predef(HKEY_CLASSES_ROOT);
        let shell = hkcr.open_subkey_with_flags("Directory\\shell", KEY_WRITE)?;
        shell.delete_subkey_all("vlp")?;
        Ok(())
    }
}
