/*

# 07. 模式匹配 - AI Coding 快速理解指南

---

## 🎯 业务场景对照表

| 类型 | 用途 | 典型示例 |
|------|------|----------|
| `match` | 穷尽匹配，必须处理所有情况 | 状态机处理、错误分类 |
| `if let` | 只关心一种情况，其他忽略 | Option 值提取、单一条件检查 |
| 解构 | 从复杂结构中提取字段 | API 响应处理、参数解析 |
| 守卫 | 在匹配基础上加条件 | 数值范围检查、条件过滤 |

## 🔍 30秒识别要点

**快速判断方法：**

- 👀 **看匹配类型**: `match` = 全覆盖，`if let` = 单一关注
- 🔧 **看解构模式**: `(x, y)`, `{name, age}` 提取字段  
- ⚠️ **看所有权**: 是否不必要的 `clone()`？

## ⚠️ AI 常见问题警告

> **危险信号** 🚨

- 🔴 **过度 unwrap** 使用 `unwrap()` 而不是模式匹配处理 Option/Result
- 🔴 **分支不全** 漏掉 match 分支，导致编译错误或逻辑缺失  
- 🔴 **if let 链** 链式 if let 太长，应该用 match 更清晰
- 🔴 **所有权浪费** 解构时不必要的 clone()

## ✅ Code Review 检查清单

☐ 所有 match 分支是否完整覆盖？  
☐ 是否选择了合适的匹配方式（match vs if let）？  
☐ 解构是否高效（避免不必要的 clone）？  
☐ 模式是否表达了业务意图？  
☐ 是否有过于复杂的守卫条件？

## 📖 推荐阅读顺序

**Step 1: 匹配覆盖检查**  
先看 match 分支是否完整，有无遗漏

**Step 2: 所有权分析**  
再看解构和移动语义是否合理

**Step 3: 性能优化验证**  
最后看是否有不必要的 clone 或复杂匹配

---

> 💡 **记住**: 模式匹配让编译器帮你找 bug，利用好这个特性！

📖 阅读顺序：
1. 先看匹配对象的类型（Option, Result, enum, tuple）
2. 再看分支覆盖情况（是否有遗漏的情况）
3. 最后看每个分支的处理逻辑（是否合理）

核心模式示例：

```rust
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

    if let Err(e) = "abc".parse::<i32>().map_err(|_| "parse err") {
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
```

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

    if let Err(e) = "abc".parse::<i32>().map_err(|_| "parse err") {
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