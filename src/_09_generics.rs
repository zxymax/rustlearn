// 第9课：泛型 (Generics)
// 本文件详细介绍 Rust 中的泛型机制
// 
// 知识点大纲：
// 1. 泛型的基本概念
// 2. 泛型函数
// 3. 泛型结构体
// 4. 泛型枚举
// 5. 泛型方法
// 6. 泛型约束
// 7. 多态性和泛型
// 8. 泛型的性能考量
// 9. 泛型与特征（Trait）
// 10. 泛型的高级用法

// 导入必要的模块
use std::fmt::Display;
use std::ops::{Add, Sub};

// 定义泛型版本的Container trait，避免编译错误
pub trait Container<T> {
    fn add(&mut self, item: T);
    fn get(&self, index: usize) -> Option<&T>;
}

// 定义一个公共函数 run()，作为本模块的入口点
// 当用户在主程序中选择 9 时，将调用此函数
pub fn run() {
    println!("=== 第9课：泛型 ===");
    println!("本示例将介绍 Rust 中的泛型机制。\n");
    
    // 调用各个示例函数
    generics_basics();
    generic_functions();
    generic_structs();
    generic_enums();
    generic_methods();
    generic_constraints();
    polymorphism_with_generics();
    performance_considerations();
    generics_with_traits();
    advanced_generics();
}

// 演示泛型的基本概念
fn generics_basics() {
    println!("\n--- 泛型的基本概念 ---");
    
    println!("泛型是一种编程概念，允许我们编写可以处理不同类型数据的代码：");
    println!("- 在 Rust 中，泛型使用尖括号 <T> 表示");
    println!("- T 是一个类型参数，可以是任何类型");
    println!("- 泛型让我们可以编写更通用、可重用的代码");
    println!("- 泛型在编译时会被单态化（monomorphization），不会有运行时开销");
    
    println!("\n泛型的使用场景：");
    println!("1. 集合类型（如 Vec<T>, HashMap<K, V>）");
    println!("2. 函数和方法需要处理多种类型的数据");
    println!("3. 结构体和枚举需要存储不同类型的数据");
    println!("4. 实现多态性行为");
    
    // 一个简单的泛型示例
    let numbers = vec![1, 2, 3, 4, 5]; // Vec<i32>
    let words = vec!["hello", "world"]; // Vec<&str>
    // let mixed = vec![1, "two", 3.0]; // 这在 Rust 中是不可能的，因为 Vec 只能存储同一种类型
    
    println!("numbers 是一个 Vec<i32> 类型: {:?}", numbers);
    println!("words 是一个 Vec<&str> 类型: {:?}", words);
    
    // 运行结果：
    // 泛型的基本概念
    // 泛型是一种编程概念，允许我们编写可以处理不同类型数据的代码：
    // - 在 Rust 中，泛型使用尖括号 <T> 表示
    // - T 是一个类型参数，可以是任何类型
    // - 泛型让我们可以编写更通用、可重用的代码
    // - 泛型在编译时会被单态化（monomorphization），不会有运行时开销
    // 
    // 泛型的使用场景：
    // 1. 集合类型（如 Vec<T>, HashMap<K, V>）
    // 2. 函数和方法需要处理多种类型的数据
    // 3. 结构体和枚举需要存储不同类型的数据
    // 4. 实现多态性行为
    // numbers 是一个 Vec<i32> 类型: [1, 2, 3, 4, 5]
    // words 是一个 Vec<&str> 类型: ["hello", "world"]
}

