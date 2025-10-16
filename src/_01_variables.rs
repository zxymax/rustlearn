// 第1课：变量和数据类型 (Variables and Data Types)
// 本文件详细介绍 Rust 中的变量声明、数据类型和类型转换等基础知识
// 
// 知识点大纲：
// 1. 变量的可变性与不可变性
// 2. 变量声明和初始化
// 3. 基本数据类型：整数、浮点数、布尔值、字符
// 4. 类型标注
// 5. 类型转换
// 6. 常量和静态变量

// 导入标准输出模块

// 定义一个公共函数 run()，作为本模块的入口点
// 当用户在主程序中选择 1 时，将调用此函数
pub fn run() {
    println!("=== 第1课：变量和数据类型 ===");
    println!("本示例将介绍 Rust 中的变量声明、可变性、基本数据类型和类型转换。\n");
    
    // 调用各个示例函数
    variable_mutability();
    basic_data_types();
    type_annotations();
    type_conversions();
    constants_and_statics();
}

// 演示变量的可变性与不可变性
// 在 Rust 中，默认情况下变量是不可变的（immutable）
// 需要使用 mut 关键字来声明可变变量
fn variable_mutability() {
    println!("\n--- 变量的可变性与不可变性 ---");
    
    // 声明一个不可变的变量
    let x = 5;
    println!("不可变变量 x = {}", x);
    
    // 以下代码会导致编译错误，因为尝试修改不可变变量
    // x = 10; // 错误：不能对不可变变量 x 进行二次赋值
    
    // 声明一个可变的变量，使用 mut 关键字
    let mut y = 5;
    println!("可变变量 y = {}", y);
    
    // 可以修改可变变量的值
    y = 10;
    println!("修改后，可变变量 y = {}", y);
    
    // 变量遮蔽（Variable Shadowing）：使用相同的名称声明新变量
    let y = y + 5; // 这里的 y 是一个新的不可变变量，遮蔽了原来的可变变量 y
    println!("变量遮蔽后，y = {}", y);
    
    // 运行结果：
    // 不可变变量 x = 5
    // 可变变量 y = 5
    // 修改后，可变变量 y = 10
    // 变量遮蔽后，y = 15
}

// 演示 Rust 的基本数据类型
// Rust 提供了多种基本数据类型，包括整数、浮点数、布尔值和字符
fn basic_data_types() {
    println!("\n--- 基本数据类型 ---");
    
    // 整数类型
    // i 表示有符号整数，u 表示无符号整数，后面的数字表示位数
    let integer_i32: i32 = -42; // 32位有符号整数
    let integer_u32: u32 = 42; // 32位无符号整数
    let integer_i64: i64 = -10000000000; // 64位有符号整数
    let integer_u64: u64 = 10000000000; // 64位无符号整数
    let integer_isize: isize = -100; // 指针大小的有符号整数
    let integer_usize: usize = 100; // 指针大小的无符号整数，常用于索引
    
    println!("整数类型：");
    println!("i32: {}", integer_i32);
    println!("u32: {}", integer_u32);
    println!("i64: {}", integer_i64);
    println!("u64: {}", integer_u64);
    println!("isize: {}", integer_isize);
    println!("usize: {}", integer_usize);
    
    // 浮点数类型
    // f32 是单精度浮点数，f64 是双精度浮点数
    let float_f32: f32 = 3.14; // 单精度浮点数
    let float_f64: f64 = 3.14159265359; // 双精度浮点数（默认浮点类型）
    
    println!("\n浮点数类型：");
    println!("f32: {}", float_f32);
    println!("f64: {}", float_f64);
    
    // 布尔值类型
    let boolean_true: bool = true;
    let boolean_false: bool = false;
    
    println!("\n布尔值类型：");
    println!("true: {}", boolean_true);
    println!("false: {}", boolean_false);
    
    // 字符类型
    // Rust 的 char 类型表示 Unicode 标量值
    let char_a: char = 'a';
    let char_zh: char = '中';
    let char_emoji: char = '😊';
    
    println!("\n字符类型：");
    println!("'a': {}", char_a);
    println!("'中': {}", char_zh);
    println!("'😊': {}", char_emoji);
    
    // 运行结果：
    // 整数类型：
    // i32: -42
    // u32: 42
    // i64: -10000000000
    // u64: 10000000000
    // isize: -100
    // usize: 100
    // 
    // 浮点数类型：
    // f32: 3.14
    // f64: 3.14159265359
    // 
    // 布尔值类型：
    // true: true
    // false: false
    // 
    // 字符类型：
    // 'a': a
    // '中': 中
    // '😊': 😊
}

