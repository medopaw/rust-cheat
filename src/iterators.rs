/*
===============================================================
04. 迭代器与集合操作 - AI Coding 快速理解指南
===============================================================

🎯 业务场景：
- 数据聚合：统计、分组、计算（如日志分析、数据报表）
- 集合转换：过滤、映射、排序（如 API 响应处理）
- 缓存逻辑：entry API 高效更新 HashMap（避免重复查找）

🔍 30秒识别迭代器模式：
- 看链式调用：.iter().map().filter().collect() 风格
- 看聚合操作：fold/reduce（累积）、group_by（分组）
- 看 HashMap 更新：entry().or_insert()/or_default() 模式
- 看性能优化：避免不必要的 clone() 和多次 HashMap 查找

⚠️ AI 常见问题：
❌ 过度使用 collect() 产生中间集合，影响性能
❌ 在循环中重复查找 HashMap（用 entry API 更高效）
❌ 不必要的 clone()（特别是在 fold 中）
❌ 混用命令式循环和函数式链式调用，可读性差
❌ 忘记 iterator 是 lazy 的，需要消费才会执行

✅ Review 清单：
- [ ] 是否高效使用了 entry API 而非多次 get/insert？
- [ ] 是否避免了不必要的 clone() 和中间集合？
- [ ] 链式调用是否清晰易读（过长时考虑拆分）？
- [ ] 是否选择了合适的数据结构（HashMap vs BTreeMap）？
- [ ] 聚合操作是否处理了空集合情况？

📖 阅读顺序：
1. 先看数据流向（输入 -> 迭代器链 -> 输出类型）
2. 再看关键操作（map/filter/fold 的闭包逻辑）
3. 最后看性能点（clone、collect、HashMap 操作）

核心模式示例：

use std::collections::HashMap;

// fold 基本用法 - 累积计算
pub fn fold_sum_demo() {
    let sum = [1,2,3].iter().fold(0, |acc, x| acc + x);
    assert_eq!(sum, 6);
    println!("fold 求和结果: {}", sum);
}

// 词频统计：entry + or_insert - 高效更新模式
pub fn word_freq<'a>(words: &'a [&'a str]) -> HashMap<&'a str, usize> {
    let mut freq = HashMap::new();
    for w in words {
        let counter = freq.entry(*w).or_insert(0); // 若无此键，则插入 0；返回 &mut usize
        *counter += 1;
    }
    freq
}

// 按 key 分组（fold 版）- 数据聚合模式
#[derive(Clone, Debug)]
pub struct Item { 
    pub key: String, 
    pub val: i32 
}

pub fn group_by_key(items: &[Item]) -> HashMap<String, Vec<Item>> {
    items.iter().cloned().fold(HashMap::new(), |mut m, it| {
        m.entry(it.key.clone()).or_default().push(it); // or_default: 若无则插 Vec::new()
        m
    })
}
*/

use std::collections::HashMap;

pub fn fold_sum_demo() {
    let sum = [1,2,3].iter().fold(0, |acc, x| acc + x);
    assert_eq!(sum, 6);
    println!("fold 求和结果: {}", sum);
}

pub fn word_freq<'a>(words: &'a [&'a str]) -> HashMap<&'a str, usize> {
    let mut freq = HashMap::new();
    for w in words {
        let counter = freq.entry(*w).or_insert(0); // 若无此键，则插入 0；返回 &mut usize
        *counter += 1;
    }
    freq
}

#[derive(Clone, Debug)]
pub struct Item { 
    pub key: String, 
    pub val: i32 
}

pub fn group_by_key(items: &[Item]) -> HashMap<String, Vec<Item>> {
    items.iter().cloned().fold(HashMap::new(), |mut m, it| {
        m.entry(it.key.clone()).or_default().push(it); // or_default: 若无则插 Vec::new()
        m
    })
}

pub fn iterators_demo() {
    println!("迭代器演示：");
    
    // fold 演示
    fold_sum_demo();
    
    // 词频统计演示
    let freq = word_freq(&["a","b","a","c","b","a"]);
    println!("词频统计: {:?}", freq);
    
    // 分组演示
    let items = vec![
        Item{ key:"x".into(), val:1 },
        Item{ key:"y".into(), val:2 },
        Item{ key:"x".into(), val:3 },
    ];
    let grouped = group_by_key(&items);
    println!("按key分组: {:?}", grouped);
}