// 第7课：包和模块 (Packages and Modules)
// 本文件详细介绍 Rust 中的包和模块系统
// 
// 知识点大纲：
// 1. 包（Package）的概念
// 2. Crate 的概念和类型
// 3. 模块（Module）的定义和使用
// 4. 可见性控制（public/private）
// 5. 使用 use 关键字导入模块
// 6. 嵌套模块
// 7. 模块文件结构
// 8. 绝对路径和相对路径
// 9. 外部包的使用
// 10. 工作空间（Workspace）

// 导入必要的模块

// 定义一个公共函数 run()，作为本模块的入口点
// 当用户在主程序中选择 7 时，将调用此函数
pub fn run() {
    println!("=== 第7课：包和模块 ===");
    println!("本示例将介绍 Rust 中的包和模块系统。\n");
    
    // 调用各个示例函数
    package_and_crate_concepts();
    module_definition();
    visibility_control();
    using_use_keyword();
    nested_modules();
    module_file_structure();
    paths_in_rust();
    external_crates();
    workspaces();
    practical_example();
}

// 演示包和 Crate 的概念
fn package_and_crate_concepts() {
    println!("\n--- 包和 Crate 的概念 ---");
    
    println!("Rust 的代码组织层次：");
    println!("1. 包（Package）：是一个项目的基本单位，包含一个 Cargo.toml 文件");
    println!("   - 可以包含多个 Crate");
    println!("   - 至少包含一个 Crate");
    
    println!("2. Crate：是一个编译单元，可以生成可执行文件或库");
    println!("   - 二进制 Crate（Binary Crate）：生成可执行文件，有 main 函数");
    println!("   - 库 Crate（Library Crate）：生成库文件，没有 main 函数");
    
    println!("3. 模块（Module）：用于组织 Crate 中的代码，可以嵌套");
    
    println!("\n当前项目结构：");
    println!("- rustlearn/ (包)");
    println!("  - Cargo.toml (包配置文件)");
    println!("  - src/ (源代码目录)");
    println!("    - main.rs (二进制 Crate 的入口文件)");
    println!("    - lib.rs (库 Crate 的入口文件，如果存在)");
    
    // 运行结果：
    // 包和 Crate 的概念
    // Rust 的代码组织层次：
    // 1. 包（Package）：是一个项目的基本单位，包含一个 Cargo.toml 文件
    //    - 可以包含多个 Crate
    //    - 至少包含一个 Crate
    // 2. Crate：是一个编译单元，可以生成可执行文件或库
    //    - 二进制 Crate（Binary Crate）：生成可执行文件，有 main 函数
    //    - 库 Crate（Library Crate）：生成库文件，没有 main 函数
    // 3. 模块（Module）：用于组织 Crate 中的代码，可以嵌套
    // 
    // 当前项目结构：
    // - rustlearn/ (包)
    //   - Cargo.toml (包配置文件)
    //   - src/ (源代码目录)
    //     - main.rs (二进制 Crate 的入口文件)
    //     - lib.rs (库 Crate 的入口文件，如果存在)
}

// 演示模块的定义
// 在这个函数中，我们将定义一些简单的模块
fn module_definition() {
    println!("\n--- 模块的定义 ---");
    
    // 定义一个简单的模块
    // 注意：在 Rust 中，模块定义通常放在文件的顶部或单独的文件中
    // 这里为了演示，我们在函数内部定义模块
    
    println!("模块的定义使用 'mod' 关键字：");
    println!("mod module_name {{");
    println!("    // 模块内容");
    println!("}}");
    
    // 定义一个数学模块
    mod math {
        // 这个函数默认是私有的
        fn add(a: i32, b: i32) -> i32 {
            a + b
        }
        
        // 使用 pub 关键字使函数变为公共的
        pub fn subtract(a: i32, b: i32) -> i32 {
            a - b
        }
    }
    
    // 由于 add 是私有的，我们无法从模块外部调用它
    // let sum = math::add(5, 3); // 这行会导致编译错误
    
    // 但我们可以调用公共的 subtract 函数
    let difference = math::subtract(10, 4);
    println!("调用公共模块函数 math::subtract(10, 4) = {}", difference);
    
    // 运行结果：
    // 模块的定义
    // 模块的定义使用 'mod' 关键字：
    // mod module_name {
    //     // 模块内容
    // }
    // 调用公共模块函数 math::subtract(10, 4) = 6
}

