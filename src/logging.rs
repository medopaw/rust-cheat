/*
===============================================================
07. 日志与可观测性：tracing 基本用法
===============================================================

// 初始化（main 里一次）：
use tracing::{info, error, instrument};
use tracing_subscriber::FmtSubscriber;

fn init_logs() {
    let sub = FmtSubscriber::builder()
        .with_max_level(tracing::Level::INFO)
        .finish();
    tracing::subscriber::set_global_default(sub).unwrap();
}

// 事件（结构化字段）：
info!(user_id = 42, action = "login", "user action");
error!(error = %e, "db write failed");

// span（范围 + 自动记录进入/退出 + 参数）：
#[instrument(skip(secret), fields(req_id = %req_id))]
fn handle(req_id: u64, secret: &str) -> anyhow::Result<()> {
    info!("start");
    Ok(())
}
*/ 

pub fn logging_concepts() {
    println!("日志与可观测性概念：");
    println!("- 结构化日志：使用字段而非字符串拼接");
    println!("- span：跟踪操作的生命周期");
    println!("- instrument：自动为函数添加 span");
    println!("- 日志级别：trace, debug, info, warn, error");
}

pub fn demonstrate_logging_patterns() {
    println!("日志模式演示：");
    println!("1. 初始化订阅者（全局一次）");
    println!("2. 记录结构化事件");
    println!("3. 使用 span 跟踪操作范围");
    println!("4. instrument 宏自动化");
}