// 第10课：生命周期 (Lifetimes)
// 本文件详细介绍 Rust 中的生命周期机制
// 
// 知识点大纲：
// 1. 生命周期的基本概念
// 2. 生命周期注解语法
// 3. 函数签名中的生命周期
// 4. 结构体中的生命周期
// 5. 方法定义中的生命周期
// 6. 生命周期省略规则
// 7. 静态生命周期
// 8. 生命周期约束
// 9. 生命周期子类型化
// 10. 高级生命周期用法

// 导入必要的模块
use std::fmt::Display;

// 定义一个公共函数 run()，作为本模块的入口点
// 当用户在主程序中选择 10 时，将调用此函数
pub fn run() {
    println!("=== 第10课：生命周期 ===");
    println!("本示例将介绍 Rust 中的生命周期机制。\n");
    
    // 调用各个示例函数
    lifetimes_basics();
    lifetime_annotations();
    function_signatures();
    struct_lifetimes();
    method_lifetimes();
    lifetime_elision();
    static_lifetimes();
    lifetime_bounds();
    lifetime_subtyping();
    advanced_lifetimes();
}

// 演示生命周期的基本概念
fn lifetimes_basics() {
    println!("\n--- 生命周期的基本概念 ---");
    
    println!("生命周期是 Rust 中的一个关键概念，用于确保引用的有效性：");
    println!("- 生命周期是对引用有效的时间段的抽象");
    println!("- 它帮助编译器在编译时确保所有引用都是有效的");
    println!("- 生命周期解决了悬垂引用（dangling references）的问题");
    println!("- 生命周期不改变任何引用或变量的存活时间");
    println!("- 它们只是被编译器用来验证引用的有效性");
    
    // 一个简单的生命周期示例
    let x = 5; // x 的生命周期开始
    let y = &x; // y 引用 x，y 的生命周期不能超过 x
    println!("x = {}, y = {}", x, y);
    // x 的生命周期结束，y 也不再有效
    
    // 错误示例（注释掉以避免编译错误）：
    // let r;
    // {
    //     let x = 5;
    //     r = &x; // 这里会导致编译错误，因为 x 的生命周期短于 r
    // }
    // println!("r = {}", r);
    
    println!("\n生命周期的主要用途：");
    println!("1. 确保引用在使用时不会指向已释放的内存");
    println!("2. 防止悬垂引用");
    println!("3. 帮助编译器进行借用检查");
    println!("4. 支持复杂的引用关系");
    
    // 运行结果：
    // 生命周期的基本概念
    // 生命周期是 Rust 中的一个关键概念，用于确保引用的有效性：
    // - 生命周期是对引用有效的时间段的抽象
    // - 它帮助编译器在编译时确保所有引用都是有效的
    // - 生命周期解决了悬垂引用（dangling references）的问题
    // - 生命周期不改变任何引用或变量的存活时间
    // - 它们只是被编译器用来验证引用的有效性
    // x = 5, y = 5
    // 
    // 生命周期的主要用途：
    // 1. 确保引用在使用时不会指向已释放的内存
    // 2. 防止悬垂引用
    // 3. 帮助编译器进行借用检查
    // 4. 支持复杂的引用关系
}

