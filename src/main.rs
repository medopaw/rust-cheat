// ===============================================================
// Rust for AI Coding — 文件内容查看器 (Learn X in Y Minutes style)
// Focus: 快速读懂/Review 逻辑，而非抠细节
// ===============================================================

use std::env;
use std::process::Command;
use std::io::{self, Cursor};
use skim::prelude::*;

mod options;
mod async_demo;
mod errors;
mod iterators;
mod concurrency;
mod logging;
mod pattern_matching;
mod io_boundaries;

struct Module {
    name: &'static str,
    file: &'static str,
    description: &'static str,
}

const MODULES: &[Module] = &[
    Module { name: "01.    📖 Options", file: "options.rs", description: "Option / Result / Result<Option, E>" },
    Module { name: "02-03. ⚡ Async", file: "async_demo.rs", description: "async/await 与 block_on" },
    Module { name: "04.    ❌ Errors", file: "errors.rs", description: "anyhow（应用层错误聚合 + 上下文）" },
    Module { name: "05.    🔄 Iterators", file: "iterators.rs", description: "迭代器 fold/reduce 与 entry/or_insert 词频/分组" },
    Module { name: "06.    🚀 Concurrency", file: "concurrency.rs", description: "并发骨架：join!/try_join!/select!/spawn" },
    Module { name: "07.    📝 Logging", file: "logging.rs", description: "日志与可观测性：tracing 基本用法" },
    Module { name: "08.    🎯 Pattern Matching", file: "pattern_matching.rs", description: "模式匹配速查（match / if let）" },
    Module { name: "09.    💾 I/O Boundaries", file: "io_boundaries.rs", description: "I/O 边界（同步 vs 异步）" },
];

#[tokio::main]
async fn main() {
    println!("🦀 Rust Cheat Sheet - 交互式学习工具");
    println!("===============================================================");
    
    let mode_items = vec![
        "📚 查看代码文件 (推荐新手)".to_string(),
        "🚀 运行代码示例 (查看效果)".to_string(),
    ];
    
    // 首先选择模式
    match show_fuzzy_menu(&mode_items) {
        Ok(Some(0)) => file_browser_mode().await,
        Ok(Some(1)) => run_examples_mode().await,
        Ok(None) => println!("再见！👋"),
        Err(e) => println!("❌ 菜单错误: {}", e),
        Ok(Some(_)) => println!("❌ 无效选择"),
    }
}

// 原有的文件浏览模式
async fn file_browser_mode() {
    println!("\n📚 进入文件查看模式");
    println!("=================");
    
    let items: Vec<String> = MODULES.iter()
        .map(|module| format!("{} - {}", module.name, module.description))
        .collect();
    
    loop {
        match show_fuzzy_menu(&items) {
            Ok(Some(selection)) => {
                let module = &MODULES[selection];
                open_file(module);
                println!();
            }
            Ok(None) => {
                println!("返回主菜单 📚");
                break;
            }
            Err(e) => {
                println!("❌ 菜单错误: {}", e);
                break;
            }
        }
    }
}

// 新增的代码运行模式
async fn run_examples_mode() {
    println!("\n🚀 进入代码运行模式");
    println!("=================");
    
    let items: Vec<String> = MODULES.iter()
        .map(|module| format!("{} - 运行示例代码", module.name))
        .collect();
    
    loop {
        match show_fuzzy_menu(&items) {
            Ok(Some(selection)) => {
                let module = &MODULES[selection];
                run_module_examples(module).await;
                println!("\n按 Enter 继续...");
                let _ = io::stdin().read_line(&mut String::new());
            }
            Ok(None) => {
                println!("返回主菜单 🚀");
                break;
            }
            Err(e) => {
                println!("❌ 菜单错误: {}", e);
                break;
            }
        }
    }
}

// 根据模块运行对应的示例代码
async fn run_module_examples(module: &Module) {
    // 清屏提供更好的视觉体验
    clear_screen();
    
    println!("🏃 运行 {} 的示例代码", module.name);
    println!("===============================================");
    
    match module.file {
        "options.rs" => {
            options::run_all_demos();
        }
        "async_demo.rs" => {
            async_demo::run_all_demos().await;
        }
        "errors.rs" => {
            if let Err(e) = errors::run_all_demos() {
                println!("错误处理演示中的预期错误: {}", e);
            }
        }
        "iterators.rs" => {
            iterators::run_all_demos();
        }
        "concurrency.rs" => {
            println!("🚀 并发演示暂未实现 - 请查看源码");
        }
        "logging.rs" => {
            println!("📝 日志演示暂未实现 - 请查看源码");
        }
        "pattern_matching.rs" => {
            println!("🎯 模式匹配演示暂未实现 - 请查看源码");
        }
        "io_boundaries.rs" => {
            println!("💾 I/O边界演示暂未实现 - 请查看源码");
        }
        _ => {
            println!("❌ 未知模块: {}", module.file);
        }
    }
}

