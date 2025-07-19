use std::fs;
use std::{io, path::Path};

use chrono::Local;
use rand::rngs::StdRng;
use rand::{Rng, SeedableRng};

// 修改 description.xml 中的 ID 和 title
pub fn deal_dscr_xml<P: AsRef<Path>>(description_file: P, new_number: &str) -> io::Result<()> {
    // 读取文件内容
    let mut content = fs::read_to_string(&description_file)?;

    // 替换 <id>xxx</id>
    if let Some(start) = content.find("<id>") {
        if let Some(end) = content[start..].find("</id>") {
            let full = &content[start..start + end + "</id>".len()];
            content = content.replacen(full, &format!("<id>{new_number}</id>"), 1);
        }
    }

    // 替换 <title locale="zh_CN"><![CDATA[xxx]]></title>
    if let Some(start) = content.find("<title locale=\"zh_CN\"><![CDATA[") {
        if let Some(end) = content[start..].find("]]></title>") {
            let full = &content[start..start + end + "]]></title>".len()];
            content = content.replacen(
                full,
                &format!("<title locale=\"zh_CN\"><![CDATA[{new_number}]]></title>"),
                1,
            );
        }
    }

    // 写入目标文件
    fs::write(&description_file, content)?;

    Ok(())
}

/// 生成锁屏编号, yyyymmdd+xxx
pub fn generate_lockscreen_number() -> String {
    let seed = Local::now().timestamp() as u64;
    let mut rng = StdRng::seed_from_u64(seed);
    let randnum: u32 = rng.random_range(100..=998);
    let date = Local::now().format("%Y%m%d").to_string();

    format!("{date}{randnum}")
}
