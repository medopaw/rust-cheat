/*
===============================================================
02. async/await 模式 - AI Coding 快速理解指南
===============================================================

🎯 业务场景：
- Web 服务、API 客户端、数据库操作、文件 I/O
- 高并发场景（相比线程，async 任务开销更小）
- I/O 密集型应用（等待网络/磁盘时不阻塞线程）

🔍 30秒识别 async 代码：
- 看函数签名：async fn -> 返回 Future，调用时需要 .await
- 看调用链：fetch().await? 的类型流转（Future -> Result -> T）
- 看 main 函数：#[tokio::main] 或 block_on 包装
- 看错误处理：async 中的 ? 如何在异步上下文中传播

⚠️ AI 常见问题：
❌ 忘记 .await，导致得到 Future 而不是实际值
❌ 在同步上下文中直接调用 async 函数
❌ 混用 block_on 和 .await（可能导致死锁）
❌ 在 async fn 中使用阻塞的同步 I/O（如 std::fs）

✅ Review 清单：
- [ ] async fn 的所有调用都有 .await 吗？
- [ ] 错误类型是否兼容（实现了 Into<Error>）？
- [ ] 是否避免了同步 I/O（用 tokio::fs 而非 std::fs）？
- [ ] 是否在合适的地方使用 spawn 来并发执行？

📖 阅读顺序：
1. 先看 main 函数的 async 包装（#[tokio::main] 还是 block_on）
2. 再看 async fn 调用链，确认每个异步调用都有 .await
3. 最后看错误类型流转，确认 ? 操作符的类型匹配

类型流转关键理解：
// 假设：async fn fetch() -> Result<String, SomeError>
// 则：
// 1) fetch() 的类型：impl Future<Output = Result<String, SomeError>>
// 2) fetch().await 的类型：Result<String, SomeError>
// 3) let body: String = fetch().await?;
//    - 若 Ok(s) -> 表达式值为 s（String）
//    - 若 Err(e) -> 提前 return Err(e.into()) 到当前函数的返回类型

async fn run() -> Result<(), SomeError> {
    let body: String = fetch().await?;
    Ok(())
}

async fn run_anyhow() -> anyhow::Result<()> {
    let body: String = fetch().await?; // SomeError -> anyhow::Error（自动 Into）
    Ok(())
}

// .await：非阻塞等待，挂起当前 async 任务，runtime 可去跑别的任务
// block_on：阻塞当前线程，把一个 Future 同步跑到完成（常在最外层使用）
use futures::executor::block_on;
async fn say() -> &'static str { "hello" }
fn main() {
    let out = block_on(say()); // 阻塞直到完成
    println!("{}", out);
}
*/

pub fn async_concepts() {
    println!("async/await 概念演示：");
    println!("- .await：非阻塞等待，挂起当前 async 任务");
    println!("- block_on：阻塞当前线程直到 Future 完成");
}

pub fn await_types_demo() {
    println!("await? 类型流演示：");
    println!("1) fetch() -> impl Future<Output = Result<String, Error>>");
    println!("2) fetch().await -> Result<String, Error>");
    println!("3) fetch().await? -> String (或提前返回 Err)");
}