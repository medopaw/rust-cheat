/*
# 🎯 Rust AI Coding 做题模块

## 核心理念
- **目标用户**：AI Coding 用户，需要具备 Rust Code Review 能力
- **核心能力**：程序理解、需求完整性检查、逻辑一致性审查、边界情况识别、业务合理性评估

## 做题流程
1. 用户完成模块学习后自动出现相关题目
2. 多选题形式，5-10个选项，2-8个正确答案
3. 做对显示祝贺，做错显示提示，循环直到做对
4. 随机化题目选择和选项顺序

## 技术实现
- 题库按模块组织，支持随机选择和选项随机化
- 键盘导航支持，视觉状态反馈
- 渐进式错误提示，优雅的用户体验
*/

use std::collections::{HashMap, HashSet};
use std::io::{self, Write};
use std::fs;
use std::fs::OpenOptions;
use rand::seq::SliceRandom;
use rand::thread_rng;
use serde::{Deserialize, Serialize};
use crossterm::{
    event::{self, KeyCode, KeyEvent, KeyModifiers, Event as CrosstermEvent},
    terminal::{self},
    cursor,
    style::{Color, SetForegroundColor, ResetColor, Print},
    ExecutableCommand, QueueableCommand,
};
use syntect::{
    parsing::SyntaxSet,
    highlighting::{ThemeSet, Style},
    util::as_24_bit_terminal_escaped,
};
use pulldown_cmark::{Parser, Event, Tag, TagEnd};