// 演示泛型函数
fn generic_functions() {
    println!("\n--- 泛型函数 ---");
    
    println!("泛型函数是可以接受不同类型参数的函数：");
    
    // 定义一个泛型函数，用于查找数组中的最大值
    fn find_max<T: PartialOrd>(list: &[T]) -> Option<&T> {
        if list.is_empty() {
            return None;
        }
        
        let mut max = &list[0];
        
        for item in list.iter().skip(1) {
            if item > max {
                max = item;
            }
        }
        
        Some(max)
    }
    
    // 定义一个简单的泛型函数，用于打印任何实现了 Display trait 的值
    fn print_value<T: Display>(value: T) {
        println!("Value: {}", value);
    }
    
    // 测试泛型函数
    let numbers = [1, 5, 3, 9, 2];
    if let Some(max) = find_max(&numbers) {
        println!("数组中的最大值: {}", max);
    }
    
    let floats = [1.5, 5.2, 3.7, 9.1, 2.8];
    if let Some(max) = find_max(&floats) {
        println!("浮点数数组中的最大值: {}", max);
    }
    
    let strings = ["apple", "banana", "orange", "pear"];
    if let Some(max) = find_max(&strings) {
        println!("字符串数组中的最大值: {}", max);
    }
    
    // 测试 print_value 函数
    print_value(42);
    print_value(3.14);
    print_value("Hello, Rust!");
    
    // 运行结果：
    // 泛型函数
    // 泛型函数是可以接受不同类型参数的函数：
    // 数组中的最大值: 9
    // 浮点数数组中的最大值: 9.1
    // 字符串数组中的最大值: pear
    // Value: 42
    // Value: 3.14
    // Value: Hello, Rust!
}

// 演示泛型结构体
fn generic_structs() {
    println!("\n--- 泛型结构体 ---");
    
    println!("泛型结构体是可以包含不同类型字段的结构体：");
    
    // 定义一个泛型结构体 Point，表示二维坐标点
    struct Point<T> {
        x: T,
        y: T,
    }
    
    // 为 Point 实现方法
    impl<T> Point<T> {
        fn new(x: T, y: T) -> Self {
            Point {
                x,
                y,
            }
        }
        
        fn get_x(&self) -> &T {
            &self.x
        }
        
        fn get_y(&self) -> &T {
            &self.y
        }
    }
    
    // 为特定类型的 Point 实现方法
    impl Point<i32> {
        fn distance_from_origin(&self) -> f64 {
            ((self.x.pow(2) + self.y.pow(2)) as f64).sqrt()
        }
    }
    
    // 定义一个具有多个泛型参数的结构体
    struct Pair<K, V> {
        key: K,
        value: V,
    }
    
    // 为 Pair 实现方法
    impl<K, V> Pair<K, V> {
        fn new(key: K, value: V) -> Self {
            Pair {
                key,
                value,
            }
        }
        
        fn get(&self) -> (&K, &V) {
            (&self.key, &self.value)
        }
    }
    
    // 测试泛型结构体
    let integer_point = Point::new(10, 20);
    println!("整数坐标点: ({}, {})", integer_point.get_x(), integer_point.get_y());
    println!("到原点的距离: {}", integer_point.distance_from_origin());
    
    let float_point = Point::new(1.5, 2.5);
    println!("浮点坐标点: ({}, {})", float_point.get_x(), float_point.get_y());
    // 以下代码会导致编译错误，因为 Point<f64> 没有实现 distance_from_origin 方法
    // println!("到原点的距离: {}", float_point.distance_from_origin());
    
    let pair1 = Pair::new("name", "Alice");
    let (key1, value1) = pair1.get();
    println!("键值对1: {} = {}", key1, value1);
    
    let pair2 = Pair::new(1, 100);
    let (key2, value2) = pair2.get();
    println!("键值对2: {} = {}", key2, value2);
    
    // 运行结果：
    // 泛型结构体
    // 泛型结构体是可以包含不同类型字段的结构体：
    // 整数坐标点: (10, 20)
    // 到原点的距离: 22.360679774997898
    // 浮点坐标点: (1.5, 2.5)
    // 键值对1: name = Alice
    // 键值对2: 1 = 100
}

