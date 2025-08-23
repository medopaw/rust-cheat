// ===============================================================
// Rust for AI Coding — 文件内容查看器 (Learn X in Y Minutes style)
// Focus: 快速读懂/Review 逻辑，而非抠细节
// ===============================================================

use std::env;
use std::process::Command;
use std::io::{self, Cursor, BufReader, BufRead, Write, IsTerminal};
use skim::prelude::*;
use std::path::Path;
use std::fs;

#[derive(Debug, Clone, Copy, PartialEq)]
enum Ide {
    VSCode,
    Cursor,
    Zed,
    None,
}

impl Ide {
    fn detect() -> Self {
        if env::var("VSCODE_PID").is_ok() || env::var("TERM_PROGRAM").map(|t| t == "vscode").unwrap_or(false) {
            Self::VSCode
        } else if env::var("CURSOR_SESSION_ID").is_ok() {
            Self::Cursor
        } else if env::var("ZED").is_ok() {
            Self::Zed
        } else {
            Self::None
        }
    }
    
    fn name(&self) -> &'static str {
        match self {
            Self::VSCode => "VSCode",
            Self::Cursor => "Cursor", 
            Self::Zed => "Zed",
            Self::None => "未知IDE",
        }
    }
    
    fn command(&self) -> Option<&'static str> {
        match self {
            Self::VSCode => Some("code"),
            Self::Cursor => Some("cursor"),
            Self::Zed => Some("zed"),
            Self::None => None,
        }
    }
    
    fn is_available(&self) -> bool {
        !matches!(self, Self::None)
    }
    
    fn try_open_file(&self, file_path: &str) -> bool {
        if let Some(cmd) = self.command() {
            match Command::new(cmd).arg(file_path).status() {
                Ok(_) => {
                    println!("✅ 已在 {} 中打开文件", self.name());
                    true
                }
                Err(_) => false,
            }
        } else {
            false
        }
    }
}

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
    
    let items: Vec<String> = MODULES.iter()
        .map(|module| format!("{} - {}", module.name, module.description))
        .collect();
    
    loop {
        clear_screen();
        match show_fuzzy_menu(&items) {
            Ok(Some(selection)) => {
                let module = &MODULES[selection];
                integrated_module_experience(module).await;
                println!("\n按 Enter 继续...");
                let _ = io::stdin().read_line(&mut String::new());
            }
            Ok(None) => {
                println!("再见！👋");
                break;
            }
            Err(e) => {
                println!("❌ 菜单错误: {}", e);
                break;
            }
        }
    }
}

// 集成体验函数：mdcat + IDE打开 + 运行代码
async fn integrated_module_experience(module: &Module) {
    clear_screen();
    
    println!("🎯 学习模块: {} - {}", module.name, module.description);
    println!("========================================");
    
    let file_path = get_file_path(module.file);
    
    // 第一步：使用 mdcat 显示文件开头的注释部分
    println!("📖 模块概述和教学要点:");
    println!("------------------------");
    display_module_comments(&file_path);
    println!();
    
    // 第二步：用户确认后在 IDE 中打开文件并运行示例
    clear_screen();
    let ide = Ide::detect();
    if ide.is_available() {
        println!("\n💡 接下来将在 {} 中打开文件: {}", ide.name(), file_path);
    } else {
        println!("\n💡 接下来将在 IDE 中打开文件: {}", file_path);
    }
    println!("   然后运行示例代码演示模块功能");
    println!("\n按 Enter 继续...");
    let _ = io::stdin().read_line(&mut String::new());
    
    let ide_opened = open_in_ide_if_available(&file_path);
    
    // 第三步：运行示例代码
    println!("🚀 运行示例代码:");
    println!("------------------");
    run_module_examples(module).await;
    
    if !ide_opened {
        println!("\n💡 提示: 在 IDE 的终端中运行此程序效果更好，建议在 VSCode/Cursor/Zed 等 IDE 中打开项目！");
    }
}