fn log_debug(message: &str) {
    // 只有在命令行传入 --debug 时才输出 debug 信息
    if crate::is_debug_enabled() {
        use std::io::Write;
        if let Ok(mut file) = OpenOptions::new()
            .create(true)
            .append(true)
            .open("debug.log")
        {
            let timestamp = std::time::SystemTime::now()
                .duration_since(std::time::UNIX_EPOCH)
                .unwrap_or_default()
                .as_secs();
            writeln!(file, "[{}] {}", timestamp, message).ok();
        }
    }
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Question {
    pub id: String,
    pub title: String,
    pub description: String,
    pub code: String,
    pub options: Vec<QuestionOption>,
    pub explanations: HashMap<String, String>, // option_id -> explanation
    pub key_points: Vec<String>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct QuestionOption {
    pub id: String,          // 唯一标识，不随顺序变化
    pub content: String,     // 选项内容
    pub is_correct: bool,    // 是否正确答案
}

#[derive(Debug, Deserialize)]
struct QuestionBank {
    module: String,
    questions: Vec<Question>,
}

#[derive(Debug)]
pub struct QuizSession {
    pub module: String,
    pub current_question: Option<Question>,
    pub question_state: QuestionState,
    pub attempt_count: usize,
    pub completed: bool,
}

#[derive(Debug)]
pub struct QuestionState {
    pub displayed_options: Vec<QuestionOption>, // 当前显示的选项顺序（随机化后）
    pub user_selections: HashSet<String>,       // 用户选择的选项ID
    pub current_focus: usize,                   // 当前焦点位置（键盘导航）
    pub phase: AnswerPhase,                     // 答题阶段
    pub revealed_status: HashMap<String, bool>, // 已显示的选项状态 (option_id -> is_correct)
    pub options_start_line: Option<u16>,        // 选项区域在屏幕上的起始行号
    pub can_use_local_redraw: bool,             // 是否可以使用局部重绘（基于终端空间检查）
    pub option_positions: Vec<u16>,             // 每个选项的行号(按不同模式记录)
    pub scroll_mode: bool,                      // 是否处于滚动模式(空间不足)
}

#[derive(Debug, PartialEq)]
pub enum AnswerPhase {
    FirstAttempt,      // 首次答题
    ShowingHints,      // 显示提示（选项正误状态）
    FinalAnswer,       // 查看最终答案和解释
}

pub struct QuizEngine {
    questions: HashMap<String, Vec<Question>>, // module -> questions
}

impl QuizEngine {
    pub fn new() -> Self {
        QuizEngine {
            questions: HashMap::new(),
        }
    }

    pub fn add_question_for_module(&mut self, module: &str, question: Question) {
        self.questions
            .entry(module.to_string())
            .or_insert_with(Vec::new)
            .push(question);
    }

    pub fn get_random_question(&self, module: &str) -> Option<Question> {
        let module_key = extract_module_key(module);
        log_debug(&format!("查找模块 '{}' -> 提取的key: '{}'", module, module_key));
        log_debug(&format!("当前存储的模块有: {:?}", self.questions.keys().collect::<Vec<_>>()));
        self.questions
            .get(&module_key)?
            .choose(&mut thread_rng())
            .cloned()
    }

    pub fn start_quiz_session(&self, module: &str) -> Option<QuizSession> {
        let mut question = self.get_random_question(module)?;
        question.options.shuffle(&mut thread_rng()); // 随机化选项顺序
        
        Some(QuizSession {
            module: module.to_string(),
            current_question: Some(question.clone()),
            question_state: QuestionState::new(&question),
            attempt_count: 0,
            completed: false,
        })
    }
}

impl QuestionState {
    pub fn new(question: &Question) -> Self {
        let mut displayed_options = question.options.clone();
        displayed_options.shuffle(&mut thread_rng());
        
        QuestionState {
            displayed_options,
            user_selections: HashSet::new(),
            current_focus: 0,
            phase: AnswerPhase::FirstAttempt,
            revealed_status: HashMap::new(),
            options_start_line: None,
            can_use_local_redraw: true, // 默认允许，会在显示时检查
            option_positions: Vec::new(), // 初始化为空，会在显示时填充
            scroll_mode: false,            // 默认非滚动模式
        }
    }

    pub fn toggle_selection(&mut self, option_index: usize) {
        log_debug(&format!("toggle_selection调用, option_index={}", option_index));
        if let Some(option) = self.displayed_options.get(option_index) {
            log_debug(&format!("找到选项 id={}, content={}", option.id, option.content));
            if self.user_selections.contains(&option.id) {
                log_debug("选项已被选择，现在取消选择");
                self.user_selections.remove(&option.id);
            } else {
                log_debug("选项未被选择，现在添加选择");
                self.user_selections.insert(option.id.clone());
            }
            log_debug(&format!("当前选择的选项: {:?}", self.user_selections));
        } else {
            log_debug(&format!("未找到索引为{}的选项", option_index));
        }
    }

    pub fn is_selected(&self, option_id: &str) -> bool {
        self.user_selections.contains(option_id)
    }

    pub fn move_focus(&mut self, direction: FocusDirection) {
        match direction {
            FocusDirection::Up => {
                if self.current_focus > 0 {
                    self.current_focus -= 1;
                } else {
                    self.current_focus = self.displayed_options.len().saturating_sub(1);
                }
            }
            FocusDirection::Down => {
                self.current_focus = (self.current_focus + 1) % self.displayed_options.len();
            }
        }
    }

    pub fn check_answer(&self, correct_options: &[String]) -> bool {
        let correct_set: HashSet<_> = correct_options.iter().collect();
        let user_set: HashSet<_> = self.user_selections.iter().collect();
        correct_set == user_set
    }

    pub fn reveal_all_status(&mut self, question: &Question) {
        self.revealed_status.clear();
        for option in &question.options {
            self.revealed_status.insert(option.id.clone(), option.is_correct);
        }
        self.phase = AnswerPhase::ShowingHints;
    }
}

#[derive(Debug)]
pub enum FocusDirection {
    Up,
    Down,
}

pub struct QuizUI;

impl QuizUI {
    
    fn render_markdown(text: &str) -> String {
        let parser = Parser::new(text);
        let mut output = String::new();
        let mut in_code_block = false;
        
        for event in parser {
            match event {
                Event::Start(Tag::CodeBlock(_)) => {
                    in_code_block = true;
                    output.push_str("  ");
                }
                Event::End(TagEnd::CodeBlock) => {
                    in_code_block = false;
                    output.push('\n');
                }
                Event::Start(Tag::Emphasis) => output.push_str("\x1b[3m"),
                Event::End(TagEnd::Emphasis) => output.push_str("\x1b[23m"),
                Event::Start(Tag::Strong) => output.push_str("\x1b[1m"),
                Event::End(TagEnd::Strong) => output.push_str("\x1b[22m"),
                Event::Start(Tag::List(_)) => {
                    // 列表开始前确保有换行
                    if !output.ends_with('\n') {
                        output.push('\n');
                    }
                },
                Event::Start(Tag::Item) => {
                    // 每个列表项前确保换行并添加bullet
                    output.push_str("\n• ");
                },
                Event::End(TagEnd::Item) => {
                    // 列表项结束后不需要额外操作，文本已经包含换行
                },
                Event::End(TagEnd::List(_)) => {
                    // 列表结束后添加额外空行
                    output.push('\n');
                },
                Event::Text(text) => {
                    if in_code_block {
                        output.push_str(&format!("\x1b[90m{}\x1b[0m", text));
                    } else {
                        output.push_str(&text);
                    }
                }
                Event::SoftBreak | Event::HardBreak => output.push('\n'),
                _ => {}
            }
        }
        
        output
    }

    fn highlight_code(code: &str, language: &str) -> String {
        let ps = SyntaxSet::load_defaults_newlines();
        let ts = ThemeSet::load_defaults();
        
        let syntax = ps.find_syntax_by_extension(language)
            .or_else(|| ps.find_syntax_by_name(language))
            .or_else(|| ps.find_syntax_by_name("Rust"))
            .unwrap_or_else(|| ps.find_syntax_plain_text());
        
        // 尝试使用更通用的主题
        let theme_name = if ts.themes.contains_key("Solarized (dark)") {
            "Solarized (dark)"
        } else if ts.themes.contains_key("InspiredGitHub") {
            "InspiredGitHub"
        } else {
            // 如果都没有，使用第一个可用的主题，或者默认主题
            ts.themes.keys().next().map(|s| s.as_str()).unwrap_or("base16-ocean.dark")
        };
        
        let theme = &ts.themes[theme_name];
        let mut highlighter = syntect::easy::HighlightLines::new(syntax, theme);
        let mut output = String::new();
        
        for line in code.lines() {
            match highlighter.highlight_line(line, &ps) {
                Ok(ranges) => {
                    let escaped = as_24_bit_terminal_escaped(&ranges[..], false);
                    output.push_str(&escaped);
                },
                Err(_) => {
                    // 如果高亮失败，至少保持原文本
                    output.push_str(line);
                }
            }
            output.push('\n');
        }
        
        output
    }

    pub fn display_question(session: &mut QuizSession) -> io::Result<()> {
        if let Some(question) = &session.current_question {
            Self::clear_screen_smooth();
            
            // 显示带颜色的标题
            io::stdout().execute(SetForegroundColor(Color::Cyan)).ok();
            println!("🎯 {} 模块 - Code Review 题目", session.module);
            io::stdout().execute(ResetColor).ok();
            println!("═══════════════════════════════════════════════════════════════");
            println!("\n📋 {}", question.title);
            println!("\n需求说明:");
            print!("{}", Self::render_markdown(&question.description));
            // 代码区域标题
            io::stdout().execute(SetForegroundColor(Color::Yellow)).ok();
            println!("\nAI 生成的代码:");
            io::stdout().execute(ResetColor).ok();
            
            // 代码区域，使用简洁的缩进显示，保持语法高亮
            println!();
            let highlighted_code = Self::highlight_code(&question.code, "rust");
            for line in highlighted_code.lines() {
                // 使用简单的缩进，保持语法高亮显示
                print!("  {}", line);
                println!();
            }
            println!();
            // 问题标题
            io::stdout().execute(SetForegroundColor(Color::Green)).ok();
            println!("\n❓ 问题：作为 Code Reviewer，你认为这段代码存在哪些问题？（多选）");
            io::stdout().execute(ResetColor).ok();
            println!();

            Self::display_options(&mut session.question_state)?;
            Self::display_instructions(&session.question_state);
            
            // 显示光标并确保所有输出都刷新
            Self::show_cursor();
        }
        Ok(())
    }

    pub fn display_options(state: &mut QuestionState) -> io::Result<()> {
        // 清空之前的位置记录
        state.option_positions.clear();
        state.can_use_local_redraw = true;
        
        use crossterm::{cursor, ExecutableCommand, terminal};
        let mut stdout = io::stdout();
        
        // 获取终端宽高信息
        let (terminal_width, terminal_height) = terminal::size().unwrap_or((80, 24));
        log_debug(&format!("终端尺寸 - 宽度: {}, 高度: {}", terminal_width, terminal_height));
        
        let options_count = state.displayed_options.len();
        log_debug(&format!("选项数量: {}", options_count));
        
        // 检测空间是否足够显示所有内容
        let (_, current_row) = cursor::position().unwrap_or((0, 0));
        let content_height = options_count + 6; // 选项 + 操作说明 + 空行 + 输入行
        let available_height = terminal_height.saturating_sub(current_row);
        
        log_debug(&format!("空间检测: 当前行={}, 终端高度={}, 可用高度={}, 需要高度={}", 
            current_row, terminal_height, available_height, content_height));
        
        if available_height >= content_height as u16 {
            // 空间足够：使用正向坐标记录实际位置
            state.scroll_mode = false;
            log_debug("使用正向坐标模式（空间足够）");
        } else {
            // 空间不足：使用倒数坐标模式
            state.scroll_mode = true;
            let bottom_reserved_lines = 6;
            
            for i in 0..options_count {
                let reverse_line = bottom_reserved_lines + (options_count - i);
                state.option_positions.push(reverse_line as u16);
                log_debug(&format!("选项{} 倒数坐标: {}", i, reverse_line));
            }
            
            log_debug("使用倒数坐标模式（空间不足）");
        }
        
        state.can_use_local_redraw = true;
        log_debug("使用倒数坐标计算的局部重绘方案");
        
        // 正常显示所有选项，按模式记录坐标
        for (i, option) in state.displayed_options.iter().enumerate() {
            // 在正向坐标模式下记录实际位置
            if !state.scroll_mode {
                let (_, actual_row) = cursor::position().unwrap_or((0, 0));
                state.option_positions.push(actual_row);
                log_debug(&format!("选项{} 正向坐标: {}", i, actual_row));
            }
            let letter = char::from(b'A' + i as u8);
            let is_focused = i == state.current_focus;
            let is_selected = state.is_selected(&option.id);
            
            let status_indicator = match state.phase {
                AnswerPhase::FirstAttempt => {
                    if is_selected { "✓" } else { " " }
                }
                AnswerPhase::ShowingHints => {
                    match state.revealed_status.get(&option.id) {
                        Some(true) => "✅",
                        Some(false) => "❌",
                        None => " ",
                    }
                }
                AnswerPhase::FinalAnswer => {
                    if option.is_correct { "✅" } else { "❌" }
                }
            };

            // 焦点标记
            if is_focused {
                io::stdout().execute(SetForegroundColor(Color::Yellow)).ok();
                print!("► ");
                io::stdout().execute(ResetColor).ok();
            } else {
                print!("  ");
            }

            // 选项标号
            io::stdout().execute(SetForegroundColor(Color::Blue)).ok();
            print!("{}. ", letter);
            io::stdout().execute(ResetColor).ok();

            // 选择标记
            if is_selected {
                io::stdout().execute(SetForegroundColor(Color::Green)).ok();
                print!("[✓] ");
                io::stdout().execute(ResetColor).ok();
            } else {
                print!("[ ] ");
            }

            // 选项内容
            if is_focused {
                io::stdout().execute(SetForegroundColor(Color::White)).ok();
            }
            print!("{}", option.content);
            io::stdout().execute(ResetColor).ok();

            // 状态指示器
            if matches!(state.phase, AnswerPhase::ShowingHints | AnswerPhase::FinalAnswer) {
                print!(" {}", status_indicator);
            }
            println!();
            
            // 在正向坐标模式下，记录println后的位置(即下一行的位置-1)
            if !state.scroll_mode {
                let (_, after_row) = cursor::position().unwrap_or((0, 1));
                let option_row = after_row.saturating_sub(1);
                // 更新最后一个位置为实际选项所在行
                if let Some(last_pos) = state.option_positions.last_mut() {
                    *last_pos = option_row;
                    log_debug(&format!("更新选项{} 正向坐标: {}", i, option_row));
                }
            }
        }
        
        let mode_desc = if state.scroll_mode { "倒数坐标" } else { "正向坐标" };
        log_debug(&format!("所有选项位置记录({}): {:?}", mode_desc, state.option_positions));
        log_debug(&format!("当前焦点选项: {}, 滚动模式: {}", state.current_focus, state.scroll_mode));
        
        Ok(())
    }

    pub fn display_instructions(state: &QuestionState) {
        println!();
        match state.phase {
            AnswerPhase::FirstAttempt => {
                // 操作说明标题
                io::stdout().execute(SetForegroundColor(Color::Magenta)).unwrap();
                println!("💡 操作说明：");
                io::stdout().execute(ResetColor).unwrap();
                
                // 操作说明内容
                io::stdout().execute(SetForegroundColor(Color::DarkGrey)).unwrap();
                println!("   j/k/↑↓ = 上下移动焦点 | space = 选择/取消当前焦点选项");
                let max_letter = char::from(b'A' + (state.displayed_options.len() - 1) as u8);
                println!("   A-{} = 直接选择对应字母选项 | PageUp/PageDown = 翻页", max_letter);
                println!("   Enter = 提交答案 | q = 退出");
                io::stdout().execute(ResetColor).unwrap();
            }
            AnswerPhase::ShowingHints => {
                io::stdout().execute(SetForegroundColor(Color::Magenta)).unwrap();
                println!("💡 根据上面的状态指示，重新选择正确的选项组合");
                io::stdout().execute(ResetColor).unwrap();
                
                io::stdout().execute(SetForegroundColor(Color::DarkGrey)).unwrap();
                println!("   ✅ = 正确选项, ❌ = 错误选项");
                println!("   j/k/↑↓ = 移动焦点 | space = 选择/取消 | A-Z = 直接选择");
                println!("   Enter = 提交 | PageUp/PageDown = 翻页");
                io::stdout().execute(ResetColor).unwrap();
            }
            AnswerPhase::FinalAnswer => {
                io::stdout().execute(SetForegroundColor(Color::Green)).unwrap();
                println!("🎉 查看详细解释，按任意键继续下一题...");
                io::stdout().execute(ResetColor).unwrap();
            }
        }
    }

    pub fn display_feedback(session: &QuizSession, is_correct: bool) {
        Self::clear_screen();
        
        if is_correct {
            println!("🎉 恭喜！你成功识别了代码中的问题。");
            println!("作为一名优秀的 Code Reviewer，你具备了审查 AI 生成 Rust 代码的核心能力！");
            println!();
            
            if let Some(question) = &session.current_question {
                println!("✅ 你正确识别了：");
                for option in &question.options {
                    if option.is_correct {
                        println!("   • {}", option.content);
                    }
                }
            }
            
            println!("\n继续保持这种审查思维，在实际 AI Coding 中你会更加高效！");
            println!("\n按 Enter 继续...");
        } else {
            println!("📚 再想想看！");
            println!();
            println!("选项状态提示已显示，请根据提示重新选择...");
        }
    }

    pub fn display_final_explanation(session: &QuizSession) {
        if let Some(question) = &session.current_question {
            Self::clear_screen();
            println!("📖 详细解释");
            println!("===============================================");
            
            println!("\n✅ 正确选项：");
            for option in &question.options {
                if option.is_correct {
                    let letter = Self::get_option_letter(&session.question_state, &option.id);
                    println!("{}. {} - {}", 
                        letter, 
                        option.content,
                        question.explanations.get(&option.id).unwrap_or(&"无详细解释".to_string())
                    );
                }
            }
            
            println!("\n❌ 错误选项：");
            for option in &question.options {
                if !option.is_correct {
                    let letter = Self::get_option_letter(&session.question_state, &option.id);
                    println!("{}. {} - {}", 
                        letter, 
                        option.content,
                        question.explanations.get(&option.id).unwrap_or(&"这个选项不是问题".to_string())
                    );
                }
            }
            
            if !question.key_points.is_empty() {
                println!("\n💡 关键学习点：");
                for point in &question.key_points {
                    println!("   • {}", point);
                }
            }
            
            println!("\n按任意键继续...");
        }
    }

    fn get_option_letter(state: &QuestionState, option_id: &str) -> char {
        for (i, option) in state.displayed_options.iter().enumerate() {
            if option.id == option_id {
                return char::from(b'A' + i as u8);
            }
        }
        '?'
    }

    pub fn clear_screen() {
        // 使用更高效的清屏方式，减少闪烁
        use crossterm::{terminal, cursor, QueueableCommand};
        
        let mut stdout = io::stdout();
        stdout.queue(cursor::MoveTo(0, 0)).ok();
        stdout.queue(terminal::Clear(terminal::ClearType::All)).ok();
        stdout.flush().ok();
    }
    
    pub fn clear_screen_smooth() {
        // 使用最优化的平滑清屏策略
        use crossterm::{terminal, cursor, QueueableCommand};
        
        let mut stdout = io::stdout();
        // 1. 隐藏光标，减少视觉干扰
        stdout.queue(cursor::Hide).ok();
        // 2. 移动到左上角
        stdout.queue(cursor::MoveTo(0, 0)).ok();
        // 3. 清除屏幕内容
        stdout.queue(terminal::Clear(terminal::ClearType::All)).ok();
        // 4. 立即刷新，确保清屏立即生效
        stdout.flush().ok();
    }
    
    pub fn show_cursor() {
        use crossterm::{cursor, QueueableCommand};
        let mut stdout = io::stdout();
        stdout.queue(cursor::Show).ok();
        stdout.flush().ok();
    }
    
    pub fn cleanup_terminal() {
        // 清理终端状态，确保退出时显示正常
        use crossterm::{terminal, cursor, style, QueueableCommand};
        
        let mut stdout = io::stdout();
        
        // 1. 重置颜色和样式
        stdout.queue(style::ResetColor).ok();
        
        // 2. 显示光标
        stdout.queue(cursor::Show).ok();
        
        // 3. 清理屏幕，移动到屏幕底部
        stdout.queue(terminal::Clear(terminal::ClearType::All)).ok();
        stdout.queue(cursor::MoveTo(0, 0)).ok();
        
        // 4. 确保所有命令立即执行
        stdout.flush().ok();
        
        log_debug("终端状态已清理，退出 quiz");
    }

    pub fn clear_from_cursor() {
        // 只清除从光标位置到屏幕末尾的内容
        print!("\x1B[0J");
        io::stdout().flush().unwrap();
    }


    pub fn update_single_option(session: &mut QuizSession, option_index: usize) -> io::Result<bool> {
        // 根据滚动模式选择不同的坐标计算方式
        log_debug(&format!("update_single_option调用, option_index={}", option_index));
        if let Some(_question) = &session.current_question {
            if let Some(&stored_line) = session.question_state.option_positions.get(option_index) {
                use crossterm::{cursor, QueueableCommand, terminal};
                let mut stdout = io::stdout();
                
                let actual_line = if session.question_state.scroll_mode {
                    // 滚动模式：从倒数行号计算实际位置
                    let (_, terminal_height) = terminal::size().unwrap_or((80, 24));
                    let actual_line = terminal_height.saturating_sub(stored_line);
                    log_debug(&format!("选项{}: 滚动模式, 倒数行号={}, 终端高度={}, 实际行号={}", 
                        option_index, stored_line, terminal_height, actual_line));
                    actual_line
                } else {
                    // 正向模式：直接使用存储的实际行号
                    log_debug(&format!("选项{}: 正向模式, 实际行号={}", 
                        option_index, stored_line));
                    stored_line
                };
                
                // 移动到计算出的实际行位置
                stdout.queue(cursor::MoveTo(0, actual_line)).ok();
                
                // 清除当前行
                stdout.queue(terminal::Clear(terminal::ClearType::CurrentLine)).ok();
                
                // 重绘这个选项
                if let Some(option) = session.question_state.displayed_options.get(option_index) {
                    let letter = char::from(b'A' + option_index as u8);
                    let is_focused = option_index == session.question_state.current_focus;
                    let is_selected = session.question_state.is_selected(&option.id);
                    
                    let status_indicator = match session.question_state.phase {
                        AnswerPhase::FirstAttempt => {
                            if is_selected { "✓" } else { " " }
                        }
                        AnswerPhase::ShowingHints => {
                            match session.question_state.revealed_status.get(&option.id) {
                                Some(true) => "✅",
                                Some(false) => "❌",
                                None => " ",
                            }
                        }
                        AnswerPhase::FinalAnswer => {
                            if option.is_correct { "✅" } else { "❌" }
                        }
                    };

                    // 焦点标记
                    if is_focused {
                        stdout.queue(SetForegroundColor(Color::Yellow)).ok();
                        stdout.queue(Print("► ")).ok();
                        stdout.queue(ResetColor).ok();
                    } else {
                        stdout.queue(Print("  ")).ok();
                    }

                    // 选项标号
                    stdout.queue(SetForegroundColor(Color::Blue)).ok();
                    stdout.queue(Print(format!("{}. ", letter))).ok();
                    stdout.queue(ResetColor).ok();

                    // 选择标记
                    if is_selected {
                        stdout.queue(SetForegroundColor(Color::Green)).ok();
                        stdout.queue(Print("[✓] ")).ok();
                        stdout.queue(ResetColor).ok();
                    } else {
                        stdout.queue(Print("[ ] ")).ok();
                    }

                    // 选项内容
                    if is_focused {
                        stdout.queue(SetForegroundColor(Color::White)).ok();
                    }
                    stdout.queue(Print(&option.content)).ok();
                    stdout.queue(ResetColor).ok();

                    // 状态指示器
                    if matches!(session.question_state.phase, AnswerPhase::ShowingHints | AnswerPhase::FinalAnswer) {
                        stdout.queue(Print(format!(" {}", status_indicator))).ok();
                    }
                }
                
                stdout.flush()?;
                log_debug("局部重绘完成");
                return Ok(true); // 成功进行局部重绘
            } else {
                log_debug(&format!("未找到选项{}的位置记录", option_index));
            }
        } else {
            log_debug("当前没有题目");
        }
        log_debug("无法进行局部重绘，需要全屏重绘");
        Ok(false) // 无法进行局部重绘，需要全屏重绘
    }

    pub fn get_user_input() -> io::Result<UserInput> {
        // 不显示输入提示符，减少视觉干扰
        // print!("\n> ");
        // io::stdout().flush()?;
        
        // Enable raw mode for immediate key response
        terminal::enable_raw_mode()?;
        
        let result = loop {
            match event::read()? {
                CrosstermEvent::Key(KeyEvent { code, modifiers, .. }) => {
                    // Handle Ctrl+C gracefully
                    if matches!(code, KeyCode::Char('c')) && modifiers.contains(KeyModifiers::CONTROL) {
                        break Ok(UserInput::Quit);
                    }
                    
                    let user_input = match code {
                        KeyCode::Char('q') | KeyCode::Char('Q') => UserInput::Quit,
                        KeyCode::Char(' ') => UserInput::ToggleSelection,
                        KeyCode::Enter => UserInput::Submit,
                        KeyCode::Char('j') | KeyCode::Char('J') => UserInput::MoveFocus(FocusDirection::Down),
                        KeyCode::Char('k') | KeyCode::Char('K') => UserInput::MoveFocus(FocusDirection::Up),
                        KeyCode::Up => UserInput::MoveFocus(FocusDirection::Up),
                        KeyCode::Down => UserInput::MoveFocus(FocusDirection::Down),
                        KeyCode::PageUp => UserInput::MoveFocus(FocusDirection::Up),
                        KeyCode::PageDown => UserInput::MoveFocus(FocusDirection::Down),
                        KeyCode::Char(c) if c.is_ascii_alphabetic() => {
                            let letter = c.to_ascii_uppercase();
                            if letter >= 'A' && letter <= 'Z' {
                                let index = (letter as u8 - b'A') as usize;
                                log_debug(&format!("用户按下字母 '{}', 转换为索引 {}", letter, index));
                                UserInput::SelectByIndex(index + 1) // +1 因为原来的逻辑期望1-based索引
                            } else {
                                UserInput::Unknown
                            }
                        },
                        _ => UserInput::Unknown,
                    };
                    
                    // Only return for meaningful inputs, ignore unknown keys
                    if !matches!(user_input, UserInput::Unknown) {
                        break Ok(user_input);
                    }
                },
                CrosstermEvent::Resize(_, _) => {
                    // 终端大小改变时返回特殊输入，触发重绘
                    break Ok(UserInput::Resize);
                },
                _ => continue, // Ignore other events like mouse, etc.
            }
        };
        
        // Disable raw mode before returning
        terminal::disable_raw_mode()?;
        result
    }
}

#[derive(Debug)]
pub enum UserInput {
    MoveFocus(FocusDirection),
    ToggleSelection,
    Submit,
    Quit,
    SelectByIndex(usize),
    Resize,  // 终端大小改变
    Unknown,
}

impl Default for QuizEngine {
    fn default() -> Self {
        Self::new()
    }
}

impl QuizSession {
    pub fn run_quiz_loop(&mut self) -> io::Result<bool> {
        // 直接使用标准终端模式，支持滚动和Page Up/Down
        log_debug("使用标准终端模式，支持用户滚动操作");
        
        self.run_quiz_loop_inner()
    }
    
    fn run_quiz_loop_inner(&mut self) -> io::Result<bool> {
        while !self.completed {
            if let Some(question) = &self.current_question {
                QuizUI::display_question(self)?;
                
                match self.handle_user_interaction()? {
                    QuizResult::Continue => continue,
                    QuizResult::Quit => return Ok(false),
                    QuizResult::Completed => {
                        self.completed = true;
                        return Ok(true);
                    }
                }
            } else {
                break;
            }
        }
        Ok(true)
    }

    fn handle_user_interaction(&mut self) -> io::Result<QuizResult> {
        loop {
            match QuizUI::get_user_input()? {
                UserInput::MoveFocus(direction) => {
                    let old_focus = self.question_state.current_focus;
                    self.question_state.move_focus(direction);
                    let new_focus = self.question_state.current_focus;
                    
                    // 尝试局部重绘，只有在有精确位置记录时才使用
                    if self.question_state.can_use_local_redraw && 
                       !self.question_state.option_positions.is_empty() &&
                       old_focus < self.question_state.option_positions.len() && 
                       new_focus < self.question_state.option_positions.len() {
                        let old_updated = QuizUI::update_single_option(self, old_focus)?;
                        let new_updated = QuizUI::update_single_option(self, new_focus)?;
                        
                        if !old_updated || !new_updated {
                            // 位置超出范围或其他问题，回退到全屏重绘
                            QuizUI::display_question(self)?;
                        }
                    } else {
                        // 没有精确位置记录，使用全屏重绘
                        QuizUI::display_question(self)?;
                    }
                }
                UserInput::ToggleSelection => {
                    let focus_index = self.question_state.current_focus;
                    self.question_state.toggle_selection(focus_index);
                    
                    // 尝试局部重绘，只有在有精确位置记录时才使用
                    if self.question_state.can_use_local_redraw && 
                       !self.question_state.option_positions.is_empty() &&
                       focus_index < self.question_state.option_positions.len() {
                        let updated = QuizUI::update_single_option(self, focus_index)?;
                        if !updated {
                            // 位置超出范围，回退到全屏重绘
                            QuizUI::display_question(self)?;
                        }
                    } else {
                        // 没有精确位置记录，使用全屏重绘
                        QuizUI::display_question(self)?;
                    }
                }
                UserInput::SelectByIndex(index) => {
                    log_debug(&format!("处理SelectByIndex, index={}, 选项总数={}", index, self.question_state.displayed_options.len()));
                    if index > 0 && index <= self.question_state.displayed_options.len() {
                        let option_index = index - 1;
                        log_debug(&format!("计算option_index={}, 当前焦点={}", option_index, self.question_state.current_focus));
                        
                        // 更新焦点位置到选中的选项
                        self.question_state.current_focus = option_index;
                        log_debug(&format!("更新焦点到{}", option_index));
                        
                        self.question_state.toggle_selection(option_index);
                        log_debug("切换选择状态完成");
                        
                        // 尝试局部重绘，只有在有精确位置记录时才使用
                        if self.question_state.can_use_local_redraw && 
                           !self.question_state.option_positions.is_empty() &&
                           option_index < self.question_state.option_positions.len() {
                            let updated = QuizUI::update_single_option(self, option_index)?;
                            if !updated {
                                // 位置超出范围，回退到全屏重绘
                                log_debug("局部重绘失败，使用全屏重绘");
                                QuizUI::display_question(self)?;
                            } else {
                                log_debug("局部重绘成功");
                            }
                        } else {
                            // 没有精确位置记录，使用全屏重绘
                            log_debug("无法使用局部重绘，使用全屏重绘");
                            QuizUI::display_question(self)?;
                        }
                    } else {
                        log_debug(&format!("index {} 超出范围", index));
                    }
                }
                UserInput::Submit => {
                    return self.handle_answer_submission();
                }
                UserInput::Quit => {
                    // 清理终端状态，避免显示错乱
                    QuizUI::cleanup_terminal();
                    return Ok(QuizResult::Quit);
                }
                UserInput::Resize => {
                    // 终端大小改变，清空所有位置记录并重新显示
                    self.question_state.options_start_line = None;
                    self.question_state.option_positions.clear();
                    self.question_state.can_use_local_redraw = true;
                    QuizUI::display_question(self)?;
                }
                UserInput::Unknown => {
                    // 忽略未知输入
                }
            }
        }
    }

    fn handle_answer_submission(&mut self) -> io::Result<QuizResult> {
        if let Some(question) = &self.current_question {
            let correct_options: Vec<String> = question
                .options
                .iter()
                .filter(|opt| opt.is_correct)
                .map(|opt| opt.id.clone())
                .collect();

            let is_correct = self.question_state.check_answer(&correct_options);
            self.attempt_count += 1;

            match self.question_state.phase {
                AnswerPhase::FirstAttempt => {
                    if is_correct {
                        QuizUI::display_feedback(self, true);
                        self.wait_for_continue()?;
                        return Ok(QuizResult::Completed);
                    } else {
                        self.question_state.reveal_all_status(question);
                        QuizUI::display_feedback(self, false);
                        self.wait_for_continue()?;
                        return Ok(QuizResult::Continue);
                    }
                }
                AnswerPhase::ShowingHints => {
                    if is_correct {
                        self.question_state.phase = AnswerPhase::FinalAnswer;
                        QuizUI::display_final_explanation(self);
                        self.wait_for_continue()?;
                        return Ok(QuizResult::Completed);
                    } else {
                        // 继续显示提示，让用户再次尝试
                        QuizUI::display_question(self)?;
                        return Ok(QuizResult::Continue);
                    }
                }
                AnswerPhase::FinalAnswer => {
                    return Ok(QuizResult::Completed);
                }
            }
        }
        Ok(QuizResult::Continue)
    }

    fn wait_for_continue(&self) -> io::Result<()> {
        use std::io::Read;
        let mut buffer = [0; 1];
        io::stdin().read_exact(&mut buffer)?;
        Ok(())
    }
}

#[derive(Debug)]
pub enum QuizResult {
    Continue,
    Completed,
    Quit,
}

pub async fn run_module_quiz(module: &str) -> io::Result<bool> {
    let mut engine = QuizEngine::default();
    
    // 加载题库
    load_questions_for_module(&mut engine, module);
    
    if let Some(mut session) = engine.start_quiz_session(module) {
        session.run_quiz_loop()
    } else {
        println!("❌ 该模块暂无练习题目");
        Ok(false)
    }
}

fn load_questions_for_module(engine: &mut QuizEngine, module: &str) {
    let module_key = extract_module_key(module);
    log_debug(&format!("加载模块 '{}' -> 提取的key: '{}'", module, module_key));
    
    let filename = match module_key.as_str() {
        "Options" => "options.json",
        "Async" => "async.json", 
        "Errors" => "errors.json",
        "Iterators" => "iterators.json",
        "Concurrency" => "concurrency.json",
        "Logging" => "logging.json",
        "Pattern" => "pattern_matching.json",
        "I/O" => "io_boundaries.json",
        _ => {
            log_debug(&format!("未匹配到模块key '{}'", module_key));
            return;
        }
    };
    
    let file_path = format!("questions/{}", filename);
    log_debug(&format!("尝试加载文件: {}", file_path));
    
    match load_questions_from_file(&file_path) {
        Ok(questions) => {
            log_debug(&format!("成功加载 {} 道题目", questions.len()));
            for question in questions {
                engine.add_question_for_module(&module_key, question);
            }
        }
        Err(e) => {
            log_debug(&format!("加载失败: {}, 回退到硬编码", e));
            // 回退到硬编码的题目（仅用于兼容）
            match module_key.as_str() {
                "Options" => load_options_questions_fallback(engine),
                _ => {}
            }
        }
    }
}

fn load_questions_from_file(file_path: &str) -> Result<Vec<Question>, Box<dyn std::error::Error>> {
    let content = fs::read_to_string(file_path)?;
    let question_bank: QuestionBank = serde_json::from_str(&content)?;
    
    // 为每个题目添加模块信息
    let mut questions = question_bank.questions;
    for question in &mut questions {
        // 注意：现在 Question 结构体中没有 module 字段了，我们在 QuizSession 中管理模块信息
    }
    
    Ok(questions)
}

fn extract_module_key(module_name: &str) -> String {
    // 从类似 "01.    📖 Options" 的格式中提取 "Options"
    // 先去除序号和符号，然后取最后一个词
    let cleaned = module_name
        .split_whitespace()
        .filter(|part| !part.starts_with(char::is_numeric) && !part.contains("📖") && !part.contains("⚡") 
                && !part.contains("❌") && !part.contains("🔄") && !part.contains("🚀") 
                && !part.contains("📝") && !part.contains("🎯") && !part.contains("💾"))
        .last()
        .unwrap_or(module_name);
    
    cleaned.to_string()
}

// 备用题目加载函数（当JSON文件不可用时）
fn load_options_questions_fallback(engine: &mut QuizEngine) {
    let sample_question = Question {
        id: "options_001".to_string(),
        title: "用户输入验证函数".to_string(),
        description: r#"需要实现一个用户输入验证函数，要求：
- 接受用户输入的字符串
- 验证输入不为空且长度在3-20字符之间
- 返回验证结果，如果无效则说明原因
- 支持邮箱格式的基本验证"#.to_string(),
        code: r#"fn validate_user_input(input: &str) -> Result<String, String> {
    if input.is_empty() {
        return Err("输入不能为空".to_string());
    }
    
    if input.len() < 3 {
        return Err("输入太短".to_string());
    }
    
    if input.contains('@') {
        if !input.contains('.') {
            return Err("邮箱格式无效".to_string());
        }
    }
    
    Ok(input.to_string())
}"#.to_string(),
        options: vec![
            QuestionOption {
                id: "opt_a".to_string(),
                content: "缺少对最大长度的检查".to_string(),
                is_correct: true,
            },
            QuestionOption {
                id: "opt_b".to_string(),
                content: "邮箱验证逻辑过于简单，无法处理复杂情况".to_string(),
                is_correct: true,
            },
            QuestionOption {
                id: "opt_c".to_string(),
                content: "函数名称不够清晰".to_string(),
                is_correct: false,
            },
            QuestionOption {
                id: "opt_d".to_string(),
                content: "应该使用 Option 而不是 Result".to_string(),
                is_correct: false,
            },
            QuestionOption {
                id: "opt_e".to_string(),
                content: "缺少对特殊字符的处理".to_string(),
                is_correct: true,
            },
            QuestionOption {
                id: "opt_f".to_string(),
                content: "此代码实现完全正确".to_string(),
                is_correct: false,
            },
        ],
        explanations: {
            let mut map = HashMap::new();
            map.insert("opt_a".to_string(), "需求明确要求长度在3-20字符之间，但代码只检查了最小长度，遗漏了最大长度检查".to_string());
            map.insert("opt_b".to_string(), "简单的包含'@'和'.'检查无法验证真实的邮箱格式，如'@.'也会通过".to_string());
            map.insert("opt_c".to_string(), "函数名validate_user_input已经很清晰地表达了其功能".to_string());
            map.insert("opt_d".to_string(), "Result类型很适合这里，因为需要返回具体的错误信息".to_string());
            map.insert("opt_e".to_string(), "需求未提及特殊字符处理，这不是必需的功能".to_string());
            map.insert("opt_f".to_string(), "代码存在多个问题，并不完全正确".to_string());
            map
        },
        key_points: vec![
            "仔细阅读需求，确保所有条件都被实现".to_string(),
            "简单的字符串包含检查往往不足以进行格式验证".to_string(),
            "边界条件（如最大值）容易被遗漏".to_string(),
        ],
    };
    
    engine.add_question_for_module("Options", sample_question);
}