// 演示泛型枚举
fn generic_enums() {
    println!("\n--- 泛型枚举 ---");
    
    println!("泛型枚举是可以包含不同类型关联数据的枚举：");
    
    // 定义一个简单的泛型枚举
    enum MyOption<T> {
        Some(T),
        None,
    }
    
    // 注意：Rust 标准库已经定义了 Option 枚举，这里只是为了演示
    
    // 定义一个包含多个泛型参数的枚举
    enum Result<T, E> {
        Ok(T),
        Err(E),
    }
    
    // 注意：Rust 标准库已经定义了 Result 枚举，这里只是为了演示
    
    // 定义一个自定义泛型枚举
    enum Message<T, U> {
        Text(String),
        Number(T),
        Pair(T, U),
        Empty,
    }
    
    // 为 Message 实现方法
    impl<T: std::fmt::Debug, U: std::fmt::Debug> Message<T, U> {
        fn print(&self) {
            match self {
                Message::Text(s) => println!("Text: {}", s),
                Message::Number(n) => println!("Number: {:?}", n),
                Message::Pair(a, b) => println!("Pair: {:?}, {:?}", a, b),
                Message::Empty => println!("Empty message"),
            }
        }
    }
    
    // 测试泛型枚举
    let msg1: Message<i32, i32> = Message::Text(String::from("Hello"));
    msg1.print();
    
    let msg2: Message<i32, i32> = Message::Number(42);
    msg2.print();
    
    let msg3: Message<&str, i32> = Message::Pair("x", 100);
    msg3.print();
    
    let msg4: Message<i32, i32> = Message::Empty;
    msg4.print();
    
    // 使用标准库中的 Option 枚举
    let some_value = Some(5);
    let none_value: Option<i32> = None;
    
    println!("Option 值: {:?}, {:?}", some_value, none_value);
    
    // 运行结果：
    // 泛型枚举
    // 泛型枚举是可以包含不同类型关联数据的枚举：
    // Text: Hello
    // Number: 42
    // Pair: "x", 100
    // Empty message
    // Option 值: Some(5), None
}

// 演示泛型方法
fn generic_methods() {
    println!("\n--- 泛型方法 ---");
    
    println!("泛型方法是在结构体或枚举上定义的可以处理不同类型数据的方法：");
    
    // 定义一个泛型结构体
    struct Container<T> {
        value: T,
    }
    
    // 为 Container 实现泛型方法
    impl<T> Container<T> {
        fn new(value: T) -> Self {
            Container {
                value,
            }
        }
        
        fn get(&self) -> &T {
            &self.value
        }
        
        // 泛型方法，接受一个不同类型的参数
        fn map<U, F>(self, f: F) -> Container<U>
        where
            F: FnOnce(T) -> U,
        {
            Container {
                value: f(self.value),
            }
        }
    }
    
    // 为特定类型的 Container 实现方法
    impl Container<i32> {
        fn increment(&mut self) {
            self.value += 1;
        }
        
        fn square(&self) -> Container<i64> {
            Container {
                value: (self.value as i64).pow(2),
            }
        }
    }
    
    // 测试泛型方法
    let container1 = Container::new(42);
    println!("Container 1 的值: {}", container1.get());
    
    let mut container2 = Container::new(10);
    container2.increment();
    println!("Container 2 递增后的值: {}", container2.get());
    
    let squared = container2.square();
    println!("Container 2 的平方: {}", squared.get());
    
    // 使用 map 方法转换类型
    let container3 = Container::new(5);
    let mapped = container3.map(|x| x.to_string());
    println!("转换为字符串后的值: {}", mapped.get());
    
    let container4 = Container::new("hello");
    let mapped2 = container4.map(|s| s.len());
    println!("字符串长度: {}", mapped2.get());
    
    // 运行结果：
    // 泛型方法
    // 泛型方法是在结构体或枚举上定义的可以处理不同类型数据的方法：
    // Container 1 的值: 42
    // Container 2 递增后的值: 11
    // Container 2 的平方: 121
    // 转换为字符串后的值: 5
    // 字符串长度: 5
}