// 演示生命周期注解语法
fn lifetime_annotations() {
    println!("\n--- 生命周期注解语法 ---");
    
    println!("生命周期注解是描述引用生命周期关系的语法：");
    println!("- 使用撇号（'）后跟名称来表示生命周期参数，如 'a、'b、'c");
    println!("- 生命周期参数放在尖括号中，如 <'a>");
    println!("- 生命周期注解不会改变引用的实际生命周期");
    println!("- 它们只是告诉编译器多个引用之间的生命周期关系");
    
    // 生命周期注解示例
    let string1 = String::from("abcd");
    let string2 = "xyz";
    
    let result = longest(string1.as_str(), string2);
    println!("较长的字符串是: {}", result);
    
    // 定义一个带有生命周期注解的函数
    fn longest<'a>(x: &'a str, y: &'a str) -> &'a str {
        if x.len() > y.len() {
            x
        } else {
            y
        }
    }
    
    // 生命周期参数命名约定：
    // - 'a、'b、'c 等单字母名称用于通用生命周期
    // - 'static 表示整个程序执行期间的生命周期
    // - 更具描述性的名称（如 'context, 'iter）可以在复杂情况下使用
    
    println!("\n生命周期注解的位置：");
    println!("1. 函数参数：fn function<'a>(x: &'a Type)");
    println!("2. 函数返回值：fn function<'a>(x: &'a Type) -> &'a Type");
    println!("3. 结构体字段：struct Struct<'a> {{ field: &'a Type }}");
    println!("4. 泛型参数一起使用：fn function<'a, T>(x: &'a T)");
    
    // 运行结果：
    // 生命周期注解语法
    // 生命周期注解是描述引用生命周期关系的语法：
    // - 使用撇号（'）后跟名称来表示生命周期参数，如 'a、'b、'c
    // - 生命周期参数放在尖括号中，如 <'a>
    // - 生命周期注解不会改变引用的实际生命周期
    // - 它们只是告诉编译器多个引用之间的生命周期关系
    // 较长的字符串是: abcd
    // 
    // 生命周期注解的位置：
    // 1. 函数参数：fn function<'a>(x: &'a Type)
    // 2. 函数返回值：fn function<'a>(x: &'a Type) -> &'a Type
    // 3. 结构体字段：struct Struct<'a> { field: &'a Type }
    // 4. 泛型参数一起使用：fn function<'a, T>(x: &'a T)
}

// 演示函数签名中的生命周期
fn function_signatures() {
    println!("\n--- 函数签名中的生命周期 ---");
    
    println!("在函数签名中使用生命周期注解来表示参数和返回值之间的生命周期关系：");
    
    // 示例1：返回两个引用中存活时间较短的那个
    fn longest<'a>(x: &'a str, y: &'a str) -> &'a str {
        if x.len() > y.len() {
            x
        } else {
            y
        }
    }
    
    // 示例2：返回值的生命周期与其中一个参数的生命周期相同
    fn first_word<'a>(s: &'a str) -> &'a str {
        let bytes = s.as_bytes();
        
        for (i, &item) in bytes.iter().enumerate() {
            if item == b' ' {
                return &s[0..i];
            }
        }
        
        &s[..]
    }
    
    // 示例3：多个不同的生命周期
    fn mix_lifetimes<'a, 'b>(x: &'a str, y: &'b str) -> (&'a str, &'b str) {
        (x, y)
    }
    
    // 示例4：生命周期与泛型结合
    fn print_ref<'a, T: Display>(x: &'a T) {
        println!("{}", x);
    }
    
    // 测试函数
    let string1 = String::from("hello");
    let string2 = String::from("world");
    
    let result = longest(string1.as_str(), string2.as_str());
    println!("最长的字符串: {}", result);
    
    let sentence = String::from("hello world");
    let word = first_word(&sentence);
    println!("第一个单词: {}", word);
    
    let (ref1, ref2) = mix_lifetimes(&string1, &string2);
    println!("混合引用: {}, {}", ref1, ref2);
    
    print_ref(&42);
    print_ref(&3.14);
    
    // 运行结果：
    // 函数签名中的生命周期
    // 在函数签名中使用生命周期注解来表示参数和返回值之间的生命周期关系：
    // 最长的字符串: world
    // 第一个单词: hello
    // 混合引用: hello, world
    // 42
    // 3.14
}