// 显示文件开头的教学注释
fn display_module_comments(file_path: &str) {
    // 提取 markdown 注释内容
    match extract_markdown_comments(file_path) {
        Some(markdown_content) => {
            if markdown_content.is_empty() {
                display_comments_fallback(file_path);
                return;
            }
            
            // 检查 glow 是否已安装
            if is_glow_available() {
                // 直接使用 glow
                if !try_glow_viewer(&markdown_content) {
                    // glow 失败，回退到 less
                    try_less_viewer(&markdown_content);
                }
            } else {
                // glow 未安装，询问用户是否安装
                if ask_user_install_glow() {
                    if install_glow() {
                        println!("✅ glow 安装成功！");
                        if !try_glow_viewer(&markdown_content) {
                            // glow 安装后仍然失败，回退到 less
                            try_less_viewer(&markdown_content);
                        }
                    } else {
                        println!("❌ glow 安装失败，使用文本模式");
                        try_less_viewer(&markdown_content);
                    }
                } else {
                    // 用户拒绝安装，使用 less
                    try_less_viewer(&markdown_content);
                }
            }
        }
        None => {
            // 无法提取注释，回退到简单显示
            display_comments_fallback(file_path);
        }
    }
}

// 检查 glow 是否可用
fn is_glow_available() -> bool {
    Command::new("which")
        .arg("glow")
        .output()
        .map(|output| output.status.success())
        .unwrap_or(false)
}

// 询问用户是否安装 glow
fn ask_user_install_glow() -> bool {
    println!("🔍 检测到您未安装 glow markdown 渲染器");
    println!("💡 glow 可以提供更好的教学内容阅读体验（彩色渲染、表格、代码高亮）");
    println!();
    print!("是否现在安装 glow？(y/n): ");
    io::stdout().flush().unwrap();
    
    let mut input = String::new();
    if io::stdin().read_line(&mut input).is_ok() {
        let answer = input.trim().to_lowercase();
        answer == "y" || answer == "yes" || answer == "是"
    } else {
        false
    }
}

// 安装 glow
fn install_glow() -> bool {
    println!("🚀 正在安装 glow...");
    println!("   (这可能需要几分钟时间)");
    
    // 检测操作系统并选择合适的安装命令
    if cfg!(target_os = "macos") {
        // macOS - 使用 brew
        if Command::new("which").arg("brew").output().is_ok() {
            let result = Command::new("brew")
                .arg("install")
                .arg("glow")
                .status();
            
            match result {
                Ok(status) if status.success() => true,
                _ => {
                    println!("⚠️  brew 安装失败，您也可以手动安装:");
                    println!("   brew install glow");
                    false
                }
            }
        } else {
            println!("⚠️  未检测到 brew，请手动安装 glow:");
            println!("   1. 安装 Homebrew: https://brew.sh/");
            println!("   2. 运行: brew install glow");
            false
        }
    } else if cfg!(target_os = "linux") {
        // Linux - 提示多种安装方式
        println!("💡 请根据您的 Linux 发行版选择安装方式:");
        println!("   Ubuntu/Debian: sudo apt install glow");
        println!("   Arch Linux: sudo pacman -S glow");
        println!("   或下载二进制文件: https://github.com/charmbracelet/glow/releases");
        false
    } else {
        // Windows 或其他系统
        println!("💡 请访问 https://github.com/charmbracelet/glow/releases 下载安装");
        false
    }
}

// 使用 less 查看文本
fn try_less_viewer(markdown_content: &str) -> bool {
    println!("📖 模块概述和教学要点（按 q 退出，↑↓ 或 j/k 导航）:");
    
    // 检查是否在非交互环境中（如测试）
    if !std::io::stdin().is_terminal() || !std::io::stdout().is_terminal() {
        // 非交互环境，显示简化版本以避免输出过长
        println!("--- 模块概述 (非交互模式) ---");
        let lines: Vec<&str> = markdown_content.lines().collect();
        let preview_lines = std::cmp::min(10, lines.len()); // 减少到10行
        for line in lines.iter().take(preview_lines) {
            println!("{}", line);
        }
        if lines.len() > preview_lines {
            println!("... ({} 行总内容，在交互模式中可查看完整版本)", lines.len());
        }
        return true;
    }
    
    // 创建临时文本文件
    match create_temp_text_file(markdown_content) {
        Some(temp_file) => {
            let result = Command::new("less")
                .arg("-R")  // 支持颜色
                .arg("-S")  // 不换行长行
                .arg("-F")  // 如果内容不足一屏就直接显示
                .arg(&temp_file)
                .status();
            
            // 清理临时文件
            let _ = fs::remove_file(&temp_file);
            
            match result {
                Ok(status) => status.success(),
                Err(_) => {
                    // less 执行失败，直接打印内容
                    println!("--- less 不可用，显示文本内容 ---");
                    println!("{}", markdown_content);
                    true
                }
            }
        }
        None => {
            // 创建临时文件失败，直接打印
            println!("--- 无法创建临时文件，显示文本内容 ---");
            println!("{}", markdown_content);
            true
        }
    }
}

