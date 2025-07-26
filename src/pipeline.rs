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
    let temp_stage_dir = TempDir::new()?;
    let temp_dist_dir = TempDir::new()?;

    let intermediate_lockscreen_zip = temp_stage_dir.path().join("lockscreen.zip");
    let assembled_lockscreen_zip = temp_dist_dir.path().join("lockscreen.zip");
    let final_itz_file = temp_dist_dir.path().join("lockscreen.itz");

    // Step 1: 打包 lockscreen 目录
    ArchiveFile::new(None, &input.join("lockscreen"))?
        .not_copy()
        .zip_with(&zip_command, &intermediate_lockscreen_zip)?;

    // Step 2: 修复 description.xml 文件
    let fixed_description_file = temp_stage_dir.path().join("description.xml");
    let lockscreen_version_number = generate_lockscreen_number();
    copy_file(
        input.join("description.xml"),
        &fixed_description_file,
        &FileCopyOptions::new(),
    )?;
    deal_dscr_xml(&fixed_description_file, &lockscreen_version_number)?;

    // Step 3: 组装的 .itz 文件
    ArchiveFiles::new(vec![
        ArchiveFile::new(
            Some(format!("lockscreen/{lockscreen_version_number}.zip")),
            &intermediate_lockscreen_zip,
        )?,
        ArchiveFile::new(None, &input.join("preview"))?,
        ArchiveFile::new(None, &fixed_description_file)?,
    ])?
    .zip_with(&zip_command, &assembled_lockscreen_zip)?;
    std::fs::rename(&assembled_lockscreen_zip, &final_itz_file)?;

    // Step 5: 再次打包 .itz 文件, 并重命名为 "lockscreen"
    ArchiveFile::new(None, &final_itz_file)?
        .not_copy()
        //zip_with 函数目前需要传入绝对路径
        .zip_with(&zip_command, output.canonicalize()?.join("lockscreen.zip"))?;
    std::fs::rename(output.join("lockscreen.zip"), output.join("lockscreen"))?;

    Ok(())
}
