/*
===============================================================
04. anyhow（应用层错误聚合 + 上下文）
===============================================================

// - 应用/二进制层常用：anyhow::Result<T> = Result<T, anyhow::Error>
// - 可装多源错误，配合 ? 使用极简
// - with_context 添加定位信息

use anyhow::{Context, Result};
fn load_cfg(path: &str) -> Result<String> {
    let s = std::fs::read_to_string(path)
        .with_context(|| format!("read config failed: {}", path))?;
    Ok(s)
}
// 库层更建议用 thiserror 自定义错误，而非 anyhow。
*/

pub fn anyhow_demo() {
    println!("anyhow 错误处理演示：");
    println!("- 应用层聚合多种错误类型");
    println!("- with_context() 添加上下文信息");
    println!("- ? 操作符自动转换错误类型");
    
    // 模拟错误处理
    match simulate_file_read("config.toml") {
        Ok(content) => println!("配置内容: {}", content),
        Err(e) => println!("错误: {}", e),
    }
}

fn simulate_file_read(path: &str) -> Result<String, &'static str> {
    if path.ends_with(".toml") {
        Ok("key = value".to_string())
    } else {
        Err("不支持的文件格式")
    }
}