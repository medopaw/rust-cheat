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

// 🎯 这是AI写异步代码时最常用的模式
// review时看：async fn调用必须.await，#[tokio::main]说明需要runtime
use std::time::Duration;
use tokio::time::sleep;

// 模拟一个异步API调用（AI最常写的模式）
async fn fetch_user_data(user_id: u32) -> Result<String, &'static str> {
    if user_id == 0 {
        return Err("invalid user id");
    }
    
    // 模拟网络延迟
    sleep(Duration::from_millis(100)).await;
    
    Ok(format!("User data for ID: {}", user_id))
}

// 🎯 训练：识别async函数的调用链
// review重点：每个async函数调用都有.await，错误用?传播
pub async fn async_chain_demo() -> Result<(), &'static str> {
    println!("=== 快速识别：async函数调用链 ===");
    
    // 类型流转：fetch_user_data() -> Future<Result<String, &str>>
    //          fetch_user_data().await -> Result<String, &str>  
    //          fetch_user_data().await? -> String
    let user_data = fetch_user_data(1).await?;
    println!("✅ 获取用户数据: {}", user_data);
    
    // AI常见模式：在async函数中调用其他async函数
    let user_count = count_users().await;
    println!("用户总数: {}", user_count);
    
    Ok(())
}

// 🎯 另一个异步函数示例（AI常写的统计类操作）
async fn count_users() -> u32 {
    sleep(Duration::from_millis(50)).await;  // 模拟查询延迟
    42  // 返回用户数量
}

// 🎯 训练：识别异步并发模式
// review重点：AI可能不知道tokio::join!，会串行执行本该并行的任务
pub async fn concurrency_demo() {
    println!("=== 快速识别：异步并发模式 ===");
    
    // ❌ AI常见问题：串行执行（低效）
    println!("串行执行（低效）:");
    let start = std::time::Instant::now();
    let _user1 = fetch_user_data(1).await;
    let _user2 = fetch_user_data(2).await;
    println!("串行耗时: {:?}", start.elapsed());
    
    // ✅ 正确模式：并行执行
    println!("并行执行（高效）:");
    let start = std::time::Instant::now();
    let (result1, result2) = tokio::join!(
        fetch_user_data(1),
        fetch_user_data(2)
    );
    println!("并行耗时: {:?}", start.elapsed());
    println!("结果1: {:?}, 结果2: {:?}", result1, result2);
}

// 🎯 实际场景：用户数据获取系统
// 这是AI写微服务时的典型模式，练习快速抓住异步逻辑
pub async fn realistic_user_service() {
    println!("=== 实际场景：用户服务异步逻辑 ===");
    
    let user_id = 1;
    
    // 步骤1：获取用户基本信息
    match fetch_user_data(user_id).await {
        Ok(user_data) => {
            println!("步骤1: 获取用户数据成功");
            
            // 步骤2：并发获取相关数据
            let (user_count, _permissions) = tokio::join!(
                count_users(),
                async { "admin" }  // 模拟权限查询
            );
            
            println!("步骤2: 用户总数={}, 用户数据={}", user_count, user_data);
        },
        Err(e) => {
            println!("❌ 获取用户数据失败: {}", e);
            return;
        }
    }
}

// 🎯 概念演示：async vs sync的区别
// 帮助理解AI什么时候会选择async
pub fn async_concepts_explanation() {
    println!("=== async/await 核心概念理解 ===");
    println!("- async fn: 返回Future，调用时需要.await");
    println!("- .await: 非阻塞等待，让出线程给其他任务");
    println!("- #[tokio::main]: 创建异步runtime执行async main");
    println!("- AI选择async的场景: 网络IO、文件IO、数据库操作");
}

// 🎯 主演示函数：展示所有异步模式
pub async fn run_all_demos() {
    println!("🚀 Async/Await模式 - AI代码快速理解训练");
    println!("=======================================");
    
    async_concepts_explanation();
    println!();
    
    if let Err(e) = async_chain_demo().await {
        println!("❌ 异步链演示失败: {}", e);
    }
    println!();
    
    concurrency_demo().await;
    println!();
    
    realistic_user_service().await;
}