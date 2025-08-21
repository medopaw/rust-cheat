/*
===============================================================
01. Option / Result / Result<Option, E>
===============================================================

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