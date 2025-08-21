/*
===============================================================
09. I/O 边界（同步 vs 异步）
===============================================================

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