// 从文件中提取 markdown 格式的注释
fn extract_markdown_comments(file_path: &str) -> Option<String> {
    match fs::read_to_string(file_path) {
        Ok(content) => {
            let mut result = String::new();
            let mut in_comment_block = false;
            
            for line in content.lines() {
                let trimmed = line.trim();
                
                // 检测多行注释开始
                if trimmed.starts_with("/*") {
                    in_comment_block = true;
                    // 如果这行只有 /*，跳过不添加内容
                    if trimmed == "/*" {
                        // 不添加任何内容，直接继续
                    } else {
                        // 去掉 /* 开头
                        let content = trimmed.strip_prefix("/*").unwrap_or(trimmed);
                        result.push_str(content);
                        result.push('\n');
                    }
                    continue;
                }
                
                // 在多行注释中
                if in_comment_block {
                    if trimmed.ends_with("*/") {
                        // 注释结束
                        let content = trimmed.strip_suffix("*/").unwrap_or(trimmed);
                        if !content.trim().is_empty() {
                            result.push_str(content);
                            result.push('\n');
                        }
                        break;
                    } else {
                        // 继续在注释中，保留原始格式（但去掉行首的空格）
                        result.push_str(line);
                        result.push('\n');
                    }
                }
            }
            
            // 清理结果：去掉开头和结尾的空白
            let trimmed_result = result.trim();
            if trimmed_result.is_empty() {
                None
            } else {
                Some(trimmed_result.to_string())
            }
        }
        Err(_) => None,
    }
}

// 使用 glow 查看器
fn try_glow_viewer(markdown_content: &str) -> bool {
    println!("📖 模块概述和教学要点（按 q 退出查看器）:");
    
    // 检查是否在非交互环境中
    if !std::io::stdin().is_terminal() || !std::io::stdout().is_terminal() {
        // 非交互环境，显示简化版本以避免输出过长
        println!("--- 模块概述 (非交互模式) ---");
        let lines: Vec<&str> = markdown_content.lines().collect();
        let preview_lines = std::cmp::min(10, lines.len()); // 限制显示行数
        for line in lines.iter().take(preview_lines) {
            println!("{}", line);
        }
        if lines.len() > preview_lines {
            println!("... ({} 行总内容，在交互模式中可查看完整版本)", lines.len());
        }
        return true;
    }
    
    // 交互环境，使用带分页的 glow
    match create_temp_markdown_file(markdown_content) {
        Some(temp_file) => {
            let result = Command::new("glow")
                .arg("--pager")  // 启用分页
                .arg("--style")
                .arg("auto")     // 自动选择样式
                .arg(&temp_file)
                .status();
            
            // 清理临时文件
            let _ = fs::remove_file(temp_file);
            
            result.map(|status| status.success()).unwrap_or(false)
        }
        None => false,
    }
}

// 创建临时 markdown 文件
fn create_temp_markdown_file(content: &str) -> Option<String> {
    use std::time::{SystemTime, UNIX_EPOCH};
    
    let timestamp = SystemTime::now()
        .duration_since(UNIX_EPOCH)
        .unwrap()
        .as_secs();
    
    let temp_file = format!("/tmp/rust_cheat_{}.md", timestamp);
    
    match fs::write(&temp_file, content) {
        Ok(_) => Some(temp_file),
        Err(_) => None,
    }
}

// 创建临时文本文件
fn create_temp_text_file(content: &str) -> Option<String> {
    use std::time::{SystemTime, UNIX_EPOCH};
    
    let timestamp = SystemTime::now()
        .duration_since(UNIX_EPOCH)
        .unwrap()
        .as_secs();
    
    let temp_file = format!("/tmp/rust_cheat_{}.txt", timestamp);
    
    match fs::write(&temp_file, content) {
        Ok(_) => Some(temp_file),
        Err(_) => None,
    }
}

