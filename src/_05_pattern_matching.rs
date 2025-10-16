// 第5课：模式匹配 (Pattern Matching)
// 本文件详细介绍 Rust 中的模式匹配语法和应用场景
// 
// 知识点大纲：
// 1. match 表达式基础
// 2. 模式匹配中的解构
// 3. 模式匹配中的范围匹配
// 4. 模式匹配中的通配符
// 5. if let 表达式
// 6. while let 表达式
// 7. for 循环中的模式
// 8. let 语句中的模式
// 9. 函数参数中的模式
// 10. 高级模式匹配技巧

// 导入标准输出模块

// 定义一个公共函数 run()，作为本模块的入口点
// 当用户在主程序中选择 5 时，将调用此函数
pub fn run() {
    println!("=== 第5课：模式匹配 ===");
    println!("本示例将介绍 Rust 中的模式匹配语法和应用场景。\n");
    
    // 调用各个示例函数
    match_basics();
    pattern_destructuring();
    range_matching();
    wildcards();
    if_let_expressions();
    while_let_expressions();
    for_loops_with_patterns();
    let_statements_with_patterns();
    function_parameters_with_patterns();
    advanced_pattern_matching();
}

// 演示 match 表达式基础
// match 表达式是 Rust 中最强大的模式匹配工具
fn match_basics() {
    println!("\n--- match 表达式基础 ---");
    
    // 定义一个简单的枚举
    enum Coin {
        Penny,
        Nickel,
        Dime,
        Quarter,
    }
    
    // 定义一个函数，使用 match 表达式处理枚举
    fn value_in_cents(coin: Coin) -> u8 {
        match coin {
            Coin::Penny => 1,
            Coin::Nickel => 5,
            Coin::Dime => 10,
            Coin::Quarter => 25,
        }
    }
    
    // 测试 match 表达式
    let penny = Coin::Penny;
    let nickel = Coin::Nickel;
    let dime = Coin::Dime;
    let quarter = Coin::Quarter;
    
    println!("便士的价值: {}", value_in_cents(penny));
    println!("镍币的价值: {}", value_in_cents(nickel));
    println!("一角硬币的价值: {}", value_in_cents(dime));
    println!("二角五分硬币的价值: {}", value_in_cents(quarter));
    
    // match 表达式必须是穷尽的，必须覆盖所有可能的情况
    
    // 运行结果：
    // 便士的价值: 1
    // 镍币的价值: 5
    // 一角硬币的价值: 10
    // 二角五分硬币的价值: 25
}

// 演示模式匹配中的解构
// 模式匹配可以解构复杂的数据结构
fn pattern_destructuring() {
    println!("\n--- 模式匹配中的解构 ---");
    
    // 定义一个结构体
    struct Point {
        x: i32,
        y: i32,
    }
    
    // 定义一个枚举，包含结构体和元组
    enum Message {
        Quit,
        Move { x: i32, y: i32 },
        Write(String),
        ChangeColor(i32, i32, i32),
    }
    
    // 解构结构体
    let p = Point { x: 10, y: 20 };
    let Point { x, y } = p;
    println!("解构结构体: x = {}, y = {}", x, y);
    
    // 在 match 表达式中解构结构体
    match p {
        Point { x: 0, y } => println!("x 坐标为 0, y = {}", y),
        Point { x, y: 0 } => println!("y 坐标为 0, x = {}", x),
        Point { x, y } => println!("普通点: ({}, {})", x, y),
    }
    
    // 在 match 表达式中解构枚举
    let msg = Message::Move { x: 30, y: 40 };
    match msg {
        Message::Quit => println!("退出消息"),
        Message::Move { x, y } => println!("移动到: ({}, {})", x, y),
        Message::Write(text) => println!("写入文本: {}", text),
        Message::ChangeColor(r, g, b) => println!("更改为颜色: RGB({}, {}, {})", r, g, b),
    }
    
    // 运行结果：
    // 解构结构体: x = 10, y = 20
    // 普通点: (10, 20)
    // 移动到: (30, 40)
}