// 演示泛型约束
fn generic_constraints() {
    println!("\n--- 泛型约束 ---");
    
    println!("泛型约束用于限制泛型参数可以接受的类型：");
    println!("- 使用 where 子句或直接在尖括号中指定约束");
    println!("- 常见的约束包括：Trait 约束、生命周期约束等");
    
    // 定义一个带有 Trait 约束的泛型函数
    fn add<T: Add<Output = T>>(a: T, b: T) -> T {
        a + b
    }
    
    // 使用 where 子句定义约束
    fn subtract<T>(a: T, b: T) -> T
    where
        T: Sub<Output = T>,
    {
        a - b
    }
    
    // 多个约束
    fn display_and_add<T>(a: T, b: T)
    where
        T: Add<Output = T> + Display + Clone,
    {
        let result = a.clone() + b.clone();
        println!("{} + {} = {}", a, b, result);
    }
    
    // 测试带有约束的泛型函数
    println!("1 + 2 = {}", add(1, 2));
    println!("3.5 + 2.5 = {}", add(3.5, 2.5));
    
    println!("5 - 3 = {}", subtract(5, 3));
    println!("10.5 - 4.2 = {}", subtract(10.5, 4.2));
    
    display_and_add(10, 20);
    display_and_add(5.5, 4.5);
    
    // 以下代码会导致编译错误，因为字符串切片没有实现 Add 操作符
    // display_and_add("hello", "world");
    
    // 修改Calculator结构体，添加Copy trait约束
    struct Calculator<T: Add<Output = T> + Sub<Output = T> + Copy> {
        value: T,
    }
    
    impl<T: Add<Output = T> + Sub<Output = T> + Copy> Calculator<T> {
        fn new(value: T) -> Self {
            Calculator { value }
        }
        
        fn add(&mut self, other: T) {
            self.value = self.value + other;
        }
        
        fn subtract(&mut self, other: T) {
            self.value = self.value - other;
        }
        
        fn get(&self) -> T {
            self.value
        }
    }
    
    // 修复Container trait的get方法实现，避免递归调用
    impl<T> Container<T> for Vec<T> {
        fn add(&mut self, item: T) {
            self.push(item);
        }
        
        fn get(&self, index: usize) -> Option<&T> {
            <[T]>::get(self.as_slice(), index)
        }
    }
    
    let mut calc = Calculator::new(100);
    println!("初始值: {}", calc.get());
    calc.add(50);
    println!("加 50 后: {}", calc.get());
    calc.subtract(25);
    println!("减 25 后: {}", calc.get());
    
    let mut float_calc = Calculator::new(10.5);
    println!("初始浮点值: {}", float_calc.get());
    float_calc.add(5.5);
    println!("加 5.5 后: {}", float_calc.get());
    
    // 运行结果：
    // 泛型约束
    // 泛型约束用于限制泛型参数可以接受的类型：
    // - 使用 where 子句或直接在尖括号中指定约束
    // - 常见的约束包括：Trait 约束、生命周期约束等
    // 1 + 2 = 3
    // 3.5 + 2.5 = 6
    // 5 - 3 = 2
    // 10.5 - 4.2 = 6.3
    // 10 + 20 = 30
    // 5.5 + 4.5 = 10
    // 初始值: 100
    // 加 50 后: 150
    // 减 25 后: 125
    // 初始浮点值: 10.5
    // 加 5.5 后: 16
}

