use crate::{
    archivefiles::{ArchiveError, ArchiveFile, ArchiveFiles},
    utils::{deal_dscr_xml, generate_lockscreen_number},
    zip_command::ZipCommand,
};
use console::Emoji;
use fs_extra::file::{CopyOptions as FileCopyOptions, copy as copy_file};
use indicatif::{ProgressBar, ProgressStyle};
use std::{path::Path, time::Duration};
use tempfile::TempDir;

static SPARKLE: Emoji<'_, '_> = Emoji("✨ ", ":-)");

pub fn process_lockscreen_package(
    input: &Path,
    output: &Path,
    zip_command: ZipCommand,
) -> Result<(), ArchiveError> {
    let temp_stage_dir = TempDir::new()?;
    let temp_dist_dir = TempDir::new()?;

    let intermediate_lockscreen_zip = temp_stage_dir.path().join("lockscreen.zip");
    let final_itz_file = temp_dist_dir.path().join("lockscreen.itz");

    // 添加打包动画
    let spinner = ProgressBar::new_spinner();
    let spinner_style = ProgressStyle::with_template("{spinner:.blue}{wide_msg}")
        .unwrap()
        .tick_strings(&["⢎ ", "⠎⠁", "⠊⠑", "⠈⠱", " ⡱", "⢀⡰", "⢄⡠", "⢆⡀", ""]);
    spinner.enable_steady_tick(Duration::from_millis(80));
    spinner.set_style(spinner_style);
    spinner.set_message(format!(" 正在打包:{}...", input.display()));

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

    // Step 3: 组装 .itz 文件
    ArchiveFiles::new(vec![
        ArchiveFile::new(
            Some(format!("lockscreen/{lockscreen_version_number}.zip")),
            &intermediate_lockscreen_zip,
        )?,
        ArchiveFile::new(None, &input.join("preview"))?,
        ArchiveFile::new(None, &fixed_description_file)?,
    ])?
    .zip_and_rename(&zip_command, &final_itz_file)?;

    // Step 4: 重新打包 .itz 文件, 并重命名为 "lockscreen"
    ArchiveFile::new(None, &final_itz_file)?
        .not_copy()
        //zip_with 函数目前需要传入绝对路径
        .zip_and_rename(&zip_command, output.canonicalize()?.join("lockscreen"))?;

    spinner.finish_with_message(format!(
        "{}输出路径: {}",
        SPARKLE,
        output.join("lockscreen").display()
    ));

    Ok(())
}