// 演示可见性控制
fn visibility_control() {
    println!("\n--- 可见性控制 ---");
    
    println!("Rust 使用 'pub' 关键字控制可见性：");
    println!("- 默认情况下，所有内容都是私有的");
    println!("- 使用 'pub' 使其变为公共的");
    println!("- 私有项只能在定义它的模块及其子模块中访问");
    
    // 定义一个包含私有和公共项的模块
    mod company {
        // 私有结构体
        struct Employee {
            name: String,
            position: String,
        }
        
        // 公共结构体，但字段是私有的
        pub struct Department {
            name: String,
            employees: Vec<Employee>,
        }
        
        // 公共结构体，公共字段
        pub struct Project {
            pub name: String,
            pub budget: u32,
        }
        
        impl Department {
            // 公共构造函数
            pub fn new(name: String) -> Self {
                Department {
                    name,
                    employees: Vec::new(),
                }
            }
            
            // 公共方法
            pub fn get_name(&self) -> &String {
                &self.name
            }
        }
        
        // 公共函数
        pub fn create_project(name: String, budget: u32) -> Project {
            Project {
                name,
                budget,
            }
        }
    }
    
    // 创建一个部门
    let engineering = company::Department::new(String::from("Engineering"));
    println!("部门名称: {}", engineering.get_name());
    
    // 创建一个项目
    let website = company::create_project(String::from("Website Redesign"), 50000);
    println!("项目名称: {}, 预算: {}", website.name, website.budget);
    
    // 以下代码会导致编译错误：
    // let employee = company::Employee { ... }; // Employee 是私有的
    // engineering.employees.push(...); // employees 字段是私有的
    
    // 运行结果：
    // 可见性控制
    // Rust 使用 'pub' 关键字控制可见性：
    // - 默认情况下，所有内容都是私有的
    // - 使用 'pub' 使其变为公共的
    // - 私有项只能在定义它的模块及其子模块中访问
    // 部门名称: Engineering
    // 项目名称: Website Redesign, 预算: 50000
}

