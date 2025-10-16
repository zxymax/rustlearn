// 第4课：枚举 (Enums)
// 本文件详细介绍 Rust 中的枚举定义、模式匹配、关联数据和方法等知识
// 
// 知识点大纲：
// 1. 枚举定义与实例化
// 2. 枚举的变体
// 3. 枚举的模式匹配
// 4. 带关联数据的枚举
// 5. 为枚举实现方法
// 6. Option 枚举
// 7. Result 枚举

// 导入标准输出模块

// 定义全局可见的枚举
// 方向枚举
enum Direction {
    North,
    East,
    South,
    West,
}

// 消息枚举
enum Message {
    Quit,
    Move { x: i32, y: i32 },
    Write(String),
    ChangeColor(i32, i32, i32),
}

// IP地址枚举
enum IpAddr {
    V4(u8, u8, u8, u8),
    V6(String),
}

// 用户输入枚举
enum Input {
    Number(i32),
    Text(String),
    Boolean(bool),
}

// HTTP状态码枚举
enum HttpStatusCode {
    Ok = 200,
    Created = 201,
    BadRequest = 400,
    Unauthorized = 401,
    NotFound = 404,
    InternalServerError = 500,
}

// 硬币枚举
enum Coin {
    Penny,
    Nickel,
    Dime,
    Quarter,
}

// 定义一个公共函数 run()，作为本模块的入口点
// 当用户在主程序中选择 4 时，将调用此函数
pub fn run() {
    println!("=== 第4课：枚举 ===");
    println!("本示例将介绍 Rust 中的枚举定义、模式匹配、关联数据和方法等知识。\n");
    
    // 调用各个示例函数
    enum_definition();
    enum_variants();
    enum_pattern_matching();
    enum_with_data();
    enum_methods();
    option_enum();
    result_enum();
}

// 演示枚举的定义与实例化
// 枚举是一种自定义数据类型，它允许我们定义一组命名的值
fn enum_definition() {
    println!("\n--- 枚举定义与实例化 ---");
    
    // 实例化枚举
    let north = Direction::North;
    let east = Direction::East;
    let south = Direction::South;
    let west = Direction::West;
    
    // 使用 match 表达式处理枚举
    println!("枚举实例化与匹配:");
    print_direction(north);
    print_direction(east);
    print_direction(south);
    print_direction(west);
    
    // 运行结果：
    // 枚举实例化与匹配:
    // 向北
    // 向东
    // 向南
    // 向西
}

// 演示枚举的变体
// 枚举的每个可能值称为变体（variant）
fn enum_variants() {
    println!("\n--- 枚举的变体 ---");
    
    // 枚举变体可以有整数常量值
    let status_ok = HttpStatusCode::Ok;
    let status_not_found = HttpStatusCode::NotFound;
    
    // 获取枚举变体的整数值
    println!("HTTP OK 状态码: {}", status_ok as i32);
    println!("HTTP Not Found 状态码: {}", status_not_found as i32);
    
    // 实例化复杂枚举
    let msg1 = Message::Quit;
    let msg2 = Message::Move { x: 10, y: 20 };
    let msg3 = Message::Write(String::from("Hello, Rust!"));
    let msg4 = Message::ChangeColor(255, 0, 0);
    
    // 打印消息类型
    println!("\n复杂枚举类型:");
    print_message_type(msg1);
    print_message_type(msg2);
    print_message_type(msg3);
    print_message_type(msg4);
    
    // 运行结果：
    // HTTP OK 状态码: 200
    // HTTP Not Found 状态码: 404
    // 
    // 复杂枚举类型:
    // 消息类型: Quit
    // 消息类型: Move
    // 消息类型: Write
    // 消息类型: ChangeColor
}

// 演示枚举的模式匹配
// 模式匹配是处理枚举的强大工具
fn enum_pattern_matching() {
    println!("\n--- 枚举的模式匹配 ---");
    
    // 定义一个函数，使用 match 表达式处理枚举
    fn value_in_cents(coin: Coin) -> u8 {
        match coin {
            Coin::Penny => {
                println!("幸运便士！");
                1
            },
            Coin::Nickel => 5,
            Coin::Dime => 10,
            Coin::Quarter => 25,
        }
    }
    
    // 测试模式匹配
    let penny = Coin::Penny;
    let nickel = Coin::Nickel;
    
    println!("便士的价值: {}", value_in_cents(penny));
    println!("镍币的价值: {}", value_in_cents(nickel));
    
    // 运行结果：
    // 幸运便士！
    // 便士的价值: 1
    // 镍币的价值: 5
}