// 演示类型标注
// 在 Rust 中，编译器通常可以推断变量的类型，但有时需要显式标注
fn type_annotations() {
    println!("\n--- 类型标注 ---");
    
    // 编译器可以推断类型
    let inferred_integer = 42; // 编译器推断为 i32
    let inferred_float = 3.14; // 编译器推断为 f64
    let inferred_boolean = true; // 编译器推断为 bool
    
    println!("编译器推断的类型：");
    println!("inferred_integer = {}, 类型: i32", inferred_integer);
    println!("inferred_float = {}, 类型: f64", inferred_float);
    println!("inferred_boolean = {}, 类型: bool", inferred_boolean);
    
    // 显式类型标注
    let explicit_integer: i64 = 42; // 显式指定为 i64
    let explicit_float: f32 = 3.14; // 显式指定为 f32
    
    println!("\n显式标注的类型：");
    println!("explicit_integer = {}, 类型: i64", explicit_integer);
    println!("explicit_float = {}, 类型: f32", explicit_float);
    
    // 类型标注在函数参数和返回值中的应用
    println!("\n调用 add 函数：");
    let result = add(10, 20);
    println!("10 + 20 = {}", result);
    
    // 运行结果：
    // 编译器推断的类型：
    // inferred_integer = 42, 类型: i32
    // inferred_float = 3.14, 类型: f64
    // inferred_boolean = true, 类型: bool
    // 
    // 显式标注的类型：
    // explicit_integer = 42, 类型: i64
    // explicit_float = 3.14, 类型: f32
    // 
    // 调用 add 函数：
    // 10 + 20 = 30
}

// 演示类型转换
// Rust 不允许隐式类型转换，必须使用 as 关键字进行显式转换
fn type_conversions() {
    println!("\n--- 类型转换 ---");
    
    // 整数之间的转换
    let a: i32 = 100;
    let b: u32 = a as u32; // 将 i32 转换为 u32
    println!("i32 {} 转换为 u32: {}", a, b);
    
    // 整数转换为浮点数
    let c: i32 = 42;
    let d: f64 = c as f64; // 将 i32 转换为 f64
    println!("i32 {} 转换为 f64: {}", c, d);
    
    // 浮点数转换为整数（会截断小数部分）
    let e: f64 = 3.99;
    let f: i32 = e as i32; // 将 f64 转换为 i32
    println!("f64 {} 转换为 i32: {}", e, f);
    
    // 布尔值转换为整数
    let g: bool = true;
    let h: i32 = g as i32; // true 转换为 1
    let i: bool = false;
    let j: i32 = i as i32; // false 转换为 0
    println!("bool true 转换为 i32: {}", h);
    println!("bool false 转换为 i32: {}", j);
    
    // 字符转换为整数（获取 Unicode 码点）
    let k: char = 'A';
    let l: u32 = k as u32; // 获取 'A' 的 Unicode 码点
    println!("char 'A' 转换为 u32 (Unicode 码点): {}", l);
    
    // 运行结果：
    // i32 100 转换为 u32: 100
    // i32 42 转换为 f64: 42
    // f64 3.99 转换为 i32: 3
    // bool true 转换为 i32: 1
    // bool false 转换为 i32: 0
    // char 'A' 转换为 u32 (Unicode 码点): 65
}

// 演示常量和静态变量
// 常量（const）和静态变量（static）都是在编译时已知的值，但有一些重要区别
fn constants_and_statics() {
    println!("\n--- 常量和静态变量 ---");
    
    // 常量声明，使用 const 关键字
    // 常量的值必须在编译时确定
    const MAX_SCORE: u32 = 100;
    println!("常量 MAX_SCORE = {}", MAX_SCORE);
    
    // 常量可以在任何作用域中声明
    const PI: f64 = 3.14159;
    println!("常量 PI = {}", PI);
    
    // 静态变量声明，使用 static 关键字
    // 静态变量在程序的整个生命周期内都存在
    static mut COUNTER: u32 = 0;
    
    // 访问可变静态变量需要使用 unsafe 块
    unsafe {
        COUNTER += 1;
        let counter_value1 = COUNTER;
        println!("静态变量 COUNTER = {}", counter_value1);
        
        COUNTER += 1;
        let counter_value2 = COUNTER;
        println!("更新后，静态变量 COUNTER = {}", counter_value2);
    }
    
    // 常量和静态变量的区别：
    // 1. 常量在编译时内联到代码中，而静态变量有固定的内存地址
    // 2. 常量总是不可变的，而静态变量可以是可变的（但需要 unsafe 块访问）
    // 3. 常量使用 const 关键字，静态变量使用 static 关键字
    
    // 运行结果：
    // 常量 MAX_SCORE = 100
    // 常量 PI = 3.14159
    // 静态变量 COUNTER = 1
    // 更新后，静态变量 COUNTER = 2
}

// 一个简单的加法函数，用于演示类型标注
// 参数 a: i32 - 第一个整数
// 参数 b: i32 - 第二个整数
// 返回值: i32 - 两个整数的和
fn add(a: i32, b: i32) -> i32 {
    a + b // 隐式返回（没有分号）
}

// 知识点总结：
// 1. 变量的可变性：Rust 中默认变量是不可变的，使用 mut 关键字可以声明可变变量
// 2. 变量遮蔽：可以使用相同的名称声明新变量，新变量会遮蔽旧变量
// 3. 基本数据类型：整数、浮点数、布尔值和字符
// 4. 整数类型：i8, i16, i32, i64, isize（有符号）和 u8, u16, u32, u64, usize（无符号）
// 5. 浮点数类型：f32（单精度）和 f64（双精度，默认）
// 6. 字符类型：表示 Unicode 标量值，可以存储中文、表情符号等
// 7. 类型标注：使用 `: 类型` 语法显式指定变量类型
// 8. 类型转换：Rust 不允许隐式类型转换，必须使用 as 关键字进行显式转换
// 9. 常量：使用 const 关键字声明，值在编译时确定
// 10. 静态变量：使用 static 关键字声明，在程序的整个生命周期内存在