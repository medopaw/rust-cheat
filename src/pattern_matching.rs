/*
===============================================================
08. 模式匹配速查（match / if let）
===============================================================

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