// 演示多态性和泛型
fn polymorphism_with_generics() {
    println!("\n--- 多态性和泛型 ---");
    
    println!("泛型允许我们实现编译时多态性：");
    println!("- 相同的代码可以处理不同类型的数据");
    println!("- 编译器会为每种具体类型生成专门的代码");
    
    // 定义一个特征（Trait）
    trait Draw {
        fn draw(&self);
    }
    
    // 实现该特征的具体类型
    struct Circle {
        radius: f64,
    }
    
    impl Draw for Circle {
        fn draw(&self) {
            println!("绘制一个半径为 {} 的圆形", self.radius);
        }
    }
    
    struct Rectangle {
        width: f64,
        height: f64,
    }
    
    impl Draw for Rectangle {
        fn draw(&self) {
            println!("绘制一个 {}x{} 的矩形", self.width, self.height);
        }
    }
    
    struct Triangle {
        base: f64,
        height: f64,
    }
    
    impl Draw for Triangle {
        fn draw(&self) {
            println!("绘制一个底为 {}，高为 {} 的三角形", self.base, self.height);
        }
    }
    
    // 使用泛型实现多态
    fn draw_shape<T: Draw>(shape: T) {
        shape.draw();
    }
    
    // 使用特征对象实现运行时多态
    fn draw_shape_dyn(shape: &dyn Draw) {
        shape.draw();
    }
    
    // 测试多态性
    let circle = Circle { radius: 5.0 };
    let rectangle = Rectangle { width: 10.0, height: 5.0 };
    let triangle = Triangle { base: 6.0, height: 8.0 };
    
    println!("使用泛型函数：");
    draw_shape(circle);
    draw_shape(rectangle);
    draw_shape(triangle);
    
    // 由于上面的调用已经消耗了对象，我们需要重新创建
    let circle = Circle { radius: 5.0 };
    let rectangle = Rectangle { width: 10.0, height: 5.0 };
    let triangle = Triangle { base: 6.0, height: 8.0 };
    
    println!("\n使用特征对象：");
    draw_shape_dyn(&circle);
    draw_shape_dyn(&rectangle);
    draw_shape_dyn(&triangle);
    
    // 创建一个特征对象的集合
    let shapes: Vec<Box<dyn Draw>> = vec![
        Box::new(Circle { radius: 3.0 }),
        Box::new(Rectangle { width: 4.0, height: 6.0 }),
        Box::new(Triangle { base: 5.0, height: 7.0 }),
    ];
    
    println!("\n遍历特征对象集合：");
    for shape in shapes {
        shape.draw();
    }
    
    // 运行结果：
    // 多态性和泛型
    // 泛型允许我们实现编译时多态性：
    // - 相同的代码可以处理不同类型的数据
    // - 编译器会为每种具体类型生成专门的代码
    // 使用泛型函数：
    // 绘制一个半径为 5 的圆形
    // 绘制一个 10x5 的矩形
    // 绘制一个底为 6，高为 8 的三角形
    // 
    // 使用特征对象：
    // 绘制一个半径为 5 的圆形
    // 绘制一个 10x5 的矩形
    // 绘制一个底为 6，高为 8 的三角形
    // 
    // 遍历特征对象集合：
    // 绘制一个半径为 3 的圆形
    // 绘制一个 4x6 的矩形
    // 绘制一个底为 5，高为 7 的三角形
}

