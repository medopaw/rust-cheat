/*

# 04. 迭代器与集合操作 - AI Coding 快速理解指南

---

## 🎯 业务场景对照表

| 类型 | 用途 | 典型示例 |
|------|------|----------|
| `iter().map().filter()` | 数据转换管道 | API 响应处理、数据清洗 |
| `fold`/`reduce` | 数据聚合计算 | 统计分析、累积计算 |
| `entry().or_insert()` | HashMap 高效更新 | 频次统计、缓存管理 |
| `group_by` | 数据分组处理 | 日志分析、分类汇总 |

## 🔍 30秒识别要点

**快速判断方法：**

- 👀 **看链式调用**: `data.iter().map().filter().collect()` 典型风格
- 🔧 **看聚合操作**: `fold`/`reduce` 累积，`group_by` 分组  
- ⚠️ **看 HashMap 模式**: `entry().or_insert()` 避免重复查找

## ⚠️ AI 常见问题警告

> **危险信号** 🚨

- 🔴 **性能浪费** 不必要的 `clone()` 和多次 HashMap 查找
- 🔴 **链式过长** 迭代器链太复杂，影响可读性  
- 🔴 **collect 滥用** 每步都 collect，而不是保持惰性计算
- 🔴 **类型推断** 复杂泛型导致编译器无法推断类型

## ✅ Code Review 检查清单

☐ 是否避免了不必要的 clone() 和中间 collect()?  
☐ HashMap 更新是否使用了 entry() API？  
☐ 迭代器链是否保持了合理的复杂度？  
☐ 是否选择了合适的聚合操作（fold vs reduce）？

## 📖 推荐阅读顺序

**Step 1: 数据流分析**  
先看数据从输入到输出的转换链条

**Step 2: 性能热点检查**  
再看是否有性能问题（clone、重复查找）

**Step 3: 可读性优化**  
最后看代码可读性和维护性

---

> 💡 **记住**: 迭代器是零成本抽象，但要避免不必要的中间分配！

# ⚠️ AI 常见性能陷阱

> **代码异味** 🦨

- 🔴 **过度收集** 使用 `collect()` 产生中间集合，影响性能
- 🔴 **重复查找** 在循环中重复查找 HashMap（用 `entry` API 更高效）
- 🔴 **无意义克隆** 不必要的 `clone()`（特别是在 `fold` 中）
- 🔴 **混合范式** 命令式循环 + 函数式链式调用，可读性差
- 🔴 **忘记惰性** Iterator 是 lazy 的，需要消费才会执行

# ✅ Performance Review 检查清单

☐ 是否高效使用了 `entry` API 而非多次 `get`/`insert`？  
☐ 是否避免了不必要的 `clone()` 和中间集合？  
☐ 链式调用是否清晰易读（过长时考虑拆分）？  
☐ 是否选择了合适的数据结构（`HashMap` vs `BTreeMap`）？  
☐ 聚合操作是否处理了空集合情况？  

# 📖 推荐代码阅读顺序

**1️⃣ 数据流向分析**  
**输入** → **迭代器链** → **输出类型**

**2️⃣ 关键操作理解**  
`map`/`filter`/`fold` 的闭包逻辑

**3️⃣ 性能热点检查**

`clone`、`collect`、HashMap 操作

---

# 📝 核心模式速查

**累积计算模式**

```rust
let sum = [1,2,3].iter().fold(0, |acc, x| acc + x);
```

**高效更新模式**  

```rust
let counter = freq.entry(key).or_insert(0);
*counter += 1;
```

**数据聚合模式**

```rust
items.iter().fold(HashMap::new(), |mut acc, item| {
    acc.entry(item.key.clone()).or_default().push(item);
    acc
})
```

*/

// 🎯 AI最常写的模式：HashMap entry API 和迭代器链
// review时看：entry()避免重复查找，fold用于累积计算
use std::collections::HashMap;

// 🎯 训练：识别 fold 聚合模式
// review重点：fold的初始值和累积逻辑是否正确
pub fn fold_sum_demo() {
    println!("=== 快速识别：fold 累积模式 ===");
    
    // 基础fold：累积求和
    let sum = [1,2,3].iter().fold(0, |acc, x| acc + x);
    assert_eq!(sum, 6);
    println!("fold 求和结果: {}", sum);
    
    // 更复杂的fold：字符串拼接
    let words = ["hello", "world", "rust"];
    let sentence = words.iter().fold(String::new(), |mut acc, word| {
        if !acc.is_empty() { acc.push(' '); }  // 添加空格分隔
        acc.push_str(word);
        acc
    });
    println!("fold 拼接结果: {}", sentence);
}