// 演示使用 use 关键字导入模块
fn using_use_keyword() {
    println!("\n--- 使用 use 关键字导入模块 ---");
    
    println!("'use' 关键字用于导入模块，避免每次都写完整路径：");
    
    // 定义一个模块层次结构
    mod instruments {
        pub mod strings {
            pub struct Guitar {
                pub brand: String,
                pub model: String,
            }
            
            impl Guitar {
                pub fn new(brand: String, model: String) -> Self {
                    Guitar {
                        brand,
                        model,
                    }
                }
                
                pub fn play(&self) {
                    println!("Playing {} {}", self.brand, self.model);
                }
            }
            
            pub struct Piano {
                pub brand: String,
                pub model: String,
            }
            
            impl Piano {
                pub fn new(brand: String, model: String) -> Self {
                    Piano {
                        brand,
                        model,
                    }
                }
            }
        }
        
        pub mod percussion {
            pub struct Drums {
                pub brand: String,
                pub pieces: u8,
            }
            
            impl Drums {
                pub fn new(brand: String, pieces: u8) -> Self {
                    Drums {
                        brand,
                        pieces,
                    }
                }
            }
        }
    }
    
    // 不使用 use 关键字的情况
    let guitar1 = instruments::strings::Guitar::new(String::from("Fender"), String::from("Stratocaster"));
    println!("不使用 use 关键字: 创建了 {} {}", guitar1.brand, guitar1.model);
    
    // 使用 use 关键字导入整个模块
    use instruments::strings;
    let piano = strings::Piano::new(String::from("Yamaha"), String::from("U3"));
    println!("使用 use 导入模块: 创建了 {} {}", piano.brand, piano.model);
    
    // 使用 use 关键字导入特定类型
    use instruments::strings::Guitar;
    let guitar2 = Guitar::new(String::from("Gibson"), String::from("Les Paul"));
    println!("使用 use 导入特定类型: 创建了 {} {}", guitar2.brand, guitar2.model);
    guitar2.play();
    
    // 使用 as 关键字重命名导入
    use instruments::percussion::Drums as DrumKit;
    let drums = DrumKit::new(String::from("Pearl"), 5);
    println!("使用 use as 重命名导入: 创建了 {} 鼓组，共 {} 件", drums.brand, drums.pieces);
    
    // 使用星号导入模块中的所有公共项
    use instruments::strings::*;
    let guitar3 = Guitar::new(String::from("Ibanez"), String::from("RG"));
    let piano2 = Piano::new(String::from("Steinway"), String::from("Model D"));
    println!("使用 * 导入所有公共项: 创建了 {} 和 {} {}", guitar3.brand, piano2.brand, piano2.model);
    
    // 运行结果：
    // 使用 use 关键字导入模块
    // 'use' 关键字用于导入模块，避免每次都写完整路径：
    // 不使用 use 关键字: 创建了 Fender Stratocaster
    // 使用 use 导入模块: 创建了 Yamaha U3
    // 使用 use 导入特定类型: 创建了 Gibson Les Paul
    // Playing Gibson Les Paul
    // 使用 use as 重命名导入: 创建了 Pearl 鼓组，共 5 件
    // 使用 * 导入所有公共项: 创建了 Ibanez 和 Steinway Model D
}

// 演示嵌套模块
fn nested_modules() {
    println!("\n--- 嵌套模块 ---");
    
    println!("Rust 允许模块嵌套，形成层次结构：");
    
    // 定义一个嵌套模块结构
    mod university {
        pub mod faculty {
            pub mod department {
                pub struct Professor {
                    pub name: String,
                    pub subject: String,
                }
                
                impl Professor {
                    pub fn new(name: String, subject: String) -> Self {
                        Professor {
                            name,
                            subject,
                        }
                    }
                    
                    pub fn teach(&self) {
                        println!("Professor {} is teaching {}", self.name, self.subject);
                    }
                }
                
                pub mod research {
                    pub struct Project {
                        pub title: String,
                        pub funding: u32,
                    }
                    
                    impl Project {
                        pub fn new(title: String, funding: u32) -> Self {
                            Project {
                                title,
                                funding,
                            }
                        }
                    }
                }
            }
        }
    }
    
    // 访问嵌套模块中的项
    let prof = university::faculty::department::Professor::new(
        String::from("Dr. Smith"),
        String::from("Computer Science")
    );
    prof.teach();
    
    // 使用 use 简化嵌套模块的访问
    use university::faculty::department::research::Project;
    let project = Project::new(
        String::from("AI Research"),
        100000
    );
    println!("研究项目: {},  funding: ${}", project.title, project.funding);
    
    // 运行结果：
    // 嵌套模块
    // Rust 允许模块嵌套，形成层次结构：
    // Professor Dr. Smith is teaching Computer Science
    // 研究项目: AI Research,  funding: $100000
}