// 演示结构体中的生命周期
fn struct_lifetimes() {
    println!("\n--- 结构体中的生命周期 ---");
    
    println!("当结构体包含引用时，必须为这些引用添加生命周期注解：");
    
    // 定义一个包含引用的结构体
    struct ImportantExcerpt<'a> {
        part: &'a str,
    }
    
    // 为结构体实现方法
    impl<'a> ImportantExcerpt<'a> {
        fn level(&self) -> i32 {
            3
        }
        
        fn announce_and_return_part(&self, announcement: &str) -> &str {
            println!("Attention please: {}", announcement);
            self.part
        }
    }
    
    // 定义一个具有多个生命周期参数的结构体
    struct MultiRef<'a, 'b> {
        first: &'a str,
        second: &'b str,
    }
    
    // 测试包含引用的结构体
    let novel = String::from("Call me Ishmael. Some years ago...");
    let first_sentence = novel.split('.').next().expect("Could not find a '.'");
    
    let i = ImportantExcerpt {
        part: first_sentence,
    };
    
    println!("重要摘录: {}", i.part);
    println!("摘录级别: {}", i.level());
    
    let announcement = "New chapter released!";
    let part = i.announce_and_return_part(announcement);
    println!("返回的部分: {}", part);
    
    // 测试多个生命周期参数的结构体
    let string1 = "hello";
    let string2 = String::from("world");
    
    let multi = MultiRef {
        first: string1,
        second: &string2,
    };
    
    println!("多引用结构体: {}, {}", multi.first, multi.second);
    
    // 运行结果：
    // 结构体中的生命周期
    // 当结构体包含引用时，必须为这些引用添加生命周期注解：
    // 重要摘录: Call me Ishmael
    // 摘录级别: 3
    // Attention please: New chapter released!
    // 返回的部分: Call me Ishmael
    // 多引用结构体: hello, world
}

// 演示方法定义中的生命周期
fn method_lifetimes() {
    println!("\n--- 方法定义中的生命周期 ---");
    
    println!("在结构体或枚举的方法中使用生命周期注解：");
    
    // 定义Descriptor trait（提前定义，以便在Book中使用）
    trait Descriptor {
        fn describe(&self) -> &str;
    }
    
    // 定义一个带有引用的结构体
    struct Book<'a> {
        title: &'a str,
        author: &'a str,
    }
    
    // 为结构体实现方法
    impl<'a> Book<'a> {
        // 生命周期标注
        fn get_title(&self) -> &'a str {
            self.title
        }
        
        // 组合两个Book的标题
        fn combine_titles<'b>(&'a self, other: &'b Book<'b>) -> String
        where
            'a: 'b,
        {
            format!("{} and {}", self.title, other.title)
        }
        
        // 比较标题
        fn compare_title(&self, other_title: &str) -> bool {
            self.title == other_title
        }
        
        // 获取较长的标题
        fn get_longer_title<'b>(&self, other_title: &'b str) -> &'b str
        where
            'a: 'b,
        {
            if self.title.len() > other_title.len() {
                self.title
            } else {
                other_title
            }
        }
    }
    
    // 为Book实现Descriptor trait
    impl<'a> Descriptor for Book<'a> {
        fn describe(&self) -> &str {
            self.title
        }
    }
    
    // 测试方法中的生命周期
    let title1 = "The Rust Programming Language";
    let author1 = "Steve Klabnik and Carol Nichols";
    
    let book1 = Book {
        title: title1,
        author: author1,
    };
    
    println!("书籍描述: {}", book1.describe());
    
    let other_title = "Programming Rust";
    let is_same = book1.compare_title(other_title);
    println!("标题相同? {}", is_same);
    
    let longer_title = book1.get_longer_title(other_title);
    println!("较长的标题: {}", longer_title);
    
    let title2 = "Effective Rust";
    let author2 = "Various Authors";
    
    let book2 = Book {
        title: title2,
        author: author2,
    };
    
    let combined = book1.combine_titles(&book2);
    println!("组合标题: {}", combined);
    
    // 运行结果：
    // 方法定义中的生命周期
    // 在结构体或枚举的方法中使用生命周期注解：
    // 书籍描述: The Rust Programming Language
    // 标题相同? false
    // 较长的标题: The Rust Programming Language
    // 组合标题: The Rust Programming Language and Effective Rust
}

