# Rust 学习示例程序

这是一个交互式的 Rust 学习示例程序，通过实际的代码示例和详细的注释，帮助初学者系统性地学习 Rust 编程语言的核心概念。

## 项目结构

项目采用模块化设计，每个 Rust 核心知识点都被组织在独立的文件中，便于学习和理解：

```
src/
├── main.rs                # 主程序入口，提供交互式选择菜单
├── _01_variables.rs       # 变量和数据类型
├── _02_functions_control_flow.rs  # 函数和流程控制
├── _03_structs.rs         # 结构体
├── _04_enums.rs           # 枚举
├── _05_pattern_matching.rs # 模式匹配
├── _06_collections.rs     # 常见集合及其操作
├── _07_packages_modules.rs # 包和模块
├── _08_error_handling.rs  # 错误处理
├── _09_generics.rs        # 泛型
└── _10_lifetimes.rs       # 生命周期
```

## 学习内容概览

### 1. 变量和数据类型 (Variables and Data Types)
- 变量的可变性与不可变性
- 变量声明和初始化
- 基本数据类型：整数、浮点数、布尔值、字符
- 类型标注
- 类型转换
- 常量和静态变量

### 2. 函数和流程控制 (Functions and Control Flow)
- 函数定义和调用
- 函数参数和返回值
- 条件表达式（if-else）
- 循环（loop、while、for）
- 控制流操作符（break、continue、return）

### 3. 结构体 (Structs)
- 结构体定义与实例化
- 元组结构体
- 单元结构体
- 结构体方法
- 关联函数
- 结构体字段可见性
- 结构体更新语法
- 解构结构体

### 4. 枚举 (Enums)
- 枚举定义
- 枚举变体
- Option 枚举
- 枚举方法和关联函数
- 枚举的内存布局

### 5. 模式匹配 (Pattern Matching)
- match 表达式
- if let 表达式
- while let 循环
- for 循环中的模式
- 解构模式
- 通配符和占位符

### 6. 常见集合及其操作 (Collections)
- 向量（Vec）
- 字符串（String）
- HashMap
- HashSet
- 集合的遍历和修改
- 集合的性能特点

### 7. 包和模块 (Packages and Modules)
- 包、 crate 和模块的概念
- 模块的定义和使用
- 路径解析
- pub 关键字控制可见性
- use 关键字简化导入
- 嵌套模块

### 8. 错误处理 (Error Handling)
- 错误类型：可恢复错误和不可恢复错误
- panic! 宏
- Result 枚举
- 错误传播
- 自定义错误类型
- 错误转换
- 错误链
- unwrap 和 expect 方法
- 错误处理最佳实践

### 9. 泛型 (Generics)
- 泛型函数
- 泛型结构体和枚举
- 泛型方法
- 泛型约束
- 关联类型
- 泛型的性能影响

### 10. 生命周期 (Lifetimes)
- 生命周期的概念
- 生命周期注解
- 函数中的生命周期
- 结构体中的生命周期
- 生命周期省略规则
- 静态生命周期

## 如何使用

1. 确保已安装 Rust 和 Cargo
2. 克隆本仓库：
   ```
   git clone https://github.com/zxymax/rustlearn.git
   cd rustlearn
   ```
3. 运行程序：
   ```
   cargo run
   ```
4. 在交互式菜单中输入数字选择要学习的知识点，或输入 `q` 退出程序

## 特点

- **交互式学习**：通过选择菜单系统，自由选择学习内容
- **详细注释**：每个示例都有详细的注释解释代码功能和原理
- **系统化设计**：按照 Rust 学习的逻辑顺序组织内容
- **实用性强**：所有示例都可以直接运行，直观展示 Rust 语法特性

## 目标读者

- Rust 初学者
- 希望系统性学习 Rust 核心概念的开发者
- 需要 Rust 参考资料的程序员

## 贡献

欢迎提交 Issue 和 Pull Request！如果你发现任何错误或有改进建议，请随时联系我们。

## 许可证

[MIT](LICENSE)