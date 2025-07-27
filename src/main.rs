mod archivefiles;
mod cli;
mod pipeline;
mod utils;
mod zip_command;

use crate::archivefiles::*;
use crate::cli::Args;
use crate::pipeline::process_lockscreen_package;
use crate::utils::pause_before_exit;
use crate::zip_command::ZipCommand;
use clap::Parser;
use console::Emoji;

static ERROR_EMOJI: Emoji<'_, '_> = Emoji("❌ ", "ERR");

fn main() {
    if let Err(err) = run() {
        eprintln!("{ERROR_EMOJI}程序运行出错, {err}");
        pause_before_exit();
        std::process::exit(1);
    }
}

fn run() -> Result<(), ArchiveError> {
    let args = Args::parse();

    args.validate_input()?;

    process_lockscreen_package(
        &args.input_path,
        &args.resolved_output_dir()?,
        ZipCommand::resolve(args.zip_path.as_deref())?,
    )?;

    Ok(())
}