// 演示带关联数据的枚举
// 枚举的变体可以携带不同类型和数量的数据
fn enum_with_data() {
    println!("\n--- 带关联数据的枚举 ---");
    
    // 实例化带关联数据的枚举
    let home = IpAddr::V4(127, 0, 0, 1);
    let loopback = IpAddr::V6(String::from("::1"));
    
    // 处理带关联数据的枚举
    print_ip_address(home);
    print_ip_address(loopback);
    
    // 实例化并处理用户输入枚举
    let input1 = Input::Number(42);
    let input2 = Input::Text(String::from("Hello"));
    let input3 = Input::Boolean(true);
    
    println!("\n用户输入处理:");
    process_input(input1);
    process_input(input2);
    process_input(input3);
    
    // 运行结果：
    // IPv4 地址: 127.0.0.1
    // IPv6 地址: ::1
    // 
    // 用户输入处理:
    // 数字输入: 42
    // 文本输入: Hello
    // 布尔输入: true
}

// 为枚举定义方法
// 可以为枚举实现方法，类似于为结构体实现方法
impl Message {
    // 定义一个方法，返回消息的描述
    fn description(&self) -> String {
        match self {
            Message::Quit => String::from("退出消息"),
            Message::Move { x, y } => format!("移动到坐标 ({}, {})", x, y),
            Message::Write(text) => format!("写入文本: {}", text),
            Message::ChangeColor(r, g, b) => format!("更改为 RGB 颜色({}, {}, {})", r, g, b),
        }
    }
}

// 演示为枚举实现方法
fn enum_methods() {
    println!("\n--- 为枚举实现方法 ---");
    
    // 实例化 Message 枚举
    let msg1 = Message::Quit;
    let msg2 = Message::Move { x: 10, y: 20 };
    let msg3 = Message::Write(String::from("Hello, Rust!"));
    let msg4 = Message::ChangeColor(255, 0, 0);
    
    // 调用枚举的方法
    println!("消息 1 描述: {}", msg1.description());
    println!("消息 2 描述: {}", msg2.description());
    println!("消息 3 描述: {}", msg3.description());
    println!("消息 4 描述: {}", msg4.description());
    
    // 运行结果：
    // 消息 1 描述: 退出消息
    // 消息 2 描述: 移动到坐标 (10, 20)
    // 消息 3 描述: 写入文本: Hello, Rust!
    // 消息 4 描述: 更改为 RGB 颜色(255, 0, 0)
}

// 演示 Option 枚举
// Option 是 Rust 标准库中的枚举，用于表示可能存在或不存在的值
fn option_enum() {
    println!("\n--- Option 枚举 ---");
    
    // Option<T> 枚举有两个变体:
    // - Some(T): 包含一个值
    // - None: 不包含任何值
    
    // 创建 Option 类型
    let some_number = Some(5);
    let some_string = Some(String::from("Hello"));
    let absent_number: Option<i32> = None;
    
    // 访问 Option 中的值
    println!("Option 值处理:");
    print_option(some_number);
    print_option_string(some_string);
    print_option(absent_number);
    
    // 使用 unwrap 方法访问 Option 中的值（如果是 None 会导致程序崩溃）
    // 注意：在实际代码中应谨慎使用 unwrap
    let value = some_number.unwrap();
    println!("使用 unwrap 获取的值: {}", value);
    
    // 使用 unwrap_or 提供默认值
    let default_value = absent_number.unwrap_or(0);
    println!("使用 unwrap_or 获取的值: {}", default_value);
    
    // 运行结果：
    // Option 值处理:
    // 有值: 5
    // 有字符串值: Hello
    // 无值
    // 使用 unwrap 获取的值: 5
    // 使用 unwrap_or 获取的值: 0
}

