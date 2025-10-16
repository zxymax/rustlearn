// 第3课：结构体 (Structs)
// 本文件详细介绍 Rust 中的结构体定义、实例化、方法和关联函数等知识
// 
// 知识点大纲：
// 1. 结构体定义与实例化
// 2. 元组结构体
// 3. 单元结构体
// 4. 结构体方法
// 5. 关联函数
// 6. 结构体字段可见性
// 7. 结构体更新语法
// 8. 解构结构体

// 导入标准输出模块

// 定义一个公共函数 run()，作为本模块的入口点
// 当用户在主程序中选择 3 时，将调用此函数
pub fn run() {
    println!("=== 第3课：结构体 ===");
    println!("本示例将介绍 Rust 中的结构体定义、实例化、方法和关联函数等知识。\n");
    
    // 调用各个示例函数
    struct_definition();
    tuple_structs();
    unit_structs();
    struct_methods();
    associated_functions();
    struct_field_visibility();
    struct_update_syntax();
    destructuring_structs();
}

// 演示结构体的定义与实例化
// 结构体是一种自定义数据类型，允许我们组合多个相关的值
fn struct_definition() {
    println!("\n--- 结构体定义与实例化 ---");
    
    // 定义一个结构体（在函数内部定义结构体是允许的，但通常在模块级别定义）
    struct Person {
        name: String,
        age: u8,
        is_student: bool,
    }
    
    // 实例化结构体
    let person1 = Person {
        name: String::from("张三"),
        age: 25,
        is_student: true,
    };
    
    // 访问结构体字段
    println!("姓名: {}", person1.name);
    println!("年龄: {}", person1.age);
    println!("是否是学生: {}", person1.is_student);
    
    // 实例化可变结构体
    let mut person2 = Person {
        name: String::from("李四"),
        age: 30,
        is_student: false,
    };
    
    // 修改可变结构体的字段
    person2.age = 31;
    person2.is_student = true;
    
    println!("\n修改后的信息:");
    println!("姓名: {}", person2.name);
    println!("年龄: {}", person2.age);
    println!("是否是学生: {}", person2.is_student);
    
    // 运行结果：
    // 姓名: 张三
    // 年龄: 25
    // 是否是学生: true
    // 
    // 修改后的信息:
    // 姓名: 李四
    // 年龄: 31
    // 是否是学生: true
}

// 演示元组结构体
// 元组结构体是结构体的一种特殊形式，看起来像元组，但有名称
fn tuple_structs() {
    println!("\n--- 元组结构体 ---");
    
    // 定义元组结构体
    struct Point(i32, i32);
    struct Color(u8, u8, u8);
    
    // 实例化元组结构体
    let origin = Point(0, 0);
    let red = Color(255, 0, 0);
    let blue = Color(0, 0, 255);
    
    // 访问元组结构体的字段
    println!("原点坐标: ({}, {})", origin.0, origin.1);
    println!("红色 RGB 值: ({}, {}, {})", red.0, red.1, red.2);
    println!("蓝色 RGB 值: ({}, {}, {})", blue.0, blue.1, blue.2);
    
    // 元组结构体的可变性
    let mut point = Point(10, 20);
    point.0 = 15; // 修改第一个字段
    point.1 = 25; // 修改第二个字段
    println!("修改后的坐标: ({}, {})", point.0, point.1);
    
    // 运行结果：
    // 原点坐标: (0, 0)
    // 红色 RGB 值: (255, 0, 0)
    // 蓝色 RGB 值: (0, 0, 255)
    // 修改后的坐标: (15, 25)
}

// 演示单元结构体
// 单元结构体没有任何字段，类似于单元类型 ()
fn unit_structs() {
    println!("\n--- 单元结构体 ---");
    
    // 定义单元结构体
    struct Unit;
    
    // 实例化单元结构体
    let unit = Unit;
    
    // 单元结构体通常用于实现 traits 或作为标记
    // 这里我们简单地确认它被创建了
    println!("单元结构体已创建: Unit");
    
    // 运行结果：
    // 单元结构体已创建: Unit
}

// 定义一个全局结构体用于演示方法和关联函数
struct Rectangle {
    width: u32,
    height: u32,
}

// 为 Rectangle 实现方法
impl Rectangle {
    // 计算面积的方法
    // &self 表示方法接受 self 的不可变引用
    fn area(&self) -> u32 {
        self.width * self.height
    }
    
    // 检查是否可以容纳另一个矩形的方法
    fn can_hold(&self, other: &Rectangle) -> bool {
        self.width >= other.width && self.height >= other.height
    }
    
    // 修改矩形大小的方法
    // &mut self 表示方法接受 self 的可变引用
    fn resize(&mut self, new_width: u32, new_height: u32) {
        self.width = new_width;
        self.height = new_height;
    }
}

// 演示结构体方法
// 方法是与结构体关联的函数，使用 impl 块定义
fn struct_methods() {
    println!("\n--- 结构体方法 ---");
    
    // 创建 Rectangle 实例
    let rect1 = Rectangle {
        width: 30,
        height: 50,
    };
    
    // 调用方法
    println!("矩形面积: {}", rect1.area());
    
    // 创建另一个 Rectangle 实例
    let rect2 = Rectangle {
        width: 20,
        height: 40,
    };
    
    // 调用方法检查是否可以容纳另一个矩形
    println!("rect1 可以容纳 rect2: {}", rect1.can_hold(&rect2));
    
    // 创建可变的 Rectangle 实例
    let mut rect3 = Rectangle {
        width: 10,
        height: 20,
    };
    
    println!("修改前的面积: {}", rect3.area());
    
    // 调用修改方法
    rect3.resize(15, 25);
    
    println!("修改后的面积: {}", rect3.area());
    
    // 运行结果：
    // 矩形面积: 1500
    // rect1 可以容纳 rect2: true
    // 修改前的面积: 200
    // 修改后的面积: 375
}