// 定义Container trait，避免Vec<T>实现时找不到trait
// 演示泛型的高级用法
fn advanced_generics() {
    println!("\n--- 泛型的高级用法 ---");
    
    println!("Rust 中的泛型还有一些高级用法：");
    
    // 1. 关联类型（Associated Types）
    trait Container {
        type Item;
        
        fn add(&mut self, item: Self::Item);
        fn get(&self, index: usize) -> Option<&Self::Item>;
    }
    
    impl<T> Container for Vec<T> {
        type Item = T;
        
        fn add(&mut self, item: T) {
            self.push(item);
        }
        
        fn get(&self, index: usize) -> Option<&T> {
            <[T]>::get(self.as_slice(), index)
        }
    }
    
    // 2. 默认泛型参数
    trait PrintWithPrefix {
        fn print_with_prefix(&self, prefix: &str);
    }
    
    impl<T: Display> PrintWithPrefix for T {
        fn print_with_prefix(&self, prefix: &str) {
            println!("{}, value: {}", prefix, self);
        }
    }
    
    // 3. 完全限定语法
    trait Animal {
        fn name() -> &'static str;
    }
    
    struct Dog;
    struct Cat;
    
    impl Animal for Dog {
        fn name() -> &'static str {
            "Dog"
        }
    }
    
    impl Animal for Cat {
        fn name() -> &'static str {
            "Cat"
        }
    }
    
    // 4. 泛型在闭包中的使用
    fn apply_function<T, R, F>(value: T, f: F) -> R
    where
        F: FnOnce(T) -> R,
    {
        f(value)
    }
    
    // 测试高级泛型用法
    let mut vec = Vec::new();
    vec.push(10);
    vec.push(20);
    vec.push(30);
    
    if let Some(value) = <[i32]>::get(vec.as_slice(), 1) {
        println!("Vec 中的第二个元素: {}", value);
    }
    
    let number = 42;
    number.print_with_prefix("数字");
    
    let text = "Hello, Rust!";
    text.print_with_prefix("文本");
    
    // 使用完全限定语法调用关联函数
    println!("Dog 的名称: {}", <Dog as Animal>::name());
    println!("Cat 的名称: {}", <Cat as Animal>::name());
    
    // 测试闭包的泛型使用
    let doubled = apply_function(5, |x| x * 2);
    println!("5 的两倍: {}", doubled);
    
    let squared = apply_function(5, |x| x * x);
    println!("5 的平方: {}", squared);
    
    let result = apply_function("hello", |s| s.len());
    println!("字符串 'hello' 的长度: {}", result);
    
    // 运行结果：
    // 泛型的高级用法
    // Rust 中的泛型还有一些高级用法：
    // Vec 中的第二个元素: 20
    // 数字, value: 42
    // 文本, value: Hello, Rust!
    // Dog 的名称: Dog
    // Cat 的名称: Cat
    // 5 的两倍: 10
    // 5 的平方: 25
    // 字符串 'hello' 的长度: 5
}