// 演示模式匹配中的范围匹配
// 可以使用范围运算符 ..= 来匹配一系列值
fn range_matching() {
    println!("\n--- 模式匹配中的范围匹配 ---");
    
    // 定义一个函数，使用范围匹配来判断成绩等级
    fn grade(score: u32) -> &'static str {
        match score {
            0..=59 => "不及格",
            60..=79 => "及格",
            80..=89 => "良好",
            90..=100 => "优秀",
            _ => "无效成绩",
        }
    }
    
    // 测试范围匹配
    println!("成绩 50: {}", grade(50));
    println!("成绩 75: {}", grade(75));
    println!("成绩 85: {}", grade(85));
    println!("成绩 95: {}", grade(95));
    println!("成绩 101: {}", grade(101));
    
    // 字符也可以进行范围匹配
    let c = 'R';
    match c {
        'A'..='Z' => println!("{} 是大写字母", c),
        'a'..='z' => println!("{} 是小写字母", c),
        '0'..='9' => println!("{} 是数字", c),
        _ => println!("{} 是其他字符", c),
    }
    
    // 运行结果：
    // 成绩 50: 不及格
    // 成绩 75: 及格
    // 成绩 85: 良好
    // 成绩 95: 优秀
    // 成绩 101: 无效成绩
    // R 是大写字母
}

// 演示模式匹配中的通配符
// 通配符 _ 用于匹配任何值，但不会绑定到变量
fn wildcards() {
    println!("\n--- 模式匹配中的通配符 ---");
    
    // 定义一个结构体
    struct Point {
        x: i32,
        y: i32,
        z: i32,
    }
    
    // 创建一个 Point 实例
    let p = Point { x: 1, y: 2, z: 3 };
    
    // 使用通配符忽略某些字段
    let Point { x, .. } = p;
    println!("只关心 x 坐标: x = {}", x);
    
    // 在 match 表达式中使用通配符
    match p {
        Point { x: 0, y, z } => println!("x 为 0: y = {}, z = {}", y, z),
        Point { x, y: 0, z } => println!("y 为 0: x = {}, z = {}", x, z),
        Point { x, y, z: 0 } => println!("z 为 0: x = {}, y = {}", x, y),
        _ => println!("所有坐标都不为 0"),
    }
    
    // 定义一个枚举
    enum Color {
        Red,
        Green,
        Blue,
        Custom(u8, u8, u8),
    }
    
    // 在 match 表达式中使用通配符
    let color = Color::Custom(255, 0, 0);
    match color {
        Color::Red => println!("红色"),
        Color::Green => println!("绿色"),
        Color::Blue => println!("蓝色"),
        Color::Custom(r, g, b) => println!("自定义颜色: RGB({}, {}, {})", r, g, b),
        _ => println!("其他颜色"), // 这个分支永远不会执行，因为我们已经覆盖了所有变体
    }
    
    // 运行结果：
    // 只关心 x 坐标: x = 1
    // 所有坐标都不为 0
    // 自定义颜色: RGB(255, 0, 0)
}

// 演示 if let 表达式
// if let 表达式是 match 表达式的简化形式，用于处理只有一个模式需要匹配的情况
fn if_let_expressions() {
    println!("\n--- if let 表达式 ---");
    
    // 定义一个 Option 类型
    let some_number: Option<i32> = Some(42);
    let absent_number: Option<i32> = None;
    
    // 使用 match 表达式处理 Option
    match some_number {
        Some(n) => println!("有值: {}", n),
        _ => (),
    }
    
    // 使用 if let 表达式简化上面的代码
    if let Some(n) = some_number {
        println!("使用 if let 有值: {}", n);
    }
    
    // if let 也可以有 else 部分
    if let Some(n) = absent_number {
        println!("这个不会执行，因为 absent_number 是 None");
    } else {
        println!("absent_number 是 None");
    }
    
    // 定义一个枚举
    enum Coin {
        Penny,
        Nickel,
        Dime,
        Quarter,
    }
    
    // 使用 if let 处理枚举
    let coin = Coin::Penny;
    if let Coin::Penny = coin {
        println!("找到一个便士！");
    }
    
    // 运行结果：
    // 有值: 42
    // 使用 if let 有值: 42
    // absent_number 是 None
    // 找到一个便士！
}

