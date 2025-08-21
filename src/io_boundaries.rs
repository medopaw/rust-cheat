/*
===============================================================
08. I/O 边界处理 - AI Coding 快速理解指南
===============================================================

🎯 业务场景：
- Web 服务：高并发处理文件和网络请求
- CLI 工具：简单文件操作，无需异步复杂性
- 数据处理：批量文件读写，网络 API 调用
- 混合应用：需要在同步和异步之间做出选择

🔍 30秒识别 I/O 模式：
- 看模块导入：std::fs (同步) vs tokio::fs (异步)
- 看函数签名：返回 Result 还是 impl Future
- 看调用方式：直接调用还是需要 .await
- 看错误类型：std::io::Error vs tokio::io::Error

⚠️ AI 常见 I/O 错误：
❌ 在 async fn 中使用阻塞的同步 I/O（如 std::fs）
❌ 混用同步和异步 I/O，导致性能问题
❌ 缺少超时控制，网络请求可能无限等待
❌ 没有重试机制，临时网络问题导致失败
❌ 不处理大文件，一次性读取到内存

✅ Review 清单：
- [ ] 同一层是否避免混用同步/异步 I/O？
- [ ] 网络请求是否有合理的超时设置？
- [ ] 是否实现了适当的重试策略？
- [ ] 错误处理是否充分（网络、文件系统错误）？
- [ ] 大文件处理是否考虑了内存使用？

📖 阅读顺序：
1. 先看上下文环境（async runtime 还是同步程序）
2. 再看 I/O 操作类型（文件、网络、数据库）
3. 最后看错误处理和资源管理（超时、清理）

设计决策指南：
- 简单脚本、CLI 工具 → 同步 I/O (std::fs, std::net)
- Web 服务、高并发 → 异步 I/O (tokio::fs, reqwest)
- 混合场景 → 保持一致性，避免 block_on 嵌套

核心 I/O 模式：

// 同步：std::fs::read_to_string / write
// 异步：tokio::fs / reqwest
// Review 要点：同一层避免混用同步/异步 I/O；明确超时/重试策略。

// 同步文件读取示例
use std::fs;
fn sync_read_example() -> std::io::Result<String> {
    let content = fs::read_to_string("example.txt")?;
    Ok(content)
}

// 异步文件读取示例
use tokio::fs;
async fn async_read_example() -> tokio::io::Result<String> {
    let content = fs::read_to_string("example.txt").await?;
    Ok(content)
}

// HTTP 请求示例
use reqwest;
async fn http_example() -> Result<String, reqwest::Error> {
    let response = reqwest::get("https://api.example.com/data").await?;
    let text = response.text().await?;
    Ok(text)
}
*/

pub fn io_concepts() {
    println!("I/O 边界概念：");
    println!("同步 I/O：");
    println!("  - std::fs::read_to_string / write");
    println!("  - 阻塞当前线程直到操作完成");
    println!("  - 适合简单脚本和小文件");
    
    println!("异步 I/O：");
    println!("  - tokio::fs 文件操作");
    println!("  - reqwest HTTP 请求");
    println!("  - 非阻塞，可并发处理");
    println!("  - 适合高并发服务");
}

pub fn io_best_practices() {
    println!("I/O 最佳实践：");
    println!("1. 同一层避免混用同步/异步 I/O");
    println!("2. 明确超时策略");
    println!("3. 实现重试机制");
    println!("4. 处理错误和资源清理");
    println!("5. 考虑缓冲和批处理");
}

pub fn sync_file_example() {
    println!("同步文件操作示例：");
    println!("使用 std::fs::read_to_string 读取文件");
}

pub fn async_file_example() {
    println!("异步文件操作示例：");
    println!("使用 tokio::fs::read_to_string 异步读取文件");
}