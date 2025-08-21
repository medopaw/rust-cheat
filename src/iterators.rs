/*
===============================================================
05. 迭代器 fold/reduce 与 entry/or_insert 词频/分组
===============================================================

use std::collections::HashMap;

// fold 基本用法
pub fn fold_sum_demo() {
    let sum = [1,2,3].iter().fold(0, |acc, x| acc + x);
    assert_eq!(sum, 6);
    println!("fold 求和结果: {}", sum);
}

// 词频统计：entry + or_insert
pub fn word_freq<'a>(words: &'a [&'a str]) -> HashMap<&'a str, usize> {
    let mut freq = HashMap::new();
    for w in words {
        let counter = freq.entry(*w).or_insert(0); // 若无此键，则插入 0；返回 &mut usize
        *counter += 1;
    }
    freq
}

// 按 key 分组（fold 版）
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