// 演示模块文件结构
fn module_file_structure() {
    println!("\n--- 模块文件结构 ---");
    
    println!("Rust 中模块与文件系统的关系：");
    println!("1. 每个文件都是一个模块");
    println!("2. 模块可以通过两种方式定义：");
    println!("   a. 在文件中使用 'mod' 关键字定义内联模块");
    println!("   b. 使用单独的文件或目录来定义模块");
    println!("3. 对于同名的目录和文件，目录会被优先使用");
    
    println!("\n模块文件结构示例：");
    println!("src/");
    println!("  main.rs (或 lib.rs)");
    println!("  module1.rs (模块文件)");
    println!("  module2.rs (模块文件)");
    println!("  module3/ (模块目录)");
    println!("    mod.rs (模块目录的入口文件)");
    println!("    submodule1.rs");
    println!("    submodule2.rs");
    
    println!("\n在当前项目中，我们的模块结构：");
    println!("src/");
    println!("  main.rs (主入口文件)");
    println!("  lesson_01.rs (第1课模块)");
    println!("  lesson_02.rs (第2课模块)");
    println!("  ...");
    println!("  lesson_07.rs (当前模块)");
    
    // 运行结果：
    // 模块文件结构
    // Rust 中模块与文件系统的关系：
    // 1. 每个文件都是一个模块
    // 2. 模块可以通过两种方式定义：
    //    a. 在文件中使用 'mod' 关键字定义内联模块
    //    b. 使用单独的文件或目录来定义模块
    // 3. 对于同名的目录和文件，目录会被优先使用
    // 
    // 模块文件结构示例：
    // src/
    //   main.rs (或 lib.rs)
    //   module1.rs (模块文件)
    //   module2.rs (模块文件)
    //   module3/ (模块目录)
    //     mod.rs (模块目录的入口文件)
    //     submodule1.rs
    //     submodule2.rs
    // 
    // 在当前项目中，我们的模块结构：
    // src/
    //   main.rs (主入口文件)
    //   lesson_01.rs (第1课模块)
    //   lesson_02.rs (第2课模块)
    //   ...
    //   lesson_07.rs (当前模块)
}

// 演示 Rust 中的路径
fn paths_in_rust() {
    println!("\n--- Rust 中的路径 ---");
    
    println!("Rust 中有两种路径表示方式：");
    println!("1. 绝对路径：从 crate 根开始，使用 crate:: 前缀");
    println!("2. 相对路径：从当前模块开始，使用 self::、super:: 或模块名称");
    
    // 定义模块结构来演示路径
    mod animals {
        pub mod mammals {
            pub struct Dog {
                pub name: String,
            }
            
            impl Dog {
                pub fn bark(&self) {
                    println!("{} is barking!", self.name);
                }
            }
        }
        
        pub mod birds {
            pub struct Sparrow {
                pub name: String,
            }
            
            impl Sparrow {
                pub fn chirp(&self) {
                    println!("{} is chirping!", self.name);
                }
                
                // 演示相对路径和绝对路径
                pub fn interact_with_mammal(&self) {
                    // 绝对路径 - 修改为相对路径，因为animals模块在函数内部
                    let dog = super::mammals::Dog {
                        name: String::from("Rex"),
                    };
                    dog.bark();
                    
                    // 相对路径（使用 super）
                    let dog2 = super::mammals::Dog {
                        name: String::from("Fido"),
                    };
                    dog2.bark();
                    
                    println!("{} is watching the dogs", self.name);
                }
            }
        }
    }
    
    // 使用相对路径（因为animals是在函数内部定义的）
    let dog = animals::mammals::Dog {
        name: String::from("Buddy"),
    };
    dog.bark();
    
    // 使用相对路径
    let sparrow = animals::birds::Sparrow {
        name: String::from("Tweety"),
    };
    sparrow.chirp();
    sparrow.interact_with_mammal();
    
    // 运行结果：
    // Rust 中的路径
    // Rust 中有两种路径表示方式：
    // 1. 绝对路径：从 crate 根开始，使用 crate:: 前缀
    // 2. 相对路径：从当前模块开始，使用 self::、super:: 或模块名称
    // Buddy is barking!
    // Tweety is chirping!
    // Rex is barking!
    // Fido is barking!
    // Tweety is watching the dogs
}

