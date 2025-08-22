/*

# 06. 日志与可观测性 - AI Coding 快速理解指南

---

## 🎯 业务场景对照表

| 类型 | 用途 | 典型示例 |
|------|------|----------|
| `info!` | 业务流程记录 | 用户登录、API 调用 |
| `warn!` | 非致命问题 | 配置缺失、性能警告 |
| `error!` | 必须关注的错误 | 数据库失连、API 失败 |
| `#[instrument]` | 跟踪函数执行范围 | 分布式追踪、性能监控 |

## 🔍 30秒识别要点

**快速判断方法：**

- 👀 **看初始化**: subscriber 设置（全局一次，通常在 main 中）
- 🔧 **看结构化**: `info!(user_id = 42, "message")` 而非字符串拼接  
- ⚠️ **看 span 使用**: `#[instrument]` 或手动 span，跟踪操作范围

## ⚠️ AI 常见问题警告

> **危险信号** 🚨

- 🔴 **滥用 println!** 使用 `println!` 而不是日志库（无法配置级别和格式）
- 🔴 **结构不合理** 日志信息不够结构化（难以查询和分析）  
- 🔴 **泄漏敏感信息** 记录敏感信息（密码、token）
- 🔴 **上下文不足** 错误日志没有足够的定位信息

## ✅ Code Review 检查清单

☐ 是否避免记录敏感信息（密码、API key）？  
☐ 错误日志是否包含足够的上下文信息？  
☐ 是否使用了合适的日志级别？  
☐ 结构化字段是否便于查询和过滤？  
☐ 关键业务操作是否有适当的日志记录？

## 📖 推荐阅读顺序

**Step 1: 日志初始化检查**  
先看 main 函数中的 subscriber 设置

**Step 2: 结构化日志分析**  
再看日志调用是否使用了结构化字段

**Step 3: 敏感信息排查**  
最后看是否有敏感信息泄漏风险

---

> 💡 **记住**: 好的日志是生产环境调试的生命线！

📖 阅读顺序：
1. 先看日志初始化（subscriber 配置，日志级别设置）
2. 再看日志使用（event vs span，结构化字段）
3. 最后看敏感信息处理（skip 字段，错误信息脱敏）

核心日志模式：

```rust
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
```

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