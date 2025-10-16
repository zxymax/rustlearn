// 第8课：错误处理 (Error Handling)
// 本文件详细介绍 Rust 中的错误处理机制

// 导入必要的模块
use std::fs::File;
use std::io::{self, Read, Write};
use std::num::ParseIntError;

// 定义一个公共函数 run()，作为本模块的入口点
pub fn run() {
    println!("=== 第8课：错误处理 ===");
    println!("本示例将介绍 Rust 中的错误处理机制。\n");
    
    // 调用各个示例函数
    error_types();
    panic_example();
    result_enum();
    error_propagation();
    custom_error_types();
    error_conversion();
    error_chaining();
    unwrap_and_expect();
    best_practices();
    error_libraries();
}

// 演示错误的类型
fn error_types() {
    println!("\n--- 错误的类型 ---");
    
    println!("Rust 中有两种主要的错误类型：");
    println!("1. 可恢复错误（Recoverable Errors）");
    println!("   - 表示可能会失败但程序可以继续执行的情况");
    println!("   - 使用 Result<T, E> 枚举来处理");
    println!("   - 例如：文件未找到、网络连接失败等");
    
    println!("2. 不可恢复错误（Unrecoverable Errors）");
    println!("   - 表示程序无法继续执行的严重错误");
    println!("   - 使用 panic! 宏来处理");
    println!("   - 例如：索引越界、断言失败等");
    
    println!("\nRust 的错误处理理念：");
    println!("- 显式处理错误而不是忽略它们");
    println!("- 错误也是值，可以像其他值一样处理");
    println!("- 区分可恢复和不可恢复错误，采取不同的处理策略");
}

// 演示 panic! 宏的使用
fn panic_example() {
    println!("\n--- panic! 宏的使用 ---");
    
    println!("panic! 宏用于处理不可恢复的错误，它会：");
    println!("1. 打印错误信息");
    println!("2. 展开调用栈（backtrace）");
    println!("3. 终止程序");
    
    println!("\n以下是 panic! 的示例，但我们不会实际触发它，因为它会终止程序：");
    println!("// panic!()");
    
    println!("\npanic! 的常见使用场景：");
    println!("1. 开发和调试阶段，用于快速发现和处理错误");
    println!("2. 发生了不可能恢复的严重错误");
    println!("3. 断言失败，验证条件不满足");
}

// 演示 Result 枚举的使用
fn result_enum() {
    println!("\n--- Result 枚举的使用 ---");
    
    println!("Result<T, E> 是一个枚举，用于处理可恢复的错误：");
    println!("- Ok(T)：表示操作成功，包含成功值");
    println!("- Err(E)：表示操作失败，包含错误值");
    
    // 示例：文件操作
    println!("\n文件操作示例：");
    let file_result = File::open("nonexistent_file.txt");
    
    match file_result {
        Ok(file) => println!("成功打开文件"),
        Err(error) => println!("无法打开文件: {:?}", error),
    }
    
    // 示例：字符串解析
    println!("\n字符串解析示例：");
    let number_str = "42";
    let parse_result = number_str.parse::<i32>();
    
    match parse_result {
        Ok(number) => println!("解析成功: {}", number),
        Err(error) => println!("解析失败: {:?}", error),
    }
    
    let invalid_str = "not a number";
    let invalid_result = invalid_str.parse::<i32>();
    
    match invalid_result {
        Ok(number) => println!("解析成功: {}", number),
        Err(error) => println!("解析失败: {:?}", error),
    }
}

// 演示错误传播
fn error_propagation() {
    println!("\n--- 错误传播 ---");
    
    println!("错误传播是指将函数中的错误传递给调用者处理：");
    println!("- 使用 ? 操作符可以简化错误传播代码");
    println!("- ? 操作符只能用于返回 Result<T, E> 或 Option<T> 的函数");
    
    // 定义一个读取文件内容的函数，不使用 ? 操作符
    fn read_file_verbose() -> Result<String, io::Error> {
        let f = File::open("nonexistent_file.txt");
        
        let mut f = match f {
            Ok(file) => file,
            Err(error) => return Err(error),
        };
        
        let mut contents = String::new();
        
        match f.read_to_string(&mut contents) {
            Ok(_) => Ok(contents),
            Err(error) => Err(error),
        }
    }
    
    // 定义一个读取文件内容的函数，使用 ? 操作符
    fn read_file_simple() -> Result<String, io::Error> {
        let mut f = File::open("nonexistent_file.txt")?;
        let mut contents = String::new();
        f.read_to_string(&mut contents)?;
        Ok(contents)
    }
    
    // 测试错误传播
    println!("\n测试 verbose 版本:");
    let result1 = read_file_verbose();
    match result1 {
        Ok(contents) => println!("文件内容: {}", contents),
        Err(error) => println!("错误: {:?}", error),
    }
    
    println!("\n测试 simple 版本 (使用 ? 操作符):");
    let result2 = read_file_simple();
    match result2 {
        Ok(contents) => println!("文件内容: {}", contents),
        Err(error) => println!("错误: {:?}", error),
    }
}