// 演示外部包的使用
fn external_crates() {
    println!("\n--- 外部包的使用 ---");
    
    println!("在 Rust 中使用外部包的步骤：");
    println!("1. 在 Cargo.toml 文件中添加依赖");
    println!("2. 使用 'use' 关键字导入外部包中的项");
    
    println!("\n例如，要使用 rand 包生成随机数：");
    println!("// 在 Cargo.toml 中添加：");
    println!("[dependencies]");
    println!("rand = \"0.8.5\"\n");
    
    println!("// 在代码中使用：");
    println!("use rand::Rng;");
    println!("fn main() {{");
    println!("    let random_number = rand::thread_rng().gen_range(1..=100);");
    println!("    println!(\"随机数: {{}}\" , random_number);");
    println!("}}\n");
    
    println!("当前项目没有添加额外的外部依赖，所以我们不能实际演示外部包的使用。");
    println!("如果需要使用外部包，请在 Cargo.toml 文件中添加依赖。");
    
    // 运行结果：
    // 外部包的使用
    // 在 Rust 中使用外部包的步骤：
    // 1. 在 Cargo.toml 文件中添加依赖
    // 2. 使用 'use' 关键字导入外部包中的项
    // 
    // 例如，要使用 rand 包生成随机数：
    // // 在 Cargo.toml 中添加：
    // [dependencies]
    // rand = "0.8.5"
    // 
    // // 在代码中使用：
    // use rand::Rng;
    // fn main() {
    //     let random_number = rand::thread_rng().gen_range(1..=100);
    //     println!("随机数: {{}}" , random_number);
    // }
    // 
    // 当前项目没有添加额外的外部依赖，所以我们不能实际演示外部包的使用。
    // 如果需要使用外部包，请在 Cargo.toml 文件中添加依赖。
}

// 演示工作空间
fn workspaces() {
    println!("\n--- 工作空间 ---");
    
    println!("工作空间（Workspace）是一组共享相同 Cargo.lock 和输出目录的包：");
    println!("1. 用于管理多个相互依赖的包");
    println!("2. 创建一个根目录，包含 Cargo.toml 文件定义工作空间");
    
    println!("\n工作空间的 Cargo.toml 示例：");
    println!("[workspace]");
    println!("members = [");
    println!("    \"package1\",");
    println!("    \"package2\",");
    println!("    \"path/to/package3\",");
    println!("]");
    
    println!("\n工作空间的优势：");
    println!("- 共享依赖，避免重复下载");
    println!("- 统一构建和测试");
    println!("- 方便管理多包项目");
    
    // 运行结果：
    // 工作空间
    // 工作空间（Workspace）是一组共享相同 Cargo.lock 和输出目录的包：
    // 1. 用于管理多个相互依赖的包
    // 2. 创建一个根目录，包含 Cargo.toml 文件定义工作空间
    // 
    // 工作空间的 Cargo.toml 示例：
    // [workspace]
    // members = [
    //     "package1",
    //     "package2",
    //     "path/to/package3",
    // ]
    // 
    // 工作空间的优势：
    // - 共享依赖，避免重复下载
    // - 统一构建和测试
    // - 方便管理多包项目
}

