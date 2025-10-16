// 第2课：函数和流程控制 (Functions and Control Flow)
// 本文件详细介绍 Rust 中的函数定义、参数传递、返回值以及各种流程控制语句
// 
// 知识点大纲：
// 1. 函数定义与调用
// 2. 函数参数
// 3. 函数返回值
// 4. if/else 条件语句
// 5. loop 循环语句
// 6. while 循环语句
// 7. for 循环语句
// 8. break 和 continue 关键字
// 9. match 表达式

// 导入标准输出模块

// 定义一个公共函数 run()，作为本模块的入口点
// 当用户在主程序中选择 2 时，将调用此函数
pub fn run() {
    println!("=== 第2课：函数和流程控制 ===");
    println!("本示例将介绍 Rust 中的函数定义、参数传递、返回值以及各种流程控制语句。\n");
    
    // 调用各个示例函数
    function_definition();
    function_parameters();
    function_return_values();
    if_else_statements();
    loop_statements();
    while_statements();
    for_statements();
    break_continue_keywords();
    match_expressions();
}

// 演示函数的定义与调用
// 在 Rust 中，函数使用 fn 关键字定义，函数名使用 snake_case 命名规范
fn function_definition() {
    println!("\n--- 函数的定义与调用 ---");
    
    // 调用一个简单的函数
    say_hello();
    
    // 调用带参数的函数
    greet_person("Alice");
    
    // 调用带返回值的函数
    let sum = calculate_sum(5, 10);
    println!("5 + 10 = {}", sum);
    
    // 运行结果：
    // Hello, Rust!
    // Hello, Alice!
    // 5 + 10 = 15
}

// 演示函数参数
// Rust 中的函数参数需要显式指定类型
fn function_parameters() {
    println!("\n--- 函数参数 ---");
    
    // 调用带多个参数的函数
    let result = multiply(3, 4);
    println!("3 * 4 = {}", result);
    
    // 调用带不同类型参数的函数
    let description = describe_number(42, true);
    println!("描述: {}", description);
    
    // 调用带可变引用参数的函数
    let mut value = 10;
    increment(&mut value, 5);
    println!("递增后的值: {}", value);
    
    // 运行结果：
    // 3 * 4 = 12
    // 描述: 42 是一个很大的数字
    // 递增后的值: 15
}

// 演示函数返回值
// Rust 中的函数可以返回单个值，使用 -> 指定返回类型
fn function_return_values() {
    println!("\n--- 函数返回值 ---");
    
    // 调用返回单个值的函数
    let square_result = square(5);
    println!("5 的平方 = {}", square_result);
    
    // 调用使用 return 关键字的函数
    let max_value = find_max(10, 20);
    println!("10 和 20 中的最大值 = {}", max_value);
    
    // 调用返回元组的函数（多返回值）
    let (sum_result, product_result) = calculate_sum_and_product(3, 7);
    println!("3 + 7 = {}, 3 * 7 = {}", sum_result, product_result);
    
    // 运行结果：
    // 5 的平方 = 25
    // 10 和 20 中的最大值 = 20
    // 3 + 7 = 10, 3 * 7 = 21
}

// 演示 if/else 条件语句
// Rust 的 if 语句是表达式，而不是语句，这意味着它可以返回一个值
fn if_else_statements() {
    println!("\n--- if/else 条件语句 ---");
    
    // 基本的 if/else 结构
    let number = 7;
    if number > 5 {
        println!("{} 大于 5", number);
    } else {
        println!("{} 小于或等于 5", number);
    }
    
    // if 作为表达式返回值
    let result = if number % 2 == 0 {
        "偶数"
    } else {
        "奇数"
    };
    println!("{} 是一个 {}", number, result);
    
    // 多个 else if 分支
    let score = 85;
    if score >= 90 {
        println!("优秀");
    } else if score >= 80 {
        println!("良好");
    } else if score >= 60 {
        println!("及格");
    } else {
        println!("不及格");
    }
    
    // 运行结果：
    // 7 大于 5
    // 7 是一个 奇数
    // 良好
}

