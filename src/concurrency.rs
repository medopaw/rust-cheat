/*
===============================================================
05. 并发编程模式 - AI Coding 快速理解指南
===============================================================

🎯 业务场景：
- Web 服务：并发处理多个请求，提升吞吐量
- 批量操作：同时处理多个 API 调用、文件操作
- 超时控制：网络请求、数据库查询的时间限制
- 后台任务：日志写入、缓存刷新、定时任务

🔍 30秒识别并发模式：
- 看 join!/try_join!：等待多个任务全部完成，收集结果
- 看 select!：等待多个任务，任一完成就继续（常用于超时）
- 看 spawn：创建独立后台任务，不阻塞当前执行
- 看错误传播：try_join! 任一失败则整体失败

⚠️ AI 常见并发错误：
❌ 忘记 .await join handle，导致任务可能被取消
❌ 在 select! 中使用可能 panic 的代码
❌ spawn 后不处理 JoinError（任务 panic 会被忽略）
❌ 过度并发导致资源耗尽（如打开太多网络连接）
❌ 数据竞争：多个任务同时修改共享状态

✅ Review 清单：
- [ ] 是否有数据竞争风险？（检查共享可变状态）
- [ ] 错误是否正确传播和处理？
- [ ] 是否有死锁可能？（如循环等待）
- [ ] 资源是否合理限制？（连接池、并发数限制）
- [ ] spawn 的任务是否有适当的错误处理？

📖 阅读顺序：
1. 先看并发策略（join 等待全部 vs select 等待任一）
2. 再看错误处理（try_join 的短路 vs 个别任务容错）
3. 最后看资源管理（spawn 的生命周期，是否需要取消）

核心并发模式：

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