// 演示 while let 表达式
// while let 表达式结合了 while 循环和 if let 表达式的功能
fn while_let_expressions() {
    println!("\n--- while let 表达式 ---");
    
    // 创建一个可变的 Vec
    let mut stack = vec![1, 2, 3, 4, 5];
    
    // 使用 while let 表达式弹出并处理栈中的元素
    println!("弹出栈中的元素:");
    while let Some(top) = stack.pop() {
        println!("弹出: {}", top);
    }
    
    // 定义一个包含 Option 的迭代器
    let mut optional_numbers = vec![Some(1), Some(2), None, Some(4), None, Some(6)];
    
    // 使用 while let 处理包含 Option 的迭代器
    println!("\n处理包含 None 的迭代器:");
    while let Some(optional) = optional_numbers.pop() {
        if let Some(number) = optional {
            println!("处理数字: {}", number);
        } else {
            println!("遇到 None");
        }
    }
    
    // 运行结果：
    // 弹出栈中的元素:
    // 弹出: 5
    // 弹出: 4
    // 弹出: 3
    // 弹出: 2
    // 弹出: 1
    // 
    // 处理包含 None 的迭代器:
    // 处理数字: 6
    // 遇到 None
    // 处理数字: 4
    // 遇到 None
    // 处理数字: 2
    // 处理数字: 1
}

// 演示 for 循环中的模式
// for 循环中也可以使用模式匹配
fn for_loops_with_patterns() {
    println!("\n--- for 循环中的模式 ---");
    
    // 定义一个数组
    let v = vec![10, 20, 30, 40, 50];
    
    // 使用 for 循环和 enumerate 方法获取索引和值
    println!("遍历数组并获取索引:");
    for (index, value) in v.iter().enumerate() {
        println!("索引 {}: 值 {}", index, value);
    }
    
    // 定义一个元组数组
    let positions = [(1, 2), (3, 4), (5, 6)];
    
    // 使用 for 循环解构元组
    println!("\n遍历元组数组:");
    for (x, y) in positions.iter() {
        println!("位置: ({}, {})", x, y);
    }
    
    // 运行结果：
    // 遍历数组并获取索引:
    // 索引 0: 值 10
    // 索引 1: 值 20
    // 索引 2: 值 30
    // 索引 3: 值 40
    // 索引 4: 值 50
    // 
    // 遍历元组数组:
    // 位置: (1, 2)
    // 位置: (3, 4)
    // 位置: (5, 6)
}

// 演示 let 语句中的模式
// let 语句本身也是一种模式匹配
fn let_statements_with_patterns() {
    println!("\n--- let 语句中的模式 ---");
    
    // 基本的 let 语句
    let x = 5;
    println!("x = {}", x);
    
    // 使用模式匹配解构元组
    let (a, b) = (10, 20);
    println!("解构元组: a = {}, b = {}", a, b);
    
    // 定义一个结构体
    struct Point {
        x: i32,
        y: i32,
    }
    
    // 使用模式匹配解构结构体
    let p = Point { x: 30, y: 40 };
    let Point { x: px, y: py } = p;
    println!("解构结构体: px = {}, py = {}", px, py);
    
    // 简化的结构体解构语法
    let Point { x, y } = p;
    println!("简化解构: x = {}, y = {}", x, y);
    
    // 使用模式匹配忽略某些值
    let (_, c) = (50, 60);
    println!("忽略第一个值: c = {}", c);
    
    // 运行结果：
    // x = 5
    // 解构元组: a = 10, b = 20
    // 解构结构体: px = 30, py = 40
    // 简化解构: x = 30, y = 40
    // 忽略第一个值: c = 60
}