// 演示 loop 循环语句
// loop 语句创建一个无限循环，可以使用 break 语句退出
fn loop_statements() {
    println!("\n--- loop 循环语句 ---");
    
    // 基本的 loop 循环
    let mut count = 0;
    loop {
        count += 1;
        println!("循环计数: {}", count);
        
        if count >= 3 {
            break; // 当计数达到 3 时退出循环
        }
    }
    
    // loop 作为表达式返回值
    let mut attempts = 0;
    let result = loop {
        attempts += 1;
        println!("尝试次数: {}", attempts);
        
        if attempts == 5 {
            break attempts * 10; // 返回一个值
        }
    };
    println!("loop 表达式返回值: {}", result);
    
    // 运行结果：
    // 循环计数: 1
    // 循环计数: 2
    // 循环计数: 3
    // 尝试次数: 1
    // 尝试次数: 2
    // 尝试次数: 3
    // 尝试次数: 4
    // 尝试次数: 5
    // loop 表达式返回值: 50
}

// 演示 while 循环语句
// while 循环在条件为真时执行
fn while_statements() {
    println!("\n--- while 循环语句 ---");
    
    // 基本的 while 循环
    let mut countdown = 5;
    while countdown > 0 {
        println!("倒计时: {}", countdown);
        countdown -= 1;
    }
    println!("倒计时结束！");
    
    // 使用 while 循环遍历数组
    let numbers = [10, 20, 30, 40, 50];
    let mut index = 0;
    while index < numbers.len() {
        println!("数组元素[{}]: {}", index, numbers[index]);
        index += 1;
    }
    
    // 运行结果：
    // 倒计时: 5
    // 倒计时: 4
    // 倒计时: 3
    // 倒计时: 2
    // 倒计时: 1
    // 倒计时结束！
    // 数组元素[0]: 10
    // 数组元素[1]: 20
    // 数组元素[2]: 30
    // 数组元素[3]: 40
    // 数组元素[4]: 50
}

// 演示 for 循环语句
// for 循环在迭代器上执行，是 Rust 中最常用的循环形式
fn for_statements() {
    println!("\n--- for 循环语句 ---");
    
    // 遍历范围
    println!("遍历范围 1 到 5:");
    for number in 1..6 {
        println!("数字: {}", number);
    }
    
    // 遍历数组
    let fruits = ["苹果", "香蕉", "橙子", "葡萄"];
    println!("\n遍历水果数组:");
    for fruit in fruits.iter() {
        println!("水果: {}", fruit);
    }
    
    // 遍历带索引的数组
    println!("\n遍历带索引的水果数组:");
    for (index, fruit) in fruits.iter().enumerate() {
        println!("水果[{}]: {}", index, fruit);
    }
    
    // 遍历字符串
    println!("\n遍历字符串中的字符:");
    for c in "Hello".chars() {
        println!("字符: {}", c);
    }
    
    // 运行结果：
    // 遍历范围 1 到 5:
    // 数字: 1
    // 数字: 2
    // 数字: 3
    // 数字: 4
    // 数字: 5
    // 
    // 遍历水果数组:
    // 水果: 苹果
    // 水果: 香蕉
    // 水果: 橙子
    // 水果: 葡萄
    // 
    // 遍历带索引的水果数组:
    // 水果[0]: 苹果
    // 水果[1]: 香蕉
    // 水果[2]: 橙子
    // 水果[3]: 葡萄
    // 
    // 遍历字符串中的字符:
    // 字符: H
    // 字符: e
    // 字符: l
    // 字符: l
    // 字符: o
}

// 演示 break 和 continue 关键字
// break 用于退出循环，continue 用于跳过当前迭代，进入下一次迭代
fn break_continue_keywords() {
    println!("\n--- break 和 continue 关键字 ---");
    
    // 使用 break 退出循环
    println!("寻找第一个大于 10 的数字:");
    for number in 1..20 {
        println!("检查: {}", number);
        if number > 10 {
            println!("找到大于 10 的数字: {}", number);
            break;
        }
    }
    
    // 使用 continue 跳过某些迭代
    println!("\n打印 1 到 10 之间的偶数:");
    for number in 1..11 {
        if number % 2 != 0 {
            continue; // 跳过奇数
        }
        println!("偶数: {}", number);
    }
    
    // 运行结果：
    // 寻找第一个大于 10 的数字:
    // 检查: 1
    // 检查: 2
    // 检查: 3
    // 检查: 4
    // 检查: 5
    // 检查: 6
    // 检查: 7
    // 检查: 8
    // 检查: 9
    // 检查: 10
    // 检查: 11
    // 找到大于 10 的数字: 11
    // 
    // 打印 1 到 10 之间的偶数:
    // 偶数: 2
    // 偶数: 4
    // 偶数: 6
    // 偶数: 8
    // 偶数: 10
}

