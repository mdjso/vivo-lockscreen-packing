mod archivefiles;
mod cli;
mod pipeline;
mod register;
mod utils;
mod zip_command;

use crate::archivefiles::*;
use crate::cli::Args;
use crate::pipeline::process_lockscreen_package;
use crate::register::{do_register, do_unregister};
use crate::utils::pause_before_exit;
use crate::zip_command::ZipCommand;
use clap::Parser;
use console::Emoji;

static ERROR_EMOJI: Emoji<'_, '_> = Emoji("❌ ", "ERR");
static SPARKLE: Emoji<'_, '_> = Emoji("✨ ", ":-)");

fn main() {
    if let Err(err) = run() {
        eprintln!("{ERROR_EMOJI}程序运行出错, {err}");
        pause_before_exit();
        std::process::exit(1);
    }
}

fn run() -> Result<(), ArchiveError> {
    let args = Args::parse();

    if args.register {
        do_register()?;
        println!("{SPARKLE}右键功能注册成功!");
        return Ok(());
    } else if args.unregister {
        do_unregister()?;
        println!("{SPARKLE}右键功能取消注册成功!");
        return Ok(());
    }

    args.validate_input()?;

    let input_path = args.input_path.as_ref().ok_or_else(|| {
        ArchiveError::InvalidPath(std::io::Error::new(
            std::io::ErrorKind::InvalidInput,
            "未提供输入路径",
        ))
    })?;

    process_lockscreen_package(
        input_path,
        &args.resolved_output_dir()?,
        ZipCommand::resolve(args.zip_path.as_deref())?,
    )?;

    Ok(())
}