// 演示自定义错误类型
fn custom_error_types() {
    println!("\n--- 自定义错误类型 ---");
    
    println!("在实际项目中，我们经常需要定义自己的错误类型：");
    
    // 定义一个简单的自定义错误类型
    #[derive(Debug)]
    enum CustomError {
        IoError(io::Error),
        ParseError(ParseIntError),
        InvalidInput(String),
    }
    
    // 实现 From trait 用于错误转换
    impl From<io::Error> for CustomError {
        fn from(error: io::Error) -> Self {
            CustomError::IoError(error)
        }
    }
    
    impl From<ParseIntError> for CustomError {
        fn from(error: ParseIntError) -> Self {
            CustomError::ParseError(error)
        }
    }
    
    // 定义一个可能返回自定义错误的函数
    fn process_input(input: &str) -> Result<i32, CustomError> {
        if input.is_empty() {
            return Err(CustomError::InvalidInput("输入不能为空".to_string()));
        }
        
        // 使用 ? 操作符，会自动调用 from 方法转换错误类型
        let number = input.parse::<i32>()?;
        
        if number < 0 {
            return Err(CustomError::InvalidInput("输入必须为正数".to_string()));
        }
        
        Ok(number)
    }
    
    // 测试自定义错误类型
    let test_cases = ["42", "-1", "not a number", ""];
    
    for test in &test_cases {
        println!("\n测试输入: '{}'", test);
        let result = process_input(test);
        match result {
            Ok(value) => println!("处理成功: {}", value),
            Err(error) => println!("处理失败: {:?}", error),
        }
    }
}

// 演示错误转换
fn error_conversion() {
    println!("\n--- 错误转换 ---");
    
    println!("错误转换允许我们在不同的错误类型之间进行转换：");
    println!("- 使用 From trait 和 Into trait");
    println!("- 使用 map_err 方法转换错误类型");
    
    // 定义两种不同的错误类型
    #[derive(Debug)]
    struct NetworkError {
        message: String,
    }
    
    #[derive(Debug)]
    struct ApiError {
        error_code: u32,
        details: String,
    }
    
    // 实现 From trait 用于错误转换
    impl From<NetworkError> for ApiError {
        fn from(error: NetworkError) -> Self {
            ApiError {
                error_code: 503,
                details: format!("网络错误: {}", error.message),
            }
        }
    }
    
    // 定义可能返回不同错误的函数
    fn fetch_data() -> Result<String, NetworkError> {
        // 模拟网络错误
        Err(NetworkError {
            message: "连接超时".to_string(),
        })
    }
    
    // 使用 map_err 转换错误类型
    fn process_data_map_err() -> Result<String, ApiError> {
        fetch_data().map_err(|e| ApiError {
            error_code: 500,
            details: format!("处理数据失败: {}", e.message),
        })
    }
    
    // 测试错误转换
    println!("\n使用 map_err 转换错误:");
    let result1 = process_data_map_err();
    match result1 {
        Ok(data) => println!("成功获取数据: {}", data),
        Err(error) => println!("API 错误: {:?}", error),
    }
}

// 演示错误链
fn error_chaining() {
    println!("\n--- 错误链 ---");
    
    println!("错误链是指在处理错误时保留原始错误的上下文：");
    
    // 定义一些错误类型
    #[derive(Debug)]
    enum DataError {
        Parse(ParseIntError),
        Validation(String),
    }
    
    #[derive(Debug)]
    enum ServiceError {
        Data(DataError),
        Io(io::Error),
        Unexpected(String),
    }
    
    // 实现 From trait 用于构建错误链
    impl From<ParseIntError> for DataError {
        fn from(err: ParseIntError) -> Self {
            DataError::Parse(err)
        }
    }
    
    impl From<DataError> for ServiceError {
        fn from(err: DataError) -> Self {
            ServiceError::Data(err)
        }
    }
    
    // 模拟一个可能失败的解析函数
    fn parse_data(data: &str) -> Result<i32, DataError> {
        let num = data.parse::<i32>()?; // 自动转换为 DataError::Parse
        
        if num < 0 {
            return Err(DataError::Validation("数值必须为正数".to_string()));
        }
        
        Ok(num)
    }
    
    // 测试错误链
    println!("\n测试无效数字输入:");
    let result1 = parse_data("not a number");
    match result1 {
        Ok(msg) => println!("成功: {}", msg),
        Err(err) => println!("错误链: {:?}", err),
    }
}

