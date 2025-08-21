/*
===============================================================
02. async/await 与 block_on
===============================================================

// .await：非阻塞等待，挂起当前 async 任务，runtime 可去跑别的任务
// block_on：阻塞当前线程，把一个 Future 同步跑到完成（常在最外层使用）
// 伪代码说明（需 futures/tokio 依赖，这里仅演示语义）：
use futures::executor::block_on;
async fn say() -> &'static str { "hello" }
fn main() {
    let out = block_on(say()); // 阻塞直到完成
    println!("{}", out);
}

===============================================================
03. await? 展开 & 类型流
===============================================================

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