// 演示函数参数中的模式
// 函数参数也可以使用模式匹配
fn function_parameters_with_patterns() {
    println!("\n--- 函数参数中的模式 ---");
    
    // 定义一个接受元组参数的函数
    fn print_coordinates((x, y): (i32, i32)) {
        println!("坐标: ({}, {})", x, y);
    }
    
    // 调用函数
    let point = (100, 200);
    print_coordinates(point);
    
    // 定义一个结构体
    struct Rectangle {
        width: u32,
        height: u32,
    }
    
    // 定义一个接受结构体参数的函数
    fn area(&Rectangle { width, height }: &Rectangle) -> u32 {
        width * height
    }
    
    // 创建一个 Rectangle 实例并调用函数
    let rect = Rectangle { width: 10, height: 20 };
    println!("矩形面积: {}", area(&rect));
    
    // 定义一个接受 Option 参数的函数
    fn process_option(option: Option<i32>) {
        match option {
            Some(value) => println!("处理值: {}", value),
            None => println!("没有值"),
        }
    }
    
    // 调用函数
    process_option(Some(42));
    process_option(None);
    
    // 运行结果：
    // 坐标: (100, 200)
    // 矩形面积: 200
    // 处理值: 42
    // 没有值
}

// 演示高级模式匹配技巧
// Rust 的模式匹配非常强大，支持许多高级特性
fn advanced_pattern_matching() {
    println!("\n--- 高级模式匹配技巧 ---");
    
    // 1. 匹配守卫（Match Guards）
    // 匹配守卫是附加在 match 分支后的条件
    let num = Some(4);
    match num {
        Some(x) if x < 5 => println!("小于 5 的数字: {}", x),
        Some(x) => println!("大于或等于 5 的数字: {}", x),
        None => println!("没有数字"),
    }
    
    // 2. @ 绑定
    // @ 符号允许我们在模式中创建变量并同时测试它
    enum Message {
        Hello { id: i32 },
    }
    
    let msg = Message::Hello { id: 5 };
    match msg {
        Message::Hello { id: id_variable @ 3..=7 } => {
            println!("找到 ID 在范围内: {}", id_variable);
        },
        Message::Hello { id: 10..=12 } => {
            println!("找到 ID 在 10-12 范围内");
        },
        Message::Hello { id } => {
            println!("找到其他 ID: {}", id);
        },
    }
    
    // 3. 解构嵌套结构体
    struct Point {
        x: i32,
        y: i32,
    }
    
    struct Rectangle {
        top_left: Point,
        bottom_right: Point,
    }
    
    let rect = Rectangle {
        top_left: Point { x: 0, y: 10 },
        bottom_right: Point { x: 10, y: 0 },
    };
    
    match rect {
        Rectangle {
            top_left: Point { x: left, y: top },
            bottom_right: Point { x: right, y: bottom },
        } => {
            println!("矩形: 左上角({}, {}), 右下角({}, {})", left, top, right, bottom);
        },
    }
    
    // 运行结果：
    // 小于 5 的数字: 4
    // 找到 ID 在范围内: 5
    // 矩形: 左上角(0, 10), 右下角(10, 0)
}

// 知识点总结：
// 1. match 表达式基础：使用 match 关键字进行模式匹配，必须覆盖所有可能的情况
// 2. 模式匹配中的解构：可以解构结构体、枚举和元组等复杂数据结构
// 3. 模式匹配中的范围匹配：使用 ..= 运算符匹配一系列值
// 4. 模式匹配中的通配符：使用 _ 忽略某些值
// 5. if let 表达式：match 表达式的简化形式，用于处理只有一个模式需要匹配的情况
// 6. while let 表达式：结合了 while 循环和 if let 表达式的功能
// 7. for 循环中的模式：可以在 for 循环中使用模式匹配
// 8. let 语句中的模式：let 语句本身也是一种模式匹配
// 9. 函数参数中的模式：函数参数也可以使用模式匹配
// 10. 高级模式匹配技巧：包括匹配守卫、@ 绑定和解构嵌套结构体等
// 11. 模式匹配的优点：使代码更简洁、更安全，减少运行时错误