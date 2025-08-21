/*
===============================================================
01. Option/Result 模式 - AI Coding 快速理解指南
===============================================================

🎯 业务场景：
- Option<T>: 查找操作，可能找不到（如用户查询、配置获取）
- Result<T,E>: 可能失败的操作（如文件读取、网络请求、解析）
- Result<Option<T>, E>: 外部操作可能出错，成功后可能为空（如数据库查询）

🔍 30秒识别要点：
- 看返回类型：Option = 可能为空，Result = 可能出错
- 看调用方式：match/if let (处理分支), ? (错误传播)
- 看错误处理：是否到处 unwrap() 还是优雅处理？

⚠️ AI 常见问题：
❌ 过度使用 unwrap()/expect() 导致 panic
❌ Result<Option<T>, E> 中混淆"错误"和"空结果"
❌ 忘记处理 None/Err 分支，导致逻辑不完整
❌ 错误信息不明确，难以调试

✅ Review 清单：
- [ ] 错误是否正确传播（优先使用 ? 而非 unwrap）
- [ ] 是否区分了"操作失败"和"结果为空"
- [ ] 所有可能的分支是否都有合理处理
- [ ] 错误信息是否有助于定位问题

📖 阅读顺序：
1. 先看函数签名的返回类型（判断是查找还是可能失败的操作）
2. 再看调用方如何处理返回值（match 还是 ? 还是 unwrap）
3. 最后看错误分支是否完整（特别注意 Result<Option<T>, E> 的三种情况）

// Option<T>：成功且无错误，只是可能为空
fn find_name(id: u64) -> Option<&'static str> {
    if id == 1 { Some("alice") } else { None }
}

// Result<T, E>：操作可能失败
fn read_number() -> Result<i32, &'static str> {
    "42".parse::<i32>().map_err(|_| "parse error")
}

// Result<Option<T>, E>：三态（错误 / 成功但空 / 成功且有值）
fn find_user(id: u64) -> Result<Option<&'static str>, &'static str> {
    if id == 0 { return Err("db offline") }
    Ok(if id == 1 { Some("alice") } else { None })
}
*/

fn find_name(id: u64) -> Option<&'static str> {
    if id == 1 { Some("alice") } else { None }
}

fn read_number() -> Result<i32, &'static str> {
    "42".parse::<i32>().map_err(|_| "parse error")
}

fn find_user(id: u64) -> Result<Option<&'static str>, &'static str> {
    if id == 0 { return Err("db offline") }
    Ok(if id == 1 { Some("alice") } else { None })
}

pub fn option_demo() {
    if let Some(name) = find_name(1) {
        println!("found: {}", name);
    } else {
        println!("not found");
    }
}

pub fn result_demo() -> Result<(), &'static str> {
    let n = read_number()?; // Ok(n) -> n；Err(e) -> 提前返回 Err(e)
    println!("n={}", n);
    Ok(())
}

pub fn result_option_demo() {
    match find_user(1) {
        Ok(Some(u)) => println!("user={}", u),
        Ok(None)    => println!("user not found"),
        Err(e)      => println!("error: {}", e),
    }
}