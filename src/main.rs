mod archivefiles;
mod cli;
mod pipeline;
mod utils;
mod zip_command;

use crate::archivefiles::*;
use crate::cli::Args;
use crate::pipeline::process_lockscreen_package;
use crate::zip_command::ZipCommand;
use clap::Parser;

fn main() -> Result<(), ArchiveError> {
    let args = Args::parse();

    args.validate_input()?;
    process_lockscreen_package(
        &args.input_path,
        &args.resolved_output_dir()?,
        ZipCommand::resolve(args.zip_path.as_deref())?,
    )?;
    Ok(())
}