// 备用方案：简单显示文件开头的注释
fn display_comments_fallback(file_path: &str) {
    match fs::File::open(file_path) {
        Ok(file) => {
            let reader = io::BufReader::new(file);
            let mut in_comment_block = false;
            let mut comment_lines = 0;
            
            for line in reader.lines() {
                if let Ok(line) = line {
                    let trimmed = line.trim();
                    
                    // 检测注释块开始
                    if trimmed.starts_with("/*") {
                        in_comment_block = true;
                    }
                    
                    // 如果在注释块中或者是单行注释，显示内容
                    if in_comment_block || trimmed.starts_with("//") {
                        println!("{}", line);
                        comment_lines += 1;
                        
                        // 限制显示行数，避免过长，特别在非交互模式下
                        let max_lines = if std::io::stdin().is_terminal() { 50 } else { 10 };
                        if comment_lines > max_lines {
                            let more_lines = if std::io::stdin().is_terminal() { 
                                "更多内容请在 IDE 中查看"
                            } else {
                                "更多内容请在交互模式中查看"
                            };
                            println!("... ({})", more_lines);
                            break;
                        }
                    }
                    
                    // 检测注释块结束
                    if in_comment_block && trimmed.ends_with("*/") {
                        in_comment_block = false;
                        // 如果注释块结束后还有代码，就停止显示
                        break;
                    }
                    
                    // 如果遇到非注释的代码行，停止显示
                    if !in_comment_block && !trimmed.starts_with("//") && !trimmed.is_empty() && !trimmed.starts_with("use") && !trimmed.starts_with("mod") {
                        break;
                    }
                }
            }
        }
        Err(e) => {
            println!("❌ 无法读取文件 {}: {}", file_path, e);
        }
    }
}

// 尝试在 IDE 中打开文件，返回是否成功
fn open_in_ide_if_available(file_path: &str) -> bool {
    let ide = Ide::detect();
    if !ide.is_available() {
        return false;
    }
    
    println!("🔍 在 IDE 中打开文件: {}", file_path);
    
    if ide.try_open_file(file_path) {
        true
    } else {
        println!("⚠️  IDE 命令执行失败");
        false
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
    // 检查是否在 TTY 环境中
    if !std::io::stdin().is_terminal() || !std::io::stdout().is_terminal() {
        return show_simple_menu(items);
    }
    
    println!("\n🎯 选择学习模块（使用箭头键选择，ESC 退出）:");
    
    // 尝试运行 skim，如果失败就回退到简单菜单
    match run_skim_menu(items) {
        Ok(result) => Ok(result),
        Err(_) => {
            println!("⚠️  模糊搜索不可用，使用简单菜单:");
            show_simple_menu(items)
        }
    }
}

fn run_skim_menu(items: &[String]) -> Result<Option<usize>, Box<dyn std::error::Error>> {
    // 准备选项数据
    let input = items.join("\n");
    let item_reader = SkimItemReader::default();
    let skim_items = item_reader.of_bufread(Cursor::new(input));
    
    // 配置 skim 选项
    let options = SkimOptionsBuilder::default()
        .height(String::from("12"))
        .multi(false)
        .prompt(String::from(""))
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

fn show_simple_menu(items: &[String]) -> Result<Option<usize>, Box<dyn std::error::Error>> {
    loop {
        println!("\n🎯 选择学习模块（输入序号，0 退出）:");
        
        for (i, item) in items.iter().enumerate() {
            println!("  {}. {}", i + 1, item);
        }
        println!("  0. 退出");
        
        print!("\n请输入选择 (0-{}): ", items.len());
        io::stdout().flush().unwrap();
        
        let mut input = String::new();
        match io::stdin().read_line(&mut input) {
            Ok(_) => {
                match input.trim().parse::<usize>() {
                    Ok(0) => return Ok(None),
                    Ok(n) if n >= 1 && n <= items.len() => return Ok(Some(n - 1)),
                    _ => {
                        println!("❌ 无效输入，请输入 0-{} 之间的数字", items.len());
                    }
                }
            }
            Err(e) => {
                println!("❌ 读取输入时出错: {}", e);
                return Err(Box::new(e));
            }
        }
    }
}


fn get_file_path(filename: &str) -> String {
    format!("src/{}", filename)
}

fn open_file(module: &Module) {
    let file_path = get_file_path(module.file);
    
    println!("📂 打开文件: {} - {}", module.name, module.description);
    
    let ide = Ide::detect();
    if ide.is_available() {
        println!("🔍 检测到 {} 环境，尝试在 IDE 中打开文件...", ide.name());
        open_in_ide(&file_path);
    } else {
        println!("🔍 未检测到 IDE 环境，使用 vi 打开文件...");
        open_in_terminal(&file_path);
    }
}

fn open_in_ide(file_path: &str) {
    let ide = Ide::detect();
    
    if ide.try_open_file(file_path) {
        // 成功在 IDE 中打开文件
    } else {
        // 如果 IDE 命令失败，fallback 到终端
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