fn show_fuzzy_menu(items: &[String]) -> Result<Option<usize>, Box<dyn std::error::Error>> {
    println!("\n📚 请选择要查看的内容（支持模糊搜索，ESC 退出）:");
    
    // 准备选项数据
    let input = items.join("\n");
    let item_reader = SkimItemReader::default();
    let skim_items = item_reader.of_bufread(Cursor::new(input));
    
    // 配置 skim 选项
    let options = SkimOptionsBuilder::default()
        .height(String::from("50%"))
        .multi(false)
        .prompt(String::from("🔍 搜索: "))
        .header(Some("使用箭头键选择，Enter 确认，ESC 退出".to_string()))
        .layout(String::from("reverse"))
        .build()?;
    
    // 运行 skim
    let skim_output = Skim::run_with(&options, Some(skim_items));
    
    match skim_output {
        Some(output) => {
            if output.is_abort {
                // 用户按了 ESC 或 Ctrl+C
                Ok(None)
            } else {
                let selected_items = output.selected_items;
                if selected_items.is_empty() {
                    Ok(None)
                } else {
                    // 找到选中项的索引
                    let selected_text = selected_items[0].output().to_string();
                    let index = items.iter().position(|item| item == &selected_text);
                    Ok(index)
                }
            }
        }
        None => {
            // skim 返回 None，通常表示用户取消
            Ok(None)
        }
    }
}

fn is_in_ide() -> bool {
    // 检查环境变量来判断是否在 IDE 中
    env::var("VSCODE_PID").is_ok() ||
    env::var("CURSOR_SESSION_ID").is_ok() ||
    env::var("ZED").is_ok() ||
    env::var("TERM_PROGRAM").map(|t| t == "vscode").unwrap_or(false)
}

fn get_file_path(filename: &str) -> String {
    format!("src/{}", filename)
}

fn open_file(module: &Module) {
    let file_path = get_file_path(module.file);
    
    println!("📂 打开文件: {} - {}", module.name, module.description);
    
    if is_in_ide() {
        println!("🔍 检测到 IDE 环境，尝试在 IDE 中打开文件...");
        open_in_ide(&file_path);
    } else {
        println!("🔍 未检测到 IDE 环境，使用 vi 打开文件...");
        open_in_terminal(&file_path);
    }
}

fn open_in_ide(file_path: &str) {
    let mut success = false;
    
    // 尝试 VSCode
    if env::var("VSCODE_PID").is_ok() || env::var("TERM_PROGRAM").map(|t| t == "vscode").unwrap_or(false) {
        if let Ok(_) = Command::new("code").arg(file_path).status() {
            println!("✅ 已在 VSCode 中打开文件");
            success = true;
        }
    }
    
    // 尝试 Cursor
    if !success && env::var("CURSOR_SESSION_ID").is_ok() {
        if let Ok(_) = Command::new("cursor").arg(file_path).status() {
            println!("✅ 已在 Cursor 中打开文件");
            success = true;
        }
    }
    
    // 尝试 Zed
    if !success && env::var("ZED").is_ok() {
        if let Ok(_) = Command::new("zed").arg(file_path).status() {
            println!("✅ 已在 Zed 中打开文件");
            success = true;
        }
    }
    
    // 如果 IDE 命令失败，fallback 到终端
    if !success {
        println!("⚠️  IDE 命令执行失败，fallback 到终端查看...");
        open_in_terminal(file_path);
    }
}

fn open_in_terminal(file_path: &str) {
    println!("📖 使用 vi 打开文件（按 :q 退出 vi）");
    
    let status = Command::new("vi")
        .arg(file_path)
        .status();
        
    match status {
        Ok(_) => println!("✅ 文件查看完成"),
        Err(e) => println!("❌ 打开文件失败: {}", e),
    }
}

// 清屏函数
fn clear_screen() {
    // 跨平台清屏
    if cfg!(windows) {
        let _ = Command::new("cls").status();
    } else {
        let _ = Command::new("clear").status();
    }
}