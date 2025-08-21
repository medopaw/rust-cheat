/*
===============================================================
07. 模式匹配 - AI Coding 快速理解指南
===============================================================

🎯 业务场景：
- 状态机处理：根据不同状态执行不同逻辑
- API 响应处理：解构复杂数据，提取需要的字段
- 配置解析：匹配不同的配置选项和参数组合
- 错误分类：根据错误类型进行不同处理

🔍 30秒识别模式匹配：
- 看 match：穷尽匹配，编译器保证所有分支都覆盖
- 看 if let：只关心一种情况，其他用 else 处理
- 看解构：从复杂数据结构中提取字段 (x, y), {name, age}
- 看守卫：Some(x) if x > 0，在匹配基础上加条件

⚠️ AI 常见模式匹配问题：
❌ 过度使用 unwrap() 而不是模式匹配处理 Option/Result
❌ 漏掉 match 分支，导致编译错误或逻辑缺失
❌ if let 链太长，应该用 match 更清晰
❌ 解构时忽略了所有权（不必要的 clone）
❌ 守卫条件过于复杂，影响可读性

✅ Review 清单：
- [ ] 所有 match 分支是否完整覆盖？
- [ ] 是否选择了合适的匹配方式（match vs if let）？
- [ ] 解构是否高效（避免不必要的 clone）？
- [ ] 模式是否表达了业务意图？
- [ ] 是否有过于复杂的守卫条件？

📖 阅读顺序：
1. 先看匹配对象的类型（Option, Result, enum, tuple）
2. 再看分支覆盖情况（是否有遗漏的情况）
3. 最后看每个分支的处理逻辑（是否合理）

核心模式示例：

// 基本的匹配模式示例
pub fn match_quick(opt_a: Option<i32>, opt_b: Option<i32>) {
    match (opt_a, opt_b) {
        (Some(a), Some(b)) => println!("both: {} {}", a, b),
        (Some(a), None)    => println!("only a: {}", a),
        _                  => println!("other"),
    }

    if let Some(x) = Some(10) {
        println!("x={}", x);
    } else {
        println!("none");
    }

    if let Err(e) = "42".parse::<i32>().map_err(|_| "parse err") {
        println!("err={}", e);
    }
}

// 解构和守卫的例子
let point = (3, 4);
match point {
    (0, 0) => println!("原点"),
    (x, 0) => println!("x轴上的点: {}", x),
    (0, y) => println!("y轴上的点: {}", y),
    (x, y) => println!("点 ({}, {})", x, y),
}

let num = Some(5);
match num {
    Some(x) if x > 3 => println!("大于3的数: {}", x),
    Some(x) => println!("小于等于3的数: {}", x),
    None => println!("无值"),
}
*/

pub fn match_quick(opt_a: Option<i32>, opt_b: Option<i32>) {
    match (opt_a, opt_b) {
        (Some(a), Some(b)) => println!("both: {} {}", a, b),
        (Some(a), None)    => println!("only a: {}", a),
        _                  => println!("other"),
    }

    if let Some(x) = Some(10) {
        println!("x={}", x);
    } else {
        println!("none");
    }

    if let Err(e) = "42".parse::<i32>().map_err(|_| "parse err") {
        println!("err={}", e);
    }
}

pub fn pattern_matching_demo() {
    println!("模式匹配演示：");
    
    // 基本匹配
    match_quick(Some(1), Some(2));
    match_quick(Some(1), None);
    match_quick(None, None);
    
    // 解构匹配
    let point = (3, 4);
    match point {
        (0, 0) => println!("原点"),
        (x, 0) => println!("x轴上的点: {}", x),
        (0, y) => println!("y轴上的点: {}", y),
        (x, y) => println!("点 ({}, {})", x, y),
    }
    
    // 守卫条件
    let num = Some(5);
    match num {
        Some(x) if x > 3 => println!("大于3的数: {}", x),
        Some(x) => println!("小于等于3的数: {}", x),
        None => println!("无值"),
    }
}