// 演示 Result 枚举
// Result 是 Rust 标准库中的枚举，用于处理可能成功或失败的操作
fn result_enum() {
    println!("\n--- Result 枚举 ---");
    
    // Result<T, E> 枚举有两个变体:
    // - Ok(T): 表示操作成功，包含成功的值
    // - Err(E): 表示操作失败，包含错误信息
    
    // 定义一个可能失败的函数
    fn divide(a: i32, b: i32) -> Result<i32, String> {
        if b == 0 {
            Err(String::from("除数不能为零"))
        } else {
            Ok(a / b)
        }
    }
    
    // 测试成功的情况
    let result1 = divide(10, 2);
    println!("10 / 2 = {}", match result1 {
        Ok(value) => value.to_string(),
        Err(err) => err,
    });
    
    // 测试失败的情况
    let result2 = divide(10, 0);
    println!("10 / 0 = {}", match result2 {
        Ok(value) => value.to_string(),
        Err(err) => err,
    });
    
    // 使用 ? 运算符简化错误处理
    // 注意：? 运算符只能在返回 Result 或 Option 的函数中使用
    fn calculate(a: i32, b: i32) -> Result<i32, String> {
        let intermediate = divide(a, b)?; // 如果 divide 返回 Err，calculate 也会返回这个 Err
        Ok(intermediate * 2)
    }
    
    let result3 = calculate(10, 2);
    let result4 = calculate(10, 0);
    
    println!("calculate(10, 2) = {}", match result3 {
        Ok(value) => value.to_string(),
        Err(err) => err,
    });
    
    println!("calculate(10, 0) = {}", match result4 {
        Ok(value) => value.to_string(),
        Err(err) => err,
    });
    
    // 运行结果：
    // 10 / 2 = 5
    // 10 / 0 = 除数不能为零
    // calculate(10, 2) = 10
    // calculate(10, 0) = 除数不能为零
}

// 以下是示例中使用的辅助函数

// 打印方向
// 参数 dir: Direction - 要打印的方向
fn print_direction(dir: Direction) {
    match dir {
        Direction::North => println!("向北"),
        Direction::East => println!("向东"),
        Direction::South => println!("向南"),
        Direction::West => println!("向西"),
    }
}

// 打印消息类型
// 参数 msg: Message - 要打印类型的消息
fn print_message_type(msg: Message) {
    match msg {
        Message::Quit => println!("消息类型: Quit"),
        Message::Move { .. } => println!("消息类型: Move"),
        Message::Write(_) => println!("消息类型: Write"),
        Message::ChangeColor(_, _, _) => println!("消息类型: ChangeColor"),
    }
}

// 打印 IP 地址
// 参数 ip: IpAddr - 要打印的 IP 地址
fn print_ip_address(ip: IpAddr) {
    match ip {
        IpAddr::V4(a, b, c, d) => println!("IPv4 地址: {}.{}.{}.{}", a, b, c, d),
        IpAddr::V6(s) => println!("IPv6 地址: {}", s),
    }
}

// 处理用户输入
// 参数 input: Input - 要处理的用户输入
fn process_input(input: Input) {
    match input {
        Input::Number(n) => println!("数字输入: {}", n),
        Input::Text(s) => println!("文本输入: {}", s),
        Input::Boolean(b) => println!("布尔输入: {}", b),
    }
}

// 打印 Option<i32> 值
// 参数 option: Option<i32> - 要打印的 Option 值
fn print_option(option: Option<i32>) {
    match option {
        Some(value) => println!("有值: {}", value),
        None => println!("无值"),
    }
}

// 打印 Option<String> 值
// 参数 option: Option<String> - 要打印的 Option 值
fn print_option_string(option: Option<String>) {
    match option {
        Some(value) => println!("有字符串值: {}", value),
        None => println!("无值"),
    }
}

// 知识点总结：
// 1. 枚举定义：使用 enum 关键字，定义一组命名的值（变体）
// 2. 枚举实例化：使用枚举名称::变体名称的语法
// 3. 枚举的变体：可以有不同的形式，包括空变体、元组变体和结构体变体
// 4. 枚举的模式匹配：使用 match 表达式处理枚举的不同变体
// 5. 带关联数据的枚举：枚举的变体可以携带数据
// 6. 为枚举实现方法：使用 impl 块为枚举定义方法
// 7. Option 枚举：标准库中的枚举，表示可能存在或不存在的值
// 8. Result 枚举：标准库中的枚举，表示可能成功或失败的操作
// 9. ? 运算符：用于简化错误处理，只能在返回 Result 或 Option 的函数中使用
// 10. 枚举的用途：枚举是 Rust 中处理多种可能情况的强大工具，可以替代空值和错误代码