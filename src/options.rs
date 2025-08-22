/*

# 01. Option/Result 模式 - AI Coding 快速理解指南

---

## 🎯 业务场景对照表

| 类型 | 用途 | 典型示例 |
|------|------|----------|
| `Option<T>` | 查找操作，可能找不到 | 用户查询、配置获取 |
| `Result<T,E>` | 可能失败的操作 | 文件读取、网络请求、解析 |
| `Result<Option<T>, E>` | 外部操作可能出错，成功后可能为空 | 数据库查询 |

## 🔍 30秒识别要点

**快速判断方法：**

- 👀 **看返回类型**: `Option` = 可能为空，`Result` = 可能出错
- 🔧 **看调用方式**: `match`/`if let` (处理分支), `?` (错误传播)  
- ⚠️ **看错误处理**: 是否到处 `unwrap()` 还是优雅处理？

## ⚠️ AI 常见问题警告

> **危险信号** 🚨

- 🔴 **过度使用** `unwrap()`/`expect()` 导致 panic
- 🔴 **概念混淆** `Result<Option<T>, E>` 中混淆"错误"和"空结果"  
- 🔴 **分支遗漏** 忘记处理 `None`/`Err` 分支，导致逻辑不完整
- 🔴 **调试困难** 错误信息不明确，难以调试

## ✅ Code Review 检查清单

☐ 错误是否正确传播（优先使用 `?` 而非 `unwrap`）  
☐ 是否区分了"操作失败"和"结果为空"  
☐ 所有可能的分支是否都有合理处理  
☐ 错误信息是否有助于定位问题  

## 📖 推荐阅读顺序

**Step 1: 函数签名分析**  
先看函数签名的返回类型（判断是查找还是可能失败的操作）

**Step 2: 调用方式检查**  
再看调用方如何处理返回值（`match` 还是 `?` 还是 `unwrap`）

**Step 3: 错误分支验证**  
最后看错误分支是否完整（特别注意 `Result<Option<T>, E>` 的**三种情况**）

---

> 💡 **记住**: AI 写的代码能编译通过，但错误处理是否优雅需要你来判断！

*/

// 🎯 这是AI最常写的"查找"操作模式
// review时看：返回类型是Option = 可能找不到，不是错误
// AI经常在这里过度使用unwrap()，要检查
fn find_name(id: u64) -> Option<&'static str> {
    if id == 1 { Some("alice") } else { None }
}

// 🎯 这是AI最常写的"可能失败"操作模式  
// review时看：parse、file、network操作常返回Result
// 注意map_err是在转换错误类型，不是处理错误
fn read_number() -> Result<i32, &'static str> {
    "42".parse::<i32>().map_err(|_| "parse error")
}

// 🎯 这是AI写数据库/API调用时的典型模式
// review时看：要区分"操作失败"vs"查询成功但无结果"
// 常见bug：把"没找到"当成错误返回
fn find_user(id: u64) -> Result<Option<&'static str>, &'static str> {
    if id == 0 { return Err("db offline") }  // 操作失败：数据库问题
    Ok(if id == 1 { Some("alice") } else { None })  // 查询成功：可能有结果或无结果
}

// 🎯 训练：快速识别Option处理模式
// AI常用if let，review时看是否遗漏了None分支的处理
pub fn option_demo() {
    println!("=== 快速识别：Option<T> 处理模式 ===");
    
    // 模式1：if let（当你只关心Some的情况）
    if let Some(name) = find_name(1) {
        println!("✅ 找到用户: {}", name);
    } else {
        println!("用户不存在");
    }
    
    // 模式2：match（当你要明确处理两种情况）
    match find_name(999) {
        Some(name) => println!("找到: {}", name),
        None => println!("用户999不存在"),  // AI经常忘记这个分支
    }
}

// 🎯 训练：快速识别错误传播模式
// review重点：看?操作符的使用，避免到处unwrap()
pub fn result_demo() -> Result<(), &'static str> {
    println!("=== 快速识别：Result<T,E> 错误传播 ===");
    
    // ? 操作符自动处理错误传播：Ok(n)->解包，Err(e)->提前返回
    let n = read_number()?;  // 这一行：成功得到i32，失败直接返回Err
    println!("✅ 解析成功: {}", n);
    Ok(())
}

// 🎯 训练：快速识别三态处理模式（AI最容易搞错的地方）
// review重点：确保三种情况都有合理处理，别把"空结果"当错误
pub fn result_option_demo() {
    println!("=== 快速识别：Result<Option<T>,E> 三态处理 ===");
    
    // 必须处理三种情况，AI经常遗漏或搞混
    match find_user(1) {
        Ok(Some(u)) => println!("✅ 操作成功+有结果: {}", u),
        Ok(None) => println!("⚠️ 操作成功+无结果"),
        Err(e) => println!("❌ 操作失败: {}", e),
    }
    
    // 测试错误情况（数据库离线）
    match find_user(0) {
        Ok(Some(u)) => println!("找到用户: {}", u),
        Ok(None) => println!("用户不存在"),
        Err(e) => println!("数据库错误: {}", e),  // 这里会走这个分支
    }
}

// 🎯 训练：识别AI常见的错误处理问题
// review重点：发现性能浪费(不必要的clone)和panic风险(unwrap)
pub fn error_handling_patterns() {
    println!("=== Review训练：AI常见错误处理问题 ===");
    
    // ❌ AI常见问题：直接unwrap()可能导致panic
    // let name = find_name(999).unwrap();  // 危险！会panic
    
    // ✅ 正确模式1：unwrap_or提供默认值
    let name = find_name(999).unwrap_or("匿名用户");
    println!("用户名: {}", name);
    
    // ✅ 正确模式2：map转换+unwrap_or_else
    let formatted = find_name(1)
        .map(|name| format!("用户: {}", name))  // 只在Some时执行转换
        .unwrap_or_else(|| "用户不存在".to_string());
    println!("{}", formatted);
}

// 🎯 实际模拟：用户管理系统的查找功能
// 这是AI经常写的业务逻辑模式，练习快速抓住主流程
pub fn realistic_user_lookup() {
    println!("=== 实际场景：用户查找系统 ===");
    
    // 模拟AI写的典型业务逻辑
    let user_id = 1;
    
    // 第一步：查找用户基本信息
    match find_user(user_id) {
        Ok(Some(username)) => {
            println!("步骤1: 找到用户 {}", username);
            
            // 第二步：查找用户详细信息
            if let Some(name) = find_name(user_id) {
                println!("步骤2: 获取详细信息 {}", name);
            }
        },
        Ok(None) => println!("用户{}不存在", user_id),
        Err(e) => {
            println!("数据库查询失败: {}", e);
            return;  // 提前退出，避免后续操作
        }
    }
}

// 🎯 主演示函数：按学习顺序展示所有模式
// 这样运行时能看到完整的学习流程
pub fn run_all_demos() {
    println!("🦀 Option/Result模式 - AI代码快速理解训练");
    println!("=====================================");
    
    option_demo();
    println!();
    
    if let Err(e) = result_demo() {
        println!("❌ Result演示失败: {}", e);
    }
    println!();
    
    result_option_demo();
    println!();
    
    error_handling_patterns();
    println!();
    
    realistic_user_lookup();
}