// 演示泛型与特征（Trait）的结合使用
fn generics_with_traits() {
    println!("\n--- 泛型与特征（Trait）的结合使用 ---");
    
    println!("泛型与特征（Trait）的结合使用是 Rust 类型系统的重要特性：");
    
    // 定义一个特征
    trait Summary {
        fn summarize(&self) -> String;
    }
    
    // 定义一些实现该特征的类型
    struct NewsArticle {
        headline: String,
        location: String,
        author: String,
        content: String,
    }
    
    impl Summary for NewsArticle {
        fn summarize(&self) -> String {
            format!("{}, by {} ({})", self.headline, self.author, self.location)
        }
    }
    
    // 为NewsArticle实现Display trait
    impl std::fmt::Display for NewsArticle {
        fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
            write!(f, "{}", self.summarize())
        }
    }
    
    struct Tweet {
        username: String,
        content: String,
        reply: bool,
        retweet: bool,
    }
    
    impl Summary for Tweet {
        fn summarize(&self) -> String {
            format!("{}: {}", self.username, self.content)
        }
    }
    
    // 定义一个泛型函数，接受任何实现了 Summary trait 的类型
    fn notify<T: Summary>(item: T) {
        println!("Breaking news! {}", item.summarize());
    }
    
    // 使用 where 子句的等价写法
    fn notify_where<T>(item: T)
    where
        T: Summary,
    {
        println!("Breaking news! {}", item.summarize());
    }
    
    // 多个 trait 约束
    fn display_and_summarize<T>(item: T)
    where
        T: Summary + Display,
    {
        println!("Display: {}", item);
        println!("Summary: {}", item.summarize());
    }
    
    // 返回实现了特定 trait 的类型
    fn returns_summarizable() -> impl Summary {
        Tweet {
            username: String::from("horse_ebooks"),
            content: String::from("of course, as you probably already know, people"),
            reply: false,
            retweet: false,
        }
    }
    
    // 测试泛型与特征的结合使用
    let article = NewsArticle {
        headline: String::from("Penguins win the Stanley Cup Championship"),
        location: String::from("Pittsburgh, PA, USA"),
        author: String::from("Iceburgh"),
        content: String::from("The Pittsburgh Penguins once again are the best hockey team in the NHL."),
    };
    
    let tweet = Tweet {
        username: String::from("horse_ebooks"),
        content: String::from("of course, as you probably already know, people"),
        reply: false,
        retweet: false,
    };
    
    println!("Article summary: {}", article.summarize());
    println!("Tweet summary: {}", tweet.summarize());
    
    notify(article);
    notify_where(tweet);
    
    // 重新创建实例，因为上面的调用已经消耗了对象
    let article = NewsArticle {
        headline: String::from("Penguins win again"),
        location: String::from("Pittsburgh"),
        author: String::from("Iceburgh"),
        content: String::from("Another championship for Pittsburgh!"),
    };
    
    display_and_summarize(article);
    
    let summarizable = returns_summarizable();
    println!("Returned summary: {}", summarizable.summarize());
    
    // 运行结果：
    // 泛型与特征（Trait）的结合使用
    // 泛型与特征（Trait）的结合使用是 Rust 类型系统的重要特性：
    // Article summary: Penguins win the Stanley Cup Championship, by Iceburgh (Pittsburgh, PA, USA)
    // Tweet summary: horse_ebooks: of course, as you probably already know, people
    // Breaking news! Penguins win the Stanley Cup Championship, by Iceburgh (Pittsburgh, PA, USA)
    // Breaking news! horse_ebooks: of course, as you probably already know, people
    // Display: NewsArticle { headline: "Penguins win again", location: "Pittsburgh", author: "Iceburgh", content: "Another championship for Pittsburgh!" }
    // Summary: Penguins win again, by Iceburgh (Pittsburgh)
    // Returned summary: horse_ebooks: of course, as you probably already know, people
}

// 演示泛型的性能考量
fn performance_considerations() {
    println!("\n--- 泛型的性能考量 ---");
    
    println!("Rust 中的泛型在性能方面有几个重要特点：");
    println!("1. 单态化（monomorphization）：编译器为每种使用的具体类型生成专用的代码");
    println!("2. 零运行时开销：泛型不会引入额外的运行时开销");
    println!("3. 静态分发：使用泛型的函数调用在编译时确定，与具体类型直接调用一样高效");
    println!("4. 类型擦除 vs 单态化：与某些语言的类型擦除不同，Rust 的单态化确保了最佳性能");
    
    // 定义一个简单的泛型函数
    fn multiply<T: std::ops::Mul<Output = T>>(a: T, b: T) -> T {
        a * b
    }
    
    // 测试性能考核（实际性能测试通常需要使用基准测试框架）
    let start = std::time::Instant::now();
    
    // 执行一些计算
    let result = multiply(10, 20);
    println!("整数乘法结果: {}", result);
    
    let result = multiply(10.5, 20.5);
    println!("浮点数乘法结果: {}", result);
    
    let duration = start.elapsed();
    println!("计算耗时: {:?}", duration);
    
    // 运行结果：
    // 泛型的性能考核
    // Rust 中的泛型在性能方面有几个重要特点：
    // 1. 单态化（monomorphization）：编译器为每种使用的具体类型生成专用的代码
    // 2. 零运行时开销：泛型不会引入额外的运行时开销
    // 3. 静态分发：使用泛型的函数调用在编译时确定，与具体类型直接调用一样高效
    // 4. 类型擦除 vs 单态化：与某些语言的类型擦除不同，Rust 的单态化确保了最佳性能
    // 整数乘法结果: 200
    // 浮点数乘法结果: 215.25
    // 计算耗时: [具体时间，根据运行环境而定]
}