// 🎯 训练：识别 HashMap entry API 的高效模式
// 这是AI最容易搞错的地方！review时一定要仔细看
pub fn word_freq<'a>(words: &'a [&'a str]) -> HashMap<&'a str, usize> {
    let mut freq = HashMap::new();
    for w in words {
        // entry API 的核心：一次查找，返回可变引用
        let counter = freq.entry(*w).or_insert(0);
        *counter += 1;
    }
    freq
}

// 🎯 演示数据结构：分组操作常用的结构
#[derive(Clone, Debug)]
pub struct Item { 
    pub key: String, 
    pub val: i32 
}

// 🎯 训练：识别 fold + entry 组合模式（AI常写的复杂聚合）
// review重点：fold中的HashMap操作，注意clone的使用是否必要
pub fn group_by_key(items: &[Item]) -> HashMap<String, Vec<Item>> {
    items.iter().cloned().fold(HashMap::new(), |mut acc, item| {
        acc.entry(item.key.clone()).or_default().push(item);  // or_default创建空Vec
        acc  // 返回累积器
    })
}

// 🎯 训练：识别迭代器链式调用模式
// review重点：链条是否过长，是否有不必要的collect()
pub fn iterator_chain_demo() {
    println!("=== 快速识别：迭代器链式调用 ===");
    
    let numbers = vec![1, 2, 3, 4, 5, 6, 7, 8, 9, 10];
    
    // 典型的AI写法：过滤->映射->收集
    let even_squares: Vec<i32> = numbers
        .iter()
        .filter(|&&x| x % 2 == 0)  // 过滤偶数
        .map(|&x| x * x)           // 计算平方
        .collect();                // 收集到Vec
    
    println!("偶数的平方: {:?}", even_squares);
    
    // 更高效的写法：不产生中间集合，直接求和
    let sum: i32 = numbers
        .iter()
        .filter(|&&x| x % 2 == 0)
        .map(|&x| x * x)
        .sum();  // 直接消费，避免collect
    
    println!("偶数平方和: {}", sum);
}

// 🎯 实际场景：日志分析（AI常写的数据处理模式）
pub fn realistic_data_processing() {
    println!("=== 实际场景：数据处理管道 ===");
    
    // 模拟日志数据
    let log_entries = vec![
        ("user1", "login", 100),
        ("user2", "logout", 150),
        ("user1", "view_page", 200),
        ("user3", "login", 250),
        ("user1", "logout", 300),
    ];
    
    // 步骤1：按用户分组统计操作次数
    let mut user_actions = HashMap::new();
    for (user, _action, _time) in &log_entries {
        *user_actions.entry(user.to_string()).or_insert(0) += 1;
    }
    
    // 步骤2：找出最活跃的用户
    let most_active = user_actions
        .iter()
        .max_by_key(|(_, &count)| count)
        .map(|(user, count)| (user.as_str(), *count));
    
    println!("用户操作统计: {:?}", user_actions);
    if let Some((user, count)) = most_active {
        println!("最活跃用户: {} ({}次操作)", user, count);
    }
}

// 🎯 演示性能陷阱：AI常见的低效写法
pub fn performance_comparison() {
    println!("=== Review训练：性能对比 ===");
    
    let data = (1..1000).collect::<Vec<i32>>();
    
    // ✅ 高效写法：直接链式处理
    let result1: i32 = data
        .iter()
        .filter(|&&x| x % 2 == 0)
        .take(10)  // 只取前10个
        .sum();
    
    println!("高效处理结果: {}", result1);
    
    // AI常见低效模式说明（注释掉避免实际执行）
    // ❌ 低效：多次collect产生中间集合
    // let evens: Vec<_> = data.iter().filter(|&&x| x % 2 == 0).collect();
    // let first_ten: Vec<_> = evens.iter().take(10).collect();
    // let sum: i32 = first_ten.iter().sum();
    
    println!("✅ 避免中间集合，使用惰性求值");
}

// 🎯 主演示函数：展示所有迭代器模式
pub fn run_all_demos() {
    println!("🔄 迭代器与集合操作 - AI代码快速理解训练");
    println!("========================================");
    
    fold_sum_demo();
    println!();
    
    // 词频统计演示
    let freq = word_freq(&["rust", "is", "great", "rust", "is", "fast"]);
    println!("=== HashMap entry API演示 ===");
    println!("词频统计: {:?}", freq);
    println!();
    
    // 分组演示
    let items = vec![
        Item{ key: "database".into(), val: 100 },
        Item{ key: "cache".into(), val: 200 },
        Item{ key: "database".into(), val: 150 },
        Item{ key: "cache".into(), val: 250 },
    ];
    let grouped = group_by_key(&items);
    println!("=== fold + entry组合演示 ===");
    println!("按系统组件分组: {:?}", grouped);
    println!();
    
    iterator_chain_demo();
    println!();
    
    realistic_data_processing();
    println!();
    
    performance_comparison();
}

// 保持向后兼容的简单演示函数
pub fn iterators_demo() {
    println!("迭代器基础演示：");
    fold_sum_demo();
}