// 演示生命周期省略规则
fn lifetime_elision() {
    println!("\n--- 生命周期省略规则 ---");
    
    println!("Rust 有一套生命周期省略规则，可以在某些情况下省略显式的生命周期注解：");
    println!("1. 每个引用参数获得自己的生命周期参数");
    println!("2. 如果只有一个输入生命周期参数，它被赋予所有输出生命周期参数");
    println!("3. 如果有多个输入生命周期参数，但其中一个是 &self 或 &mut self，");
    println!("   那么 self 的生命周期被赋予所有输出生命周期参数");
    
    // 示例：应用省略规则的函数
    
    // 原始版本（显式生命周期）
    // fn first_word<'a>(s: &'a str) -> &'a str { ... }
    
    // 省略版本
    fn first_word(s: &str) -> &str {
        let bytes = s.as_bytes();
        
        for (i, &item) in bytes.iter().enumerate() {
            if item == b' ' {
                return &s[0..i];
            }
        }
        
        &s[..]
    }
    
    // 另一个示例：方法中的省略
    struct Person {
        name: String,
    }
    
    impl Person {
        // 这里应用了省略规则 3
        fn get_name(&self) -> &str {
            &self.name
        }
    }
    
    // 无法省略的情况
    // fn longest(x: &str, y: &str) -> &str { ... }
    // 这个函数不能省略生命周期注解，因为它有两个输入生命周期参数
    // 且返回值的生命周期与哪一个相关不明确
    
    // 测试省略规则
    let sentence = String::from("hello world");
    let word = first_word(&sentence);
    println!("第一个单词: {}", word);
    
    let person = Person { name: String::from("Alice") };
    let name = person.get_name();
    println!("人名: {}", name);
    
    // 运行结果：
    // 生命周期省略规则
    // Rust 有一套生命周期省略规则，可以在某些情况下省略显式的生命周期注解：
    // 1. 每个引用参数获得自己的生命周期参数
    // 2. 如果只有一个输入生命周期参数，它被赋予所有输出生命周期参数
    // 3. 如果有多个输入生命周期参数，但其中一个是 &self 或 &mut self，
    //    那么 self 的生命周期被赋予所有输出生命周期参数
    // 第一个单词: hello
    // 人名: Alice
}

// 演示静态生命周期
fn static_lifetimes() {
    println!("\n--- 静态生命周期 ---");
    
    println!("'static 是一个特殊的生命周期，表示整个程序的执行期间：");
    println!("- 字符串字面量默认具有 'static 生命周期");
    println!("- 可以显式地将变量标记为 'static");
    println!("- 'static 生命周期的引用必须指向在程序整个生命周期内都有效的数据");
    
    // 字符串字面量默认具有 'static 生命周期
    let s: &'static str = "I have a static lifetime";
    println!("静态字符串: {}", s);
    
    // 显式声明静态变量
    static mut COUNTER: i32 = 0;
    
    // 在安全的 Rust 中访问静态可变变量需要 unsafe 块
    unsafe {
        COUNTER += 1;
        let counter_value = COUNTER;
        println!("计数器值: {}", counter_value);
    }
    
    // 函数返回 'static 生命周期的引用
    fn get_static_string() -> &'static str {
        "This is a static string"
    }
    
    // 创建一个具有 'static 生命周期的字符串
    fn create_static_string() -> &'static str {
        // 注意：这样做是错误的，因为 data 会在函数结束时被释放
        // let data = String::from("Not static");
        // &data
        
        // 正确的方法是使用 Box::leak
        let data = Box::new(String::from("Created as static"));
        Box::leak(data)
    }
    
    let static_str = get_static_string();
    println!("从函数获取的静态字符串: {}", static_str);
    
    let created_static = create_static_string();
    println!("创建的静态字符串: {}", created_static);
    
    // 运行结果：
    // 静态生命周期
    // 'static 是一个特殊的生命周期，表示整个程序的执行期间：
    // - 字符串字面量默认具有 'static 生命周期
    // - 可以显式地将变量标记为 'static
    // - 'static 生命周期的引用必须指向在程序整个生命周期内都有效的数据
    // 静态字符串: I have a static lifetime
    // 计数器值: 1
    // 从函数获取的静态字符串: This is a static string
    // 创建的静态字符串: Created as static
}

