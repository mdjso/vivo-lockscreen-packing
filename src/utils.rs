use std::fs;
use std::io::Write;
use std::{io, path::Path};

use chrono::Local;
use rand::rngs::StdRng;
use rand::{Rng, SeedableRng};

/// 生成锁屏编号, yyyymmdd+xxx
pub fn generate_lockscreen_number() -> String {
    let seed = Local::now().timestamp() as u64;
    let mut rng = StdRng::seed_from_u64(seed);
    let randnum: u32 = rng.random_range(100..=998);
    let date = Local::now().format("%Y%m%d").to_string();

    format!("{date}{randnum}")
}

// 修改 description.xml 中的 ID 和 title
pub fn deal_dscr_xml<P: AsRef<Path>>(description_file: P, new_number: &str) -> io::Result<()> {
    let path = description_file.as_ref();
    let content = fs::read_to_string(path)?;

    let updated = replace_id_and_title(&content, new_number)
        .ok_or_else(|| io::Error::other("id and title replacements must both succeed"))?;

    fs::write(path, updated)?;
    Ok(())
}

pub fn pause_before_exit() {
    eprint!("按回车键退出...");
    let _ = std::io::stdout().flush();
    let _ = std::io::stdin().read_line(&mut String::new());
}

fn replace_id_and_title(content: &str, new_number: &str) -> Option<String> {
    let (id_start_tag, id_end_tag) = ("<id>", "</id>");
    let (title_start_tag, title_end_tag) = (r##"<title locale="zh_CN"><![CDATA["##, "]]></title>");

    replace_tag(content, id_start_tag, id_end_tag, new_number)
        .and_then(|c| replace_tag(&c, title_start_tag, title_end_tag, new_number))
}

fn replace_tag(content: &str, start_tag: &str, end_tag: &str, new_value: &str) -> Option<String> {
    let start_index = content.find(start_tag)?;
    let end_index_rel = content[start_index..].find(end_tag)?;
    let end_index = start_index + end_index_rel + end_tag.len();

    let old_segment = &content[start_index..end_index];
    let new_segment = format!("{start_tag}{new_value}{end_tag}");

    Some(content.replacen(old_segment, &new_segment, 1))
}
