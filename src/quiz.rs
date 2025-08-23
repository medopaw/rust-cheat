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
use rand::seq::SliceRandom;
use rand::thread_rng;
use serde::{Deserialize, Serialize};

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
        println!("DEBUG: 查找模块 '{}' -> 提取的key: '{}'", module, module_key);
        println!("DEBUG: 当前存储的模块有: {:?}", self.questions.keys().collect::<Vec<_>>());
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
        }
    }

    pub fn toggle_selection(&mut self, option_index: usize) {
        if let Some(option) = self.displayed_options.get(option_index) {
            if self.user_selections.contains(&option.id) {
                self.user_selections.remove(&option.id);
            } else {
                self.user_selections.insert(option.id.clone());
            }
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
    pub fn display_question(session: &QuizSession) -> io::Result<()> {
        if let Some(question) = &session.current_question {
            Self::clear_screen();
            
            println!("🎯 {} 模块 - Code Review 题目", session.module);
            println!("===============================================");
            println!("\n📋 {}", question.title);
            println!("\n需求说明:");
            println!("{}", question.description);
            println!("\nAI 生成的代码:");
            println!("```rust");
            println!("{}", question.code);
            println!("```");
            println!("\n问题：作为 Code Reviewer，你认为这段代码存在哪些问题？（多选）");
            println!();

            Self::display_options(&session.question_state)?;
            Self::display_instructions(&session.question_state);
        }
        Ok(())
    }

    pub fn display_options(state: &QuestionState) -> io::Result<()> {
        for (i, option) in state.displayed_options.iter().enumerate() {
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

            let focus_marker = if is_focused { "►" } else { " " };
            let selection_marker = if is_selected { "[x]" } else { "[ ]" };

            println!("{} {}. {} {}", focus_marker, letter, selection_marker, option.content);
            if matches!(state.phase, AnswerPhase::ShowingHints | AnswerPhase::FinalAnswer) {
                print!("    {}", status_indicator);
            }
            println!();
        }
        Ok(())
    }

    pub fn display_instructions(state: &QuestionState) {
        println!();
        match state.phase {
            AnswerPhase::FirstAttempt => {
                println!("💡 输入选项编号(1-{})选择/取消，直接按Enter提交答案，输入q退出", state.displayed_options.len());
                println!("   也可以输入: space(空格选择当前焦点), j/k(上下移动焦点)");
            }
            AnswerPhase::ShowingHints => {
                println!("💡 根据上面的状态指示，重新选择正确的选项组合");
                println!("   ✅ = 正确选项, ❌ = 错误选项");
            }
            AnswerPhase::FinalAnswer => {
                println!("🎉 查看详细解释，按 Enter 继续下一题...");
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
        print!("\x1B[2J\x1B[1;1H");
        io::stdout().flush().unwrap();
    }

    pub fn get_user_input() -> io::Result<UserInput> {
        use std::io::{stdin, Read};
        
        print!("\n> ");
        io::stdout().flush()?;
        
        let mut line = String::new();
        stdin().read_line(&mut line)?;
        let input = line.trim();
        
        match input {
            "q" | "Q" | "quit" => Ok(UserInput::Quit),
            " " | "space" => Ok(UserInput::ToggleSelection),
            "" | "enter" | "submit" => Ok(UserInput::Submit),
            "j" | "J" | "down" => Ok(UserInput::MoveFocus(FocusDirection::Down)),
            "k" | "K" | "up" => Ok(UserInput::MoveFocus(FocusDirection::Up)),
            _ => {
                if let Ok(index) = input.parse::<usize>() {
                    Ok(UserInput::SelectByIndex(index))
                } else {
                    Ok(UserInput::Unknown)
                }
            }
        }
    }
}

#[derive(Debug)]
pub enum UserInput {
    MoveFocus(FocusDirection),
    ToggleSelection,
    Submit,
    Quit,
    SelectByIndex(usize),
    Unknown,
}

impl Default for QuizEngine {
    fn default() -> Self {
        Self::new()
    }
}

impl QuizSession {
    pub fn run_quiz_loop(&mut self) -> io::Result<bool> {
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
                    self.question_state.move_focus(direction);
                    QuizUI::display_question(self)?;
                }
                UserInput::ToggleSelection => {
                    let focus_index = self.question_state.current_focus;
                    self.question_state.toggle_selection(focus_index);
                    QuizUI::display_question(self)?;
                }
                UserInput::SelectByIndex(index) => {
                    if index > 0 && index <= self.question_state.displayed_options.len() {
                        self.question_state.toggle_selection(index - 1);
                        QuizUI::display_question(self)?;
                    }
                }
                UserInput::Submit => {
                    return self.handle_answer_submission();
                }
                UserInput::Quit => {
                    return Ok(QuizResult::Quit);
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
    println!("DEBUG: 加载模块 '{}' -> 提取的key: '{}'", module, module_key);
    
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
            println!("DEBUG: 未匹配到模块key '{}'", module_key);
            return;
        }
    };
    
    let file_path = format!("questions/{}", filename);
    println!("DEBUG: 尝试加载文件: {}", file_path);
    
    match load_questions_from_file(&file_path) {
        Ok(questions) => {
            println!("DEBUG: 成功加载 {} 道题目", questions.len());
            for question in questions {
                engine.add_question_for_module(&module_key, question);
            }
        }
        Err(e) => {
            println!("DEBUG: 加载失败: {}, 回退到硬编码", e);
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