// 为 Rectangle 实现关联函数
impl Rectangle {
    // 创建一个正方形的关联函数
    // 关联函数不接受 self 参数，使用结构体名称调用
    fn square(size: u32) -> Rectangle {
        Rectangle {
            width: size,
            height: size,
        }
    }
    
    // 创建一个默认矩形的关联函数
    fn default() -> Rectangle {
        Rectangle {
            width: 100,
            height: 100,
        }
    }
}

// 演示关联函数
// 关联函数是与结构体关联但不作用于特定实例的函数
fn associated_functions() {
    println!("\n--- 关联函数 ---");
    
    // 使用关联函数创建正方形
    let square = Rectangle::square(20);
    println!("正方形 - 宽: {}, 高: {}, 面积: {}", 
             square.width, square.height, square.area());
    
    // 使用关联函数创建默认矩形
    let default_rect = Rectangle::default();
    println!("默认矩形 - 宽: {}, 高: {}, 面积: {}", 
             default_rect.width, default_rect.height, default_rect.area());
    
    // 运行结果：
    // 正方形 - 宽: 20, 高: 20, 面积: 400
    // 默认矩形 - 宽: 100, 高: 100, 面积: 10000
}

// 演示结构体字段可见性
// 默认情况下，结构体字段是私有的，使用 pub 关键字可以使其变为公有的
// 注意：这个示例在模块级别才会有明显效果
fn struct_field_visibility() {
    println!("\n--- 结构体字段可见性 ---");
    
    // 在当前模块中，我们可以访问结构体的所有字段
    let rect = Rectangle {
        width: 50,
        height: 30,
    };
    
    println!("访问私有字段: 宽 = {}, 高 = {}", rect.width, rect.height);
    
    // 注意：在实际项目中，如果结构体定义在另一个模块中，
    // 默认情况下我们无法直接访问其私有字段
    // 需要使用 pub 关键字将字段声明为公有
    
    // 运行结果：
    // 访问私有字段: 宽 = 50, 高 = 30
}

// 定义一个结构体用于演示更新语法
struct User {
    username: String,
    email: String,
    sign_in_count: u64,
    active: bool,
}

// 演示结构体更新语法
// 结构体更新语法允许我们从另一个实例复制部分值
fn struct_update_syntax() {
    println!("\n--- 结构体更新语法 ---");
    
    // 创建一个 User 实例
    let user1 = User {
        username: String::from("alice"),
        email: String::from("alice@example.com"),
        sign_in_count: 1,
        active: true,
    };
    
    // 使用结构体更新语法从 user1 创建 user2
    // ..user1 表示复制 user1 中未显式设置的字段
    let user2 = User {
        email: String::from("bob@example.com"),
        username: String::from("bob"),
        ..user1
    };
    
    // 打印 user2 的信息
    println!("user2 用户名: {}", user2.username);
    println!("user2 邮箱: {}", user2.email);
    println!("user2 登录次数: {}", user2.sign_in_count);
    println!("user2 是否活跃: {}", user2.active);
    
    // 注意：当使用 .. 语法时，Rust 会尝试移动所有权
    // 所以在上面的例子中，user1 在创建 user2 后不能再被使用
    // 因为 String 类型不实现 Copy trait
    
    // 运行结果：
    // user2 用户名: bob
    // user2 邮箱: bob@example.com
    // user2 登录次数: 1
    // user2 是否活跃: true
}

// 演示解构结构体
// 解构允许我们将结构体的字段值绑定到变量
fn destructuring_structs() {
    println!("\n--- 解构结构体 ---");
    
    // 创建一个 Rectangle 实例
    let rect = Rectangle {
        width: 40,
        height: 60,
    };
    
    // 解构结构体
    let Rectangle { width, height } = rect;
    println!("解构后的宽: {}, 解构后的高: {}", width, height);
    
    // 在模式匹配中解构
    match rect {
        Rectangle { width: 0, .. } => println!("宽度为 0"),
        Rectangle { height: 0, .. } => println!("高度为 0"),
        Rectangle { width, height } => {
            println!("在 match 中解构: 宽 = {}, 高 = {}", width, height);
        },
    }
    
    // 部分解构
    let Rectangle { width, .. } = rect;
    println!("只解构宽度: {}", width);
    
    // 运行结果：
    // 解构后的宽: 40, 解构后的高: 60
    // 在 match 中解构: 宽 = 40, 高 = 60
    // 只解构宽度: 40
}

// 知识点总结：
// 1. 结构体定义：使用 struct 关键字，定义一组相关联的字段
// 2. 结构体实例化：使用结构体名称和字段值创建实例
// 3. 元组结构体：类似于元组但有名称的结构体
// 4. 单元结构体：没有任何字段的结构体
// 5. 结构体方法：与结构体实例关联的函数，使用 impl 块定义，第一个参数是 self
// 6. 关联函数：与结构体关联但不作用于特定实例的函数，使用结构体名称调用
// 7. 结构体字段可见性：默认私有，使用 pub 关键字可以使其变为公有
// 8. 结构体更新语法：使用 .. 语法从另一个实例复制未显式设置的字段
// 9. 解构结构体：将结构体的字段值绑定到变量，可以在 let 语句或 match 表达式中使用
// 10. 可变结构体：使用 mut 关键字创建可变实例，可以修改其字段值