// 演示 unwrap 和 expect 方法
fn unwrap_and_expect() {
    println!("\n--- unwrap 和 expect 方法 ---");
    
    println!("unwrap 和 expect 方法是处理 Result 和 Option 的便捷方法：");
    println!("- unwrap(): 如果是 Ok/Some 则返回值，否则 panic!");
    println!("- expect(msg): 类似于 unwrap，但提供自定义 panic 消息");
    
    // Ok 结果的 unwrap
    let ok_result: Result<i32, &str> = Ok(42);
    let value1 = ok_result.unwrap();
    println!("Ok.unwrap() = {}", value1);
    
    // Some 值的 unwrap
    let some_value: Option<i32> = Some(100);
    let value2 = some_value.unwrap();
    println!("Some.unwrap() = {}", value2);
    
    // 使用 expect 提供自定义错误消息
    let ok_result2: Result<i32, &str> = Ok(99);
    let value3 = ok_result2.expect("这不会发生");
    println!("Ok.expect() = {}", value3);
    
    println!("\nunwrap 和 expect 的适用场景：");
    println!("1. 原型开发和快速测试");
    println!("2. 确定不会失败的操作");
    println!("3. 开发和调试阶段");
}

// 演示错误处理的最佳实践
fn best_practices() {
    println!("\n--- 错误处理的最佳实践 ---");
    
    println!("Rust 错误处理的一些最佳实践：");
    println!("1. 优先使用 Result 处理可恢复错误，而不是 panic!");
    println!("2. 为公共 API 定义明确的错误类型");
    println!("3. 实现 From trait 以支持错误转换");
    println!("4. 使用 ? 操作符简化错误传播");
    println!("5. 提供有意义的错误信息");
    
    // 示例：实现良好的错误处理
    #[derive(Debug)]
    enum UserError {
        InvalidUsername(String),
        InvalidPasswordLength,
    }
    
    impl std::fmt::Display for UserError {
        fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
            match self {
                UserError::InvalidUsername(name) => 
                    write!(f, "无效的用户名: '{}'. 用户名必须至少包含 3 个字符.", name),
                UserError::InvalidPasswordLength => 
                    write!(f, "无效的密码长度. 密码必须至少包含 8 个字符.")
            }
        }
    }
    
    // 定义一个符合最佳实践的函数
    fn create_user(username: &str, password: &str) -> Result<(), UserError> {
        // 验证输入
        if username.len() < 3 {
            return Err(UserError::InvalidUsername(username.to_string()));
        }
        
        if password.len() < 8 {
            return Err(UserError::InvalidPasswordLength);
        }
        
        // 这里应该是实际的用户创建逻辑
        println!("用户创建逻辑将在这里执行...");
        
        // 模拟成功
        Ok(())
    }
    
    // 测试错误处理最佳实践
    let test_cases = [("alice", "password123"), ("bo", "password123"), ("charlie", "pass")];
    
    for (username, password) in &test_cases {
        println!("\n测试创建用户: username='{}', password='{}'", username, password);
        let result = create_user(username, password);
        match result {
            Ok(_) => println!("用户创建成功！"),
            Err(error) => println!("用户创建失败: {}", error),
        }
    }
}

// 演示错误处理库的使用
    // 演示错误处理库的使用
    fn error_libraries() {
        println!("\n--- 错误处理库的使用 ---");
        
        println!("Rust 社区提供了一些优秀的错误处理库，可以简化错误处理代码：");
        println!("1. thiserror: 主要用于定义库的错误类型");
        println!("2. anyhow: 主要用于应用程序中的错误处理");
        
        println!("\n使用 thiserror 的优势：");
        println!("- 自动派生常见的 trait（如 Debug、Display）");
        println!("- 简化 From trait 的实现");
        println!("- 支持错误原因链接");
        
        println!("\n使用 anyhow 的优势：");
        println!("- 可以处理任何实现了 Error trait 的错误类型");
        println!("- 提供了方便的上下文添加方法");
        println!("- 简化错误处理代码");
        
        println!("\n要使用这些库，需要在 Cargo.toml 中添加依赖:");
        println!("[dependencies]");
        println!("thiserror = \"1.0\"");
        println!("anyhow = \"1.0\"");
        
        println!("\n注意: 由于我们没有在当前项目中添加这些依赖,所以这里不提供具体的代码示例.");
    }