// 实用的模块组织示例
fn practical_example() {
    println!("\n--- 实用的模块组织示例 ---");
    
    // 定义一个更复杂的模块结构，模拟一个简单的电子商务系统
    mod ecommerce {
        pub mod products {
            pub struct Product {
                pub id: u32,
                pub name: String,
                pub price: f64,
                pub category: String,
            }
            
            impl Product {
                pub fn new(id: u32, name: String, price: f64, category: String) -> Self {
                    Product {
                        id,
                        name,
                        price,
                        category,
                    }
                }
                
                pub fn display(&self) {
                    println!("Product #{}: {}, ${}, Category: {}", 
                             self.id, self.name, self.price, self.category);
                }
            }
        }
        
        pub mod cart {
            use super::products::Product;
            
            pub struct ShoppingCart {
                items: Vec<(Product, u32)>, // (产品, 数量)
            }
            
            impl ShoppingCart {
                pub fn new() -> Self {
                    ShoppingCart {
                        items: Vec::new(),
                    }
                }
                
                pub fn add_item(&mut self, product: Product, quantity: u32) {
                    self.items.push((product, quantity));
                }
                
                pub fn remove_item(&mut self, product_id: u32) {
                    self.items.retain(|(product, _)| product.id != product_id);
                }
                
                pub fn calculate_total(&self) -> f64 {
                    self.items.iter()
                        .map(|(product, quantity)| product.price * *quantity as f64)
                        .sum()
                }
                
                pub fn display(&self) {
                    println!("购物车内容：");
                    for (product, quantity) in &self.items {
                        product.display();
                        println!("数量: {}", quantity);
                        println!("小计: ${}", product.price * *quantity as f64);
                        println!("---");
                    }
                    println!("总计: ${}", self.calculate_total());
                }
            }
        }
        
        pub mod customer {
            pub struct Customer {
                pub id: u32,
                pub name: String,
                pub email: String,
            }
            
            impl Customer {
                pub fn new(id: u32, name: String, email: String) -> Self {
                    Customer {
                        id,
                        name,
                        email,
                    }
                }
            }
        }
    }
    
    // 使用电子商务模块
    use ecommerce::products::Product;
    use ecommerce::cart::ShoppingCart;
    use ecommerce::customer::Customer;
    
    // 创建一些产品
    let laptop = Product::new(1, String::from("Laptop"), 999.99, String::from("Electronics"));
    let phone = Product::new(2, String::from("Smartphone"), 499.99, String::from("Electronics"));
    let book = Product::new(3, String::from("Rust Programming Book"), 29.99, String::from("Books"));
    
    // 创建一个客户
    let customer = Customer::new(101, String::from("John Doe"), String::from("john@example.com"));
    
    // 创建购物车并添加产品
    let mut cart = ShoppingCart::new();
    cart.add_item(laptop, 1);
    cart.add_item(phone, 2);
    cart.add_item(book, 3);
    
    // 显示客户信息和购物车
    println!("客户: {}, Email: {}", customer.name, customer.email);
    cart.display();
    
    // 删除一个产品并重新计算总价
    println!("\n删除产品 2 后的购物车：");
    cart.remove_item(2);
    cart.display();
    
    // 运行结果：
    // 实用的模块组织示例
    // 客户: John Doe, Email: john@example.com
    // 购物车内容：
    // Product #1: Laptop, $999.99, Category: Electronics
    // 数量: 1
    // 小计: $999.99
    // ---
    // Product #2: Smartphone, $499.99, Category: Electronics
    // 数量: 2
    // 小计: $999.98
    // ---
    // Product #3: Rust Programming Book, $29.99, Category: Books
    // 数量: 3
    // 小计: $89.97
    // ---
    // 总计: $2089.94
    // 
    // 删除产品 2 后的购物车：
    // 购物车内容：
    // Product #1: Laptop, $999.99, Category: Electronics
    // 数量: 1
    // 小计: $999.99
    // ---
    // Product #3: Rust Programming Book, $29.99, Category: Books
    // 数量: 3
    // 小计: $89.97
    // ---
    // 总计: $1089.96
}

// 知识点总结：
// 1. 包（Package）：是 Rust 项目的基本单位，包含一个 Cargo.toml 文件，定义项目的元数据和依赖。
// 2. Crate：是 Rust 的编译单元，可以是二进制 Crate（生成可执行文件）或库 Crate（生成库文件）。
// 3. 模块（Module）：用于组织 Crate 中的代码，控制可见性和命名空间。
// 4. 可见性控制：使用 pub 关键字控制模块项的可见性，默认为私有。
// 5. use 关键字：用于导入模块，避免每次都写完整路径，可以使用 as 重命名导入。
// 6. 嵌套模块：Rust 允许模块嵌套，形成层次结构，便于组织复杂代码。
// 7. 模块文件结构：模块可以定义在单个文件中，也可以使用目录结构组织。
// 8. 路径：Rust 使用绝对路径（以 crate:: 开头）和相对路径（使用 self::、super:: 或模块名称）访问模块项。
// 9. 外部包：在 Cargo.toml 中添加依赖，可以使用外部包提供的功能。
// 10. 工作空间：用于管理多个相互依赖的包，共享相同的 Cargo.lock 和输出目录。