// 演示生命周期约束
fn lifetime_bounds() {
    println!("\n--- 生命周期约束 ---");
    
    println!("生命周期约束用于指定泛型类型参数与生命周期之间的关系：");
    
    // 定义一个带有生命周期约束的泛型函数
    fn print_longest<'a, T>(x: &'a T, y: &'a T)
    where
        T: Display + PartialOrd,
    {
        if x > y {
            println!("较长的值: {}", x);
        } else {
            println!("较长的值: {}", y);
        }
    }
    
    // 定义一个带有生命周期约束的结构体
    struct Container<'a, T: 'a> {
        item: &'a T,
    }
    
    // 定义一个带有生命周期和特征约束的函数
    fn process_and_print<'a, T>(item: &'a T)
    where
        T: Display + 'a,
    {
        println!("处理并打印: {}", item);
    }
    
    // 测试生命周期约束
    let num1 = 42;
    let num2 = 100;
    print_longest(&num1, &num2);
    
    let float1 = 3.14;
    let float2 = 2.71;
    print_longest(&float1, &float2);
    
    let string1 = String::from("hello");
    let string2 = String::from("world");
    print_longest(&string1, &string2);
    
    // 测试带有生命周期约束的结构体
    let value = 42;
    let container = Container { item: &value };
    println!("容器中的项目: {}", container.item);
    
    // 测试带有生命周期和特征约束的函数
    process_and_print(&num1);
    process_and_print(&string1);
    
    // 运行结果：
    // 生命周期约束
    // 生命周期约束用于指定泛型类型参数与生命周期之间的关系：
    // 较长的值: 100
    // 较长的值: 3.14
    // 较长的值: world
    // 容器中的项目: 42
    // 处理并打印: 42
    // 处理并打印: world
}

// 演示生命周期子类型化
fn lifetime_subtyping() {
    println!("\n--- 生命周期子类型化 ---");
    
    println!("生命周期子类型化允许我们表达一个生命周期比另一个生命周期长的关系：");
    println!("- 如果 'a 是 'b 的子类型，表示 'a 的生命周期至少与 'b 一样长");
    println!("- 记作 'a: 'b");
    
    // 定义一个使用生命周期子类型化的函数
    fn longer_lived<'a, 'b>(x: &'a i32, _: &'b i32) -> &'a i32
    where
        'a: 'b,
    {
        x
    }
    
    // 定义一个结构体，其中一个生命周期必须长于另一个
    struct RefPair<'a, 'b: 'a> {
        first: &'a i32,
        second: &'b i32,
    }
    
    // 测试生命周期子类型化
    let outer = 100;
    {
        let inner = 200;
        let result = longer_lived(&outer, &inner);
        println!("结果: {}", result);
        
        // 这里 outer 的生命周期比 inner 长，所以是有效的
        let pair = RefPair {
            first: &outer,
            second: &inner,
        };
        println!("RefPair: {}, {}", pair.first, pair.second);
    }
    
    // 生命周期子类型化在 trait 对象中的应用
    trait Descriptor {
        fn describe(&self) -> &str;
    }
    
    struct Description<'a> {
        text: &'a str,
    }
    
    impl<'a> Descriptor for Description<'a> {
        fn describe(&self) -> &str {
            self.text
        }
    }
    
    // 接受任何生命周期的 trait 对象
    fn print_description<'a>(desc: &'a dyn Descriptor) {
        println!("描述: {}", desc.describe());
    }
    
    let description = Description { text: "示例描述" };
    print_description(&description);
    
    // 运行结果：
    // 生命周期子类型化
    // 生命周期子类型化允许我们表达一个生命周期比另一个生命周期长的关系：
    // - 如果 'a 是 'b 的子类型，表示 'a 的生命周期至少与 'b 一样长
    // - 记作 'a: 'b
    // 结果: 100
    // RefPair: 100, 200
    // 描述: 示例描述
}

