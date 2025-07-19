use std::path::Path;

use crate::{
    archivefiles::{ArchiveError, ArchiveFile, ArchiveFiles},
    utils::{deal_dscr_xml, generate_lockscreen_number},
    zip_command::ZipCommand,
};
use fs_extra::file::{CopyOptions as FileCopyOptions, copy as copy_file};
use tempfile::TempDir;

pub fn process_lockscreen_package(
    input: &Path,
    output: &Path,
    zip_command: ZipCommand,
) -> Result<(), ArchiveError> {
    let temp_dir_1 = TempDir::new()?;
    let temp_dir_2 = TempDir::new()?;

    let intermediate = temp_dir_1.path().join("lockscreen.zip");
    let final_zip = temp_dir_2.path().join("lockscreen.zip");
    let final_itz = temp_dir_2.path().join("lockscreen.itz");

    // Step 1
    ArchiveFile::new(Some(".".into()), &input.join("lockscreen"))?
        .not_copy()
        .zip_with(&zip_command, &intermediate)?;

    // Step 2
    let description_fix = temp_dir_1.path().join("description.xml");
    let number = generate_lockscreen_number();
    copy_file(
        input.join("description.xml"),
        &description_fix,
        &FileCopyOptions::new(),
    )?;
    deal_dscr_xml(&description_fix, &number)?;

    // Step 3
    ArchiveFiles::new(vec![
        ArchiveFile::new(Some(format!("lockscreen/{number}.zip")), &intermediate)?,
        ArchiveFile::new(None, &input.join("preview"))?,
        ArchiveFile::new(None, &description_fix)?,
    ])?
    .zip_with(&zip_command, &final_zip)?;

    // Step 4
    std::fs::rename(final_zip, &final_itz)?;

    // Step 5
    ArchiveFile::new(None, &final_itz)?
        .not_copy()
        //zip_to 函数目前传入绝对路径
        .zip_with(&zip_command, output.canonicalize()?.join("lockscreen.zip"))?;
    std::fs::rename(output.join("lockscreen.zip"), output.join("lockscreen"))?;

    Ok(())
}
