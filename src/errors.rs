/*
===============================================================
03. 错误处理模式 - AI Coding 快速理解指南  
===============================================================

🎯 业务场景：
- 应用层错误聚合：多种不同错误类型统一处理
- 用户面向的错误信息：添加上下文便于调试定位
- CLI 工具、Web 服务的错误处理链

🔍 30秒识别错误处理模式：
- 看返回类型：anyhow::Result<T> = 应用层统一错误
- 看 ? 操作符：自动错误转换和提前返回
- 看 with_context()：添加调试上下文信息
- 看 map_err()：手动错误类型转换

⚠️ AI 常写的反模式：
❌ 到处使用 unwrap()/expect() 导致程序 panic
❌ 错误信息不明确（"Something went wrong"）
❌ 混用不同错误处理策略（anyhow + 自定义错误）
❌ 在库层使用 anyhow（应该用 thiserror 自定义错误）
❌ 忽略错误上下文，难以定位问题源头

✅ Review 清单：
- [ ] 用户可控输入是否有验证和合适的错误提示？
- [ ] 错误信息是否包含足够的调试信息？
- [ ] 是否避免了 panic!（除非真的不可恢复）？
- [ ] 错误传播路径是否清晰（? 链条）？
- [ ] 是否在合适的层次使用了 with_context？

📖 阅读顺序：
1. 先看函数返回类型（anyhow::Result 还是自定义错误）
2. 再看错误产生点（哪些操作可能失败）
3. 最后看错误处理（? 传播还是 match 处理，是否有上下文）

*/

// 🎯 AI常用的错误处理库：anyhow用于应用层统一错误
use anyhow::{Context, Result, anyhow, bail};
use std::fs;

// 🎯 这是AI写应用层代码时最常用的错误处理模式
// review时看：anyhow::Result统一各种错误，with_context添加调试信息
fn load_config_file(path: &str) -> Result<String> {
    let content = fs::read_to_string(path)
        .with_context(|| format!("Failed to read config file: {}", path))?;
    Ok(content)
}

// 🎯 另一个典型模式：解析配置文件
fn parse_config(content: &str) -> Result<serde_json::Value> {
    serde_json::from_str(content)
        .with_context(|| "Failed to parse JSON config")
}

// 🎯 训练：识别错误传播链
// review重点：每个?都是潜在的错误传播点，要检查是否合理
pub fn error_chain_demo() -> Result<()> {
    println!("=== 快速识别：错误传播链 ===");
    
    // 错误传播链：文件读取 -> JSON解析
    let config_content = load_config_file("config.json")?;  // 可能的文件错误
    let _config = parse_config(&config_content)?;  // 可能的解析错误
    
    println!("✅ 配置加载成功");
    Ok(())
}

// 🎯 训练：识别手动错误创建模式
// AI常用anyhow!和bail!宏来创建自定义错误
pub fn custom_error_demo() -> Result<()> {
    println!("=== 快速识别：自定义错误模式 ===");
    
    let user_id = 0;
    
    // 模式1：用anyhow!宏创建错误
    if user_id == 0 {
        return Err(anyhow!("Invalid user ID: {}", user_id));
    }
    
    // 模式2：用bail!宏直接返回错误（更简洁）
    let permission_level = 1;
    if permission_level < 5 {
        bail!("Insufficient permissions: level {}", permission_level);
    }
    
    Ok(())
}

// 🎯 训练：识别多层错误处理
// 这是AI写复杂业务逻辑时的常见模式
pub fn nested_error_handling() -> Result<()> {
    println!("=== 快速识别：多层错误处理 ===");
    
    // 第一层：配置检查
    match validate_config() {
        Ok(_) => println!("配置验证通过"),
        Err(e) => {
            println!("配置验证失败: {}", e);
            return Ok(()); // 继续执行，不传播错误
        }
    }
    
    // 第二层：用户认证（这里会传播错误）
    authenticate_user()?;
    
    Ok(())
}

// 模拟配置验证函数
fn validate_config() -> Result<()> {
    let config_exists = false; // 模拟检查结果
    if !config_exists {
        bail!("Configuration file missing");
    }
    Ok(())
}

// 模拟用户认证函数
fn authenticate_user() -> Result<()> {
    let auth_token = None::<String>; // 模拟认证令牌
    match auth_token {
        Some(_) => Ok(()),
        None => Err(anyhow!("Authentication token not found")),
    }
}

// 🎯 实际场景：应用初始化流程
// 这是AI写CLI或服务器应用时的典型模式
pub fn realistic_app_initialization() -> Result<()> {
    println!("=== 实际场景：应用初始化流程 ===");
    
    // 步骤1：加载配置
    println!("步骤1: 加载配置文件...");
    // 注意：这里故意让文件不存在来演示错误处理
    match load_config_file("nonexistent.json") {
        Ok(_) => println!("配置加载成功"),
        Err(e) => {
            println!("配置加载失败: {}", e);
            println!("使用默认配置继续运行...");
        }
    }
    
    // 步骤2：初始化数据库连接（模拟）
    println!("步骤2: 初始化数据库连接...");
    init_database()
        .with_context(|| "Database initialization failed during startup")?;
    
    println!("✅ 应用初始化完成");
    Ok(())
}

// 模拟数据库初始化
fn init_database() -> Result<()> {
    let db_available = true; // 模拟数据库状态
    if db_available {
        Ok(())
    } else {
        bail!("Database connection failed")
    }
}

// 🎯 演示错误处理最佳实践vs常见问题
pub fn error_handling_patterns() {
    println!("=== Review训练：错误处理最佳实践 ===");
    
    // ✅ 好的做法：适当的错误上下文
    println!("✅ 正确：使用with_context提供调试信息");
    
    // ❌ AI常见问题演示（已注释掉避免panic）
    // println!("❌ 错误：直接unwrap可能导致panic");
    // let _result = load_config_file("bad.json").unwrap(); // 危险！
    
    // ✅ 正确的处理方式
    match load_config_file("bad.json") {
        Ok(content) => println!("配置内容: {}", content),
        Err(e) => println!("配置加载失败，使用默认设置: {}", e),
    }
}

// 🎯 主演示函数：展示所有错误处理模式
pub fn run_all_demos() -> Result<()> {
    println!("❌ 错误处理模式 - AI代码快速理解训练");
    println!("=====================================");
    
    error_handling_patterns();
    println!();
    
    // 注意：这些函数可能返回错误，但我们要继续演示
    if let Err(e) = error_chain_demo() {
        println!("错误链演示失败（正常，用于演示）: {}", e);
    }
    println!();
    
    if let Err(e) = custom_error_demo() {
        println!("自定义错误演示失败（正常，用于演示）: {}", e);
    }
    println!();
    
    if let Err(e) = nested_error_handling() {
        println!("多层错误处理演示失败: {}", e);
    }
    println!();
    
    if let Err(e) = realistic_app_initialization() {
        println!("应用初始化演示失败: {}", e);
    }
    
    Ok(())
}