// 演示 match 表达式
// match 表达式类似于 switch 语句，但更加强大
fn match_expressions() {
    println!("\n--- match 表达式 ---");
    
    // 基本的 match 表达式
    let number = 3;
    match number {
        1 => println!("一"),
        2 => println!("二"),
        3 => println!("三"),
        4 => println!("四"),
        5 => println!("五"),
        _ => println!("其他数字"), // _ 是通配符，匹配任何值
    }
    
    // match 作为表达式返回值
    let color = "red";
    let status = match color {
        "red" => "警告",
        "green" => "安全",
        "yellow" => "注意",
        _ => "未知颜色",
    };
    println!("颜色 {} 表示: {}", color, status);
    
    // 匹配范围
    let score = 85;
    match score {
        0..=59 => println!("不及格"),
        60..=79 => println!("及格"),
        80..=89 => println!("良好"),
        90..=100 => println!("优秀"),
        _ => println!("分数无效"),
    }
    
    // 运行结果：
    // 三
    // 颜色 red 表示: 警告
    // 良好
}

// 以下是示例中使用的辅助函数

// 一个简单的无参数、无返回值的函数
fn say_hello() {
    println!("Hello, Rust!");
}

// 带一个参数的函数
// 参数 name: &str - 要问候的人的名字
fn greet_person(name: &str) {
    println!("Hello, {}!", name);
}

// 带两个参数并返回结果的函数
// 参数 a: i32 - 第一个整数
// 参数 b: i32 - 第二个整数
// 返回值: i32 - 两个整数的和
fn calculate_sum(a: i32, b: i32) -> i32 {
    a + b // 隐式返回
}

// 带两个参数的乘法函数
// 参数 a: i32 - 第一个整数
// 参数 b: i32 - 第二个整数
// 返回值: i32 - 两个整数的乘积
fn multiply(a: i32, b: i32) -> i32 {
    a * b
}

// 带两个不同类型参数的函数
// 参数 n: i32 - 要描述的数字
// 参数 is_large: bool - 表示数字是否很大
// 返回值: String - 对数字的描述
fn describe_number(n: i32, is_large: bool) -> String {
    if is_large {
        format!("{} 是一个很大的数字", n)
    } else {
        format!("{} 是一个不大的数字", n)
    }
}

// 带可变引用参数的函数
// 参数 value: &mut i32 - 要递增的值的可变引用
// 参数 amount: i32 - 递增的数量
fn increment(value: &mut i32, amount: i32) {
    *value += amount; // 使用 * 解引用
}

// 计算一个数的平方
// 参数 x: i32 - 要计算平方的数
// 返回值: i32 - x 的平方
fn square(x: i32) -> i32 {
    x * x
}

// 找出两个数中的最大值
// 参数 a: i32 - 第一个数
// 参数 b: i32 - 第二个数
// 返回值: i32 - 较大的数
fn find_max(a: i32, b: i32) -> i32 {
    if a > b {
        return a; // 使用 return 关键字显式返回
    } else {
        return b;
    }
}

// 同时计算两个数的和与积
// 参数 a: i32 - 第一个数
// 参数 b: i32 - 第二个数
// 返回值: (i32, i32) - 元组，包含和与积
fn calculate_sum_and_product(a: i32, b: i32) -> (i32, i32) {
    (a + b, a * b) // 返回元组
}

// 知识点总结：
// 1. 函数定义：使用 fn 关键字，函数名使用 snake_case 命名规范
// 2. 函数参数：需要显式指定类型，多个参数用逗号分隔
// 3. 函数返回值：使用 -> 指定返回类型，可以是单个值或元组（多返回值）
// 4. 隐式返回：函数的最后一个表达式（不带分号）会被作为返回值
// 5. if/else 语句：是表达式，可以返回值，不需要括号包围条件
// 6. loop 循环：无限循环，可以使用 break 退出，也可以返回值
// 7. while 循环：条件为真时执行，类似于其他语言
// 8. for 循环：在迭代器上执行，是最常用的循环形式
// 9. break 关键字：用于退出循环
// 10. continue 关键字：用于跳过当前迭代，进入下一次迭代
// 11. match 表达式：模式匹配，类似于 switch 但更强大，可以作为表达式返回值