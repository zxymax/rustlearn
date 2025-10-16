// Rust 学习示例程序
// 本文件是一个知识点列表选择器，您可以选择不同的数字运行对应的 Rust 语法示例

// 导入标准输入输出模块
use std::io;

// 定义每个知识点示例的标题和描述
const LESSONS: &[(&str, &str)] = &[
    ("1", "变量和数据类型 (Variables and Data Types)"),
    ("2", "函数和流程控制 (Functions and Control Flow)"),
    ("3", "结构体 (Structs)"),
    ("4", "枚举 (Enums)"),
    ("5", "模式匹配 (Pattern Matching)"),
    ("6", "常见集合及其操作 (Collections)"),
    ("7", "包和模块 (Packages and Modules)"),
    ("8", "错误处理 (Error Handling)"),
    ("9", "泛型 (Generics)"),
    ("10", "生命周期 (Lifetimes)"),
];

fn main() {
    loop {
        // 打印欢迎信息
        println!("=== Rust 学习示例程序 ===");
        println!("请选择您想学习的知识点:");
        
        // 打印所有知识点列表
        for &(num, title) in LESSONS {
            println!("{}. {}", num, title);
        }
        
        println!("q. 退出程序");
        
        // 读取用户输入
        let mut choice = String::new();
        io::stdin().read_line(&mut choice).expect("无法读取输入");
        
        // 去除输入字符串中的换行符和空格
        let choice = choice.trim();
        
        // 根据用户选择执行对应的示例
        match choice {
            "1" => _01_variables::run(),
            "2" => _02_functions_control_flow::run(),
            "3" => _03_structs::run(),
            "4" => _04_enums::run(),
            "5" => _05_pattern_matching::run(),
            "6" => _06_collections::run(),
            "7" => _07_packages_modules::run(),
            "8" => _08_error_handling::run(),
            "9" => _09_generics::run(),
            "10" => _10_lifetimes::run(),
            "q" | "Q" => {
                println!("感谢使用 Rust 学习示例程序！再见！");
                break;
            },
            _ => println!("无效的选择，请重新输入。\n"),
        }
        
        // 等待用户按回车继续
        println!("\n按回车键继续...");
        let mut _continue = String::new();
        io::stdin().read_line(&mut _continue).expect("无法读取输入");
    }
}

// 以下模块声明将在创建对应文件后取消注释
// 第1课：变量和数据类型
mod _01_variables;

// 第2课：函数和流程控制
mod _02_functions_control_flow;

// 第3课：结构体
mod _03_structs;

// 第4课：枚举
mod _04_enums;

// 第5课：模式匹配
mod _05_pattern_matching;

// 第6课：常见集合及其操作
mod _06_collections;

// 第7课：包和模块
mod _07_packages_modules;

// 第8课：错误处理
mod _08_error_handling;

// 第9课：泛型
mod _09_generics;

// 第10课：生命周期
mod _10_lifetimes;

