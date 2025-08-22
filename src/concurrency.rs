/*

# 05. 并发编程模式 - AI Coding 快速理解指南

---

## 🎯 业务场景对照表

| 类型 | 用途 | 典型示例 |
|------|------|----------|
| `join!/try_join!` | 等待多个任务全部完成，收集结果 | 批量 API 调用、文件并行处理 |
| `select!` | 等待多个任务，任一完成就继续 | 超时控制、条件等待 |
| `spawn` | 创建独立后台任务 | 日志写入、缓存刷新 |
| `Arc<Mutex<T>>` | 多任务共享可变状态 | 计数器、缓存更新 |

## 🔍 30秒识别要点

**快速判断方法：**

- 👀 **看等待方式**: `join!` = 全部完成，`select!` = 任一完成
- 🔧 **看错误传播**: `try_join!` 任一失败则整体失败  
- ⚠️ **看数据共享**: 是否有 `Arc<Mutex<T>>` 模式？

## ⚠️ AI 常见问题警告

> **危险信号** 🚨

- 🔴 **任务泄漏** 忘记 `.await` join handle，导致任务可能被取消
- 🔴 **panic 传播** spawn 后不处理 JoinError（任务 panic 会被忽略）  
- 🔴 **资源耗尽** 过度并发导致连接池耗尽
- 🔴 **数据竞争** 多个任务同时修改共享状态

## ✅ Code Review 检查清单

☐ 是否有数据竞争风险？（检查共享可变状态）  
☐ 错误是否正确传播和处理？  
☐ 是否有死锁可能？（如循环等待）  
☐ 资源是否合理限制？（连接池、并发数限制）  
☐ spawn 的任务是否有适当的错误处理？

## 📖 推荐阅读顺序

**Step 1: 并发模式识别**  
先看函数中的 `join!`/`select!`/`spawn` 使用模式

**Step 2: 数据共享检查**  
再看是否有 `Arc<Mutex<T>>` 等共享状态模式

**Step 3: 错误处理验证**  
最后看并发任务的错误处理是否完整

---

> 💡 **记住**: 并发不是银弹，要在复杂性和性能之间找平衡！

📖 阅读顺序：
1. 先看并发策略（join 等待全部 vs select 等待任一）
2. 再看错误处理（try_join 的短路 vs 个别任务容错）
3. 最后看资源管理（spawn 的生命周期，是否需要取消）

核心并发模式：

```rust
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
```

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