// 演示高级生命周期用法
fn advanced_lifetimes() {
    println!("\n--- 高级生命周期用法 ---");
    
    println!("Rust 中的一些高级生命周期用法：");
    
    // 1. 高阶函数中的生命周期
    fn apply_function<'a, F, R>(f: F) -> R
    where
        F: FnOnce() -> R,
    {
        f()
    }
    
    // 2. 嵌套引用中的生命周期
    fn nested_references<'a, 'b>(x: &'a &'b str) -> &'b str {
        *x
    }
    
    // 3. 生命周期与闭包结合
    fn with_lifetime<'a, T, F, R>(value: &'a T, f: F) -> R
    where
        F: FnOnce(&'a T) -> R,
    {
        f(value)
    }
    
    // 4. 协变和逆变
    // 注意：这是一个高级概念，这里只做简单演示
    trait Handler<'a> {
        fn handle(&self, data: &'a str);
    }
    
    struct SimpleHandler;
    
    impl<'a> Handler<'a> for SimpleHandler {
        fn handle(&self, data: &'a str) {
            println!("处理数据: {}", data);
        }
    }
    
    // 测试高级生命周期用法
    let string = String::from("hello");
    let result = apply_function(|| string.len());
    println!("字符串长度: {}", result);
    
    let outer = "outer";
    let inner = &outer;
    let nested = nested_references(&inner);
    println!("嵌套引用: {}", nested);
    
    let number = 42;
    let processed = with_lifetime(&number, |n| n * 2);
    println!("处理后的数字: {}", processed);
    
    let handler = SimpleHandler;
    let data = "test data";
    handler.handle(data);
    
    // 运行结果：
    // 高级生命周期用法
    // Rust 中的一些高级生命周期用法：
    // 字符串长度: 5
    // 嵌套引用: outer
    // 处理后的数字: 84
    // 处理数据: test data
}

// 知识点总结：
// 1. 生命周期的基本概念：生命周期是对引用有效的时间段的抽象，用于确保引用的有效性和防止悬垂引用。
// 2. 生命周期注解语法：使用撇号（'）后跟名称来表示生命周期参数，如 'a、'b、'c。
// 3. 函数签名中的生命周期：在函数签名中使用生命周期注解来表示参数和返回值之间的生命周期关系。
// 4. 结构体中的生命周期：当结构体包含引用时，必须为这些引用添加生命周期注解。
// 5. 方法定义中的生命周期：在结构体或枚举的方法中使用生命周期注解，表示 self、参数和返回值之间的生命周期关系。
// 6. 生命周期省略规则：Rust 有一套生命周期省略规则，可以在某些情况下省略显式的生命周期注解。
// 7. 静态生命周期：'static 是一个特殊的生命周期，表示整个程序的执行期间，字符串字面量默认具有 'static 生命周期。
// 8. 生命周期约束：用于指定泛型类型参数与生命周期之间的关系，如 T: 'a 表示 T 中的所有引用都必须至少与 'a 一样长。
// 9. 生命周期子类型化：允许表达一个生命周期比另一个生命周期长的关系，记作 'a: 'b。
// 10. 高级生命周期用法：包括高阶函数中的生命周期、嵌套引用中的生命周期、生命周期与闭包结合、协变和逆变等高级概念。