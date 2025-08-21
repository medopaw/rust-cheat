/*
===============================================================
03. 错误处理模式 - AI Coding 快速理解指南  
===============================================================

🎯 业务场景：
- 应用层错误聚合：多种不同错误类型统一处理
- 用户面向的错误信息：添加上下文便于调试定位
- CLI 工具、Web 服务的错误处理链

🔍 30秒识别错误处理模式：
- 看返回类型：anyhow::Result<T> = 应用层统一错误
- 看 ? 操作符：自动错误转换和提前返回
- 看 with_context()：添加调试上下文信息
- 看 map_err()：手动错误类型转换

⚠️ AI 常写的反模式：
❌ 到处使用 unwrap()/expect() 导致程序 panic
❌ 错误信息不明确（"Something went wrong"）
❌ 混用不同错误处理策略（anyhow + 自定义错误）
❌ 在库层使用 anyhow（应该用 thiserror 自定义错误）
❌ 忽略错误上下文，难以定位问题源头

✅ Review 清单：
- [ ] 用户可控输入是否有验证和合适的错误提示？
- [ ] 错误信息是否包含足够的调试信息？
- [ ] 是否避免了 panic!（除非真的不可恢复）？
- [ ] 错误传播路径是否清晰（? 链条）？
- [ ] 是否在合适的层次使用了 with_context？

📖 阅读顺序：
1. 先看函数返回类型（anyhow::Result 还是自定义错误）
2. 再看错误产生点（哪些操作可能失败）
3. 最后看错误处理（? 传播还是 match 处理，是否有上下文）

核心模式：
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