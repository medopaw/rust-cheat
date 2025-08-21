/*
===============================================================
06. 并发骨架：join!/try_join!/select!/spawn（tokio）
===============================================================

// 基本的并发模式（需 tokio 依赖）
use tokio::try_join;
async fn a() -> anyhow::Result<i32> { Ok(1) }
async fn b() -> anyhow::Result<i32> { Ok(2) }
async fn run_join() -> anyhow::Result<()> {
    let (x, y) = try_join!(a(), b())?; // 任一 Err 直接返回 Err
    println!("{}", x + y);
    Ok(())
}

use tokio::{select, time::{sleep, Duration}};
async fn run_select() {
    select! {
        _ = sleep(Duration::from_secs(1)) => println!("timeout"),
        _ = async_work() => println!("done"),
    }
}

// 后台任务：
let handle = tokio::spawn(async { 42 });
let val = handle.await?; // JoinHandle<Result<T, JoinError>>
*/

pub fn concurrency_concepts() {
    println!("并发概念演示：");
    println!("- join! : 等待所有任务完成");
    println!("- try_join! : 等待所有任务完成，任一失败则返回错误");
    println!("- select! : 等待任一任务完成");
    println!("- spawn : 创建独立的后台任务");
}

// 模拟并发操作
pub fn simulate_concurrent_work() {
    println!("模拟并发工作：");
    println!("1. 启动多个任务");
    println!("2. 等待所有任务完成");
    println!("3. 处理结果或错误");
}