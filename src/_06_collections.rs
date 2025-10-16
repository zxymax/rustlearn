// 第6课：常见集合及其操作 (Collections)
// 本文件详细介绍 Rust 中的常见集合类型和操作方法
// 
// 知识点大纲：
// 1. Vector (动态数组)
// 2. String (字符串)
// 3. HashMap (哈希映射)
// 4. HashSet (哈希集合)
// 5. BTreeMap (有序映射)
// 6. BTreeSet (有序集合)
// 7. 集合的遍历和迭代
// 8. 集合的常见操作
// 9. 集合的性能特点
// 10. 集合的所有权问题

// 导入标准输出模块和需要的集合类型
use std::collections::{HashMap, HashSet, BTreeMap, BTreeSet};

// 定义一个公共函数 run()，作为本模块的入口点
// 当用户在主程序中选择 6 时，将调用此函数
pub fn run() {
    println!("=== 第6课：常见集合及其操作 ===");
    println!("本示例将介绍 Rust 中的常见集合类型和操作方法。\n");
    
    // 调用各个示例函数
    vector_collection();
    string_collection();
    hashmap_collection();
    hashset_collection();
    btreemap_collection();
    btreeset_collection();
    collection_iteration();
    common_operations();
    performance_considerations();
    ownership_issues();
}

// 演示 Vector 集合
// Vector 是一个动态数组，可以存储多个相同类型的元素
fn vector_collection() {
    println!("\n--- Vector (动态数组) ---");
    
    // 创建一个新的空 Vector
    let mut v1: Vec<i32> = Vec::new();
    println!("创建空 Vector: v1 = {:?}", v1);
    
    // 使用 vec! 宏创建包含初始值的 Vector
    let v2 = vec![1, 2, 3, 4, 5];
    println!("使用 vec! 宏创建 Vector: v2 = {:?}", v2);
    
    // 向 Vector 中添加元素
    v1.push(10);
    v1.push(20);
    v1.push(30);
    println!("添加元素后: v1 = {:?}", v1);
    
    // 访问 Vector 中的元素
    // 方法 1: 使用索引访问（如果索引越界会导致程序崩溃）
    let first = v2[0];
    println!("v2 的第一个元素: {}", first);
    
    // 方法 2: 使用 get 方法访问（安全，返回 Option）
    let second = v2.get(1);
    match second {
        Some(value) => println!("v2 的第二个元素: {}", value),
        None => println!("索引超出范围"),
    }
    
    // 遍历 Vector
    println!("遍历 v2 中的元素:");
    for element in &v2 {
        println!("{} ", element);
    }
    
    // 修改 Vector 中的元素
    println!("修改前: v1 = {:?}", v1);
    if let Some(element) = v1.get_mut(0) {
        *element = 100;
    }
    println!("修改后: v1 = {:?}", v1);
    
    // 获取 Vector 的长度
    println!("v1 的长度: {}", v1.len());
    
    // 检查 Vector 是否为空
    println!("v1 是否为空: {}", v1.is_empty());
    
    // 运行结果：
    // 创建空 Vector: v1 = []
    // 使用 vec! 宏创建 Vector: v2 = [1, 2, 3, 4, 5]
    // 添加元素后: v1 = [10, 20, 30]
    // v2 的第一个元素: 1
    // v2 的第二个元素: 2
    // 遍历 v2 中的元素:
    // 1 
    // 2 
    // 3 
    // 4 
    // 5 
    // 修改前: v1 = [10, 20, 30]
    // 修改后: v1 = [100, 20, 30]
    // v1 的长度: 3
    // v1 是否为空: false
}

// 演示 String 集合
// String 是 Rust 中的可变字符串类型
fn string_collection() {
    println!("\n--- String (字符串) ---");
    
    // 创建一个空字符串
    let mut s1 = String::new();
    println!("创建空字符串: s1 = '{}'", s1);
    
    // 使用字符串字面量创建 String
    let s2 = String::from("Hello");
    println!("使用 from 方法创建字符串: s2 = '{}'", s2);
    
    // 拼接字符串
    s1.push_str("Rust");
    println!("使用 push_str 添加字符串: s1 = '{}'", s1);
    
    // 添加单个字符
    s1.push('!');
    println!("使用 push 添加字符: s1 = '{}'", s1);
    
    // 使用 + 运算符拼接字符串
    let s3 = s2.clone() + " " + &s1;
    println!("使用 + 运算符拼接字符串: s3 = '{}'", s3);
    
    // 使用 format! 宏拼接字符串
    let s4 = format!("{} {} {}", s2, s1, "World");
    println!("使用 format! 宏拼接字符串: s4 = '{}'", s4);
    
    // 访问字符串长度
    println!("s4 的长度: {}", s4.len());
    
    // 遍历字符串中的字符
    println!("遍历 s4 中的字符:");
    for c in s4.chars() {
        println!("{}", c);
    }
    
    // 遍历字符串中的字节
    println!("遍历 s4 中的前 10 个字节:");
    for (i, byte) in s4.bytes().take(10).enumerate() {
        println!("字节 {}: {}", i, byte);
    }
    
    // 注意：Rust 的 String 不支持直接通过索引访问字符
    // 因为 UTF-8 编码的字符可能占用多个字节
    
    // 运行结果：
    // 创建空字符串: s1 = ''
    // 使用 from 方法创建字符串: s2 = 'Hello'
    // 使用 push_str 添加字符串: s1 = 'Rust'
    // 使用 push 添加字符: s1 = 'Rust!'
    // 使用 + 运算符拼接字符串: s3 = 'Hello Rust!'
    // 使用 format! 宏拼接字符串: s4 = 'Hello Rust! World'
    // s4 的长度: 17
    // 遍历 s4 中的字符:
    // H
    // e
    // l
    // l
    // o
    // 
    // R
    // u
    // s
    // t
    // !
    // 
    // W
    // o
    // r
    // l
    // d
    // 遍历 s4 中的前 10 个字节:
    // 字节 0: 72
    // 字节 1: 101
    // 字节 2: 108
    // 字节 3: 108
    // 字节 4: 111
    // 字节 5: 32
    // 字节 6: 82
    // 字节 7: 117
    // 字节 8: 115
    // 字节 9: 116
}

// 演示 HashMap 集合
// HashMap 是一个键值对集合，基于哈希表实现
fn hashmap_collection() {
    println!("\n--- HashMap (哈希映射) ---");
    
    // 创建一个新的空 HashMap
    let mut scores = HashMap::new();
    
    // 向 HashMap 中插入键值对
    scores.insert(String::from("Alice"), 100);
    scores.insert(String::from("Bob"), 85);
    scores.insert(String::from("Charlie"), 90);
    
    println!("HashMap: {:?}", scores);
    
    // 访问 HashMap 中的值
    let name = String::from("Alice");
    if let Some(score) = scores.get(&name) {
        println!("{} 的分数: {}", name, score);
    } else {
        println!("未找到 {} 的分数", name);
    }
    
    // 遍历 HashMap
    println!("遍历 HashMap:");
    for (key, value) in &scores {
        println!("{}: {}", key, value);
    }
    
    // 检查键是否存在
    let name2 = String::from("David");
    println!("HashMap 中是否包含 {}: {}", name2, scores.contains_key(&name2));
    
    // 插入新值并获取旧值
    let old_score = scores.insert(String::from("Alice"), 105);
    println!("Alice 的旧分数: {:?}", old_score);
    println!("更新后的 HashMap: {:?}", scores);
    
    // 只在键不存在时插入
    scores.entry(String::from("David")).or_insert(75);
    scores.entry(String::from("Alice")).or_insert(0); // 不会覆盖现有值
    println!("使用 entry 方法后的 HashMap: {:?}", scores);
    
    // 获取 HashMap 的长度
    println!("HashMap 的长度: {}", scores.len());
    
    // 删除键值对
    scores.remove(&name);
    println!("删除 Alice 后的 HashMap: {:?}", scores);
    
    // 运行结果：
    // HashMap: {"Bob": 85, "Alice": 100, "Charlie": 90}
    // Alice 的分数: 100
    // 遍历 HashMap:
    // Bob: 85
    // Alice: 100
    // Charlie: 90
    // HashMap 中是否包含 David: false
    // Alice 的旧分数: Some(100)
    // 更新后的 HashMap: {"Bob": 85, "Alice": 105, "Charlie": 90}
    // 使用 entry 方法后的 HashMap: {"Bob": 85, "Alice": 105, "David": 75, "Charlie": 90}
    // HashMap 的长度: 4
    // 删除 Alice 后的 HashMap: {"Bob": 85, "David": 75, "Charlie": 90}
}

// 演示 HashSet 集合
// HashSet 是一个存储唯一值的集合，基于哈希表实现
fn hashset_collection() {
    println!("\n--- HashSet (哈希集合) ---");
    
    // 创建一个新的空 HashSet
    let mut numbers = HashSet::new();
    
    // 向 HashSet 中插入元素
    numbers.insert(1);
    numbers.insert(2);
    numbers.insert(3);
    numbers.insert(4);
    numbers.insert(5);
    
    // 尝试插入重复的元素（将被忽略）
    let result = numbers.insert(3);
    println!("插入重复元素 3 的结果: {}", result);
    
    println!("HashSet: {:?}", numbers);
    
    // 检查元素是否存在
    println!("HashSet 中是否包含 3: {}", numbers.contains(&3));
    println!("HashSet 中是否包含 10: {}", numbers.contains(&10));
    
    // 遍历 HashSet
    println!("遍历 HashSet:");
    for number in &numbers {
        println!("{}", number);
    }
    
    // 获取 HashSet 的长度
    println!("HashSet 的长度: {}", numbers.len());
    
    // 删除元素
    numbers.remove(&3);
    println!("删除 3 后的 HashSet: {:?}", numbers);
    
    // 集合操作
    let mut set1: HashSet<i32> = vec![1, 2, 3, 4, 5].into_iter().collect();
    let set2: HashSet<i32> = vec![4, 5, 6, 7, 8].into_iter().collect();
    
    // 交集
    let intersection: HashSet<_> = set1.intersection(&set2).collect();
    println!("set1 和 set2 的交集: {:?}", intersection);
    
    // 并集
    set1.extend(&set2);
    println!("set1 和 set2 的并集: {:?}", set1);
    
    // 运行结果：
    // 插入重复元素 3 的结果: false
    // HashSet: {3, 1, 2, 5, 4}
    // HashSet 中是否包含 3: true
    // HashSet 中是否包含 10: false
    // 遍历 HashSet:
    // 3
    // 1
    // 2
    // 5
    // 4
    // HashSet 的长度: 5
    // 删除 3 后的 HashSet: {1, 2, 5, 4}
    // set1 和 set2 的交集: {4, 5}
    // set1 和 set2 的并集: {7, 1, 4, 5, 6, 8, 2, 3}
}

// 演示 BTreeMap 集合
// BTreeMap 是一个按键排序的键值对集合，基于 B 树实现
fn btreemap_collection() {
    println!("\n--- BTreeMap (有序映射) ---");
    
    // 创建一个新的空 BTreeMap
    let mut scores = BTreeMap::new();
    
    // 向 BTreeMap 中插入键值对
    scores.insert(String::from("Alice"), 100);
    scores.insert(String::from("Bob"), 85);
    scores.insert(String::from("Charlie"), 90);
    
    println!("BTreeMap: {:?}", scores);
    
    // 遍历 BTreeMap（按键排序）
    println!("遍历 BTreeMap (按键排序):");
    for (key, value) in &scores {
        println!("{}: {}", key, value);
    }
    
    // BTreeMap 支持的其他操作与 HashMap 类似
    // 如 get, contains_key, len, remove 等
    
    // 运行结果：
    // BTreeMap: {"Alice": 100, "Bob": 85, "Charlie": 90}
    // 遍历 BTreeMap (按键排序):
    // Alice: 100
    // Bob: 85
    // Charlie: 90
}

// 演示 BTreeSet 集合
// BTreeSet 是一个存储唯一值并自动排序的集合，基于 B 树实现
fn btreeset_collection() {
    println!("\n--- BTreeSet (有序集合) ---");
    
    // 创建一个新的空 BTreeSet
    let mut numbers = BTreeSet::new();
    
    // 向 BTreeSet 中插入元素
    numbers.insert(5);
    numbers.insert(2);
    numbers.insert(7);
    numbers.insert(1);
    numbers.insert(9);
    
    println!("BTreeSet: {:?}", numbers);
    
    // 遍历 BTreeSet（自动排序）
    println!("遍历 BTreeSet (自动排序):");
    for number in &numbers {
        println!("{}", number);
    }
    
    // BTreeSet 支持的其他操作与 HashSet 类似
    // 如 contains, len, remove 等
    
    // 运行结果：
    // BTreeSet: {1, 2, 5, 7, 9}
    // 遍历 BTreeSet (自动排序):
    // 1
    // 2
    // 5
    // 7
    // 9
}

// 演示集合的遍历和迭代
// 所有集合类型都支持迭代操作
fn collection_iteration() {
    println!("\n--- 集合的遍历和迭代 ---");
    
    // 遍历 Vector
    let v = vec![10, 20, 30, 40, 50];
    println!("遍历 Vector (不可变引用):");
    for element in &v {
        println!("{}", element);
    }
    
    // 可变遍历 Vector
    let mut mv = vec![10, 20, 30, 40, 50];
    println!("\n遍历 Vector (可变引用，增加值):");
    for element in &mut mv {
        *element += 5;
        println!("{}", element);
    }
    
    // 使用 into_iter 消耗集合
    println!("\n使用 into_iter 消耗 Vector:");
    let sum: i32 = v.into_iter().sum();
    println!("元素总和: {}", sum);
    
    // 遍历 HashMap
    let mut map = HashMap::new();
    map.insert("one", 1);
    map.insert("two", 2);
    map.insert("three", 3);
    
    println!("\n遍历 HashMap 的键:");
    for key in map.keys() {
        println!("{}", key);
    }
    
    println!("\n遍历 HashMap 的值:");
    for value in map.values() {
        println!("{}", value);
    }
    
    println!("\n遍历 HashMap 的键值对:");
    for (key, value) in &map {
        println!("{}: {}", key, value);
    }
    
    // 运行结果：
    // 遍历 Vector (不可变引用):
    // 10
    // 20
    // 30
    // 40
    // 50
    // 
    // 遍历 Vector (可变引用，增加值):
    // 15
    // 25
    // 35
    // 45
    // 55
    // 
    // 使用 into_iter 消耗 Vector:
    // 元素总和: 150
    // 
    // 遍历 HashMap 的键:
    // three
    // one
    // two
    // 
    // 遍历 HashMap 的值:
    // 3
    // 1
    // 2
    // 
    // 遍历 HashMap 的键值对:
    // three: 3
    // one: 1
    // two: 2
}

// 演示集合的常见操作
// 各种集合都支持一些常见的操作
fn common_operations() {
    println!("\n--- 集合的常见操作 ---");
    
    // Vector 的常见操作
    let mut v = vec![1, 2, 3, 4, 5];
    println!("原始 Vector: {:?}", v);
    
    // 添加元素
    v.push(6);
    println!("添加元素后: {:?}", v);
    
    // 删除最后一个元素
    let last = v.pop();
    println!("删除的最后一个元素: {:?}", last);
    println!("删除后: {:?}", v);
    
    // 插入元素
    v.insert(2, 100);
    println!("在索引 2 插入 100 后: {:?}", v);
    
    // 删除指定索引的元素
    let removed = v.remove(2);
    println!("删除索引 2 的元素: {}", removed);
    println!("删除后: {:?}", v);
    
    // 字符串的常见操作
    let mut s = String::from("Hello");
    println!("\n原始字符串: '{}'", s);
    
    // 追加字符串
    s.push_str(" Rust");
    println!("追加字符串后: '{}'", s);
    
    // 检查前缀和后缀
    println!("以 'Hello' 开头: {}", s.starts_with("Hello"));
    println!("以 'Rust' 结尾: {}", s.ends_with("Rust"));
    
    // 替换子字符串
    let new_s = s.replace("Rust", "World");
    println!("替换后: '{}'", new_s);
    
    // 运行结果：
    // 原始 Vector: [1, 2, 3, 4, 5]
    // 添加元素后: [1, 2, 3, 4, 5, 6]
    // 删除的最后一个元素: Some(6)
    // 删除后: [1, 2, 3, 4, 5]
    // 在索引 2 插入 100 后: [1, 2, 100, 3, 4, 5]
    // 删除索引 2 的元素: 100
    // 删除后: [1, 2, 3, 4, 5]
    // 
    // 原始字符串: 'Hello'
    // 追加字符串后: 'Hello Rust'
    // 以 'Hello' 开头: true
    // 以 'Rust' 结尾: true
    // 替换后: 'Hello World'
}

// 演示集合的性能特点
// 不同的集合类型有不同的性能特点
fn performance_considerations() {
    println!("\n--- 集合的性能特点 ---");
    
    // Vector 性能特点
    println!("Vector 性能特点:");
    println!("- 随机访问: O(1)");
    println!("- 在末尾添加/删除元素: 平均 O(1)");
    println!("- 在中间插入/删除元素: O(n)");
    
    // String 性能特点
    println!("\nString 性能特点:");
    println!("- 追加字符串到末尾: 平均 O(1)");
    println!("- 随机访问字符: O(n) (因为 UTF-8 编码)");
    
    // HashMap 性能特点
    println!("\nHashMap 性能特点:");
    println!("- 插入键值对: 平均 O(1)");
    println!("- 查找键值对: 平均 O(1)");
    println!("- 删除键值对: 平均 O(1)");
    println!("- 遍历: O(n)");
    
    // BTreeMap 性能特点
    println!("\nBTreeMap 性能特点:");
    println!("- 插入键值对: O(log n)");
    println!("- 查找键值对: O(log n)");
    println!("- 删除键值对: O(log n)");
    println!("- 有序遍历: O(n)");
    
    // 选择集合的建议
    println!("\n选择集合的建议:");
    println!("- 需要动态数组: 使用 Vector");
    println!("- 需要键值对映射且不需要排序: 使用 HashMap");
    println!("- 需要键值对映射且需要排序: 使用 BTreeMap");
    println!("- 需要存储唯一值且不需要排序: 使用 HashSet");
    println!("- 需要存储唯一值且需要排序: 使用 BTreeSet");
    
    // 运行结果：
    // Vector 性能特点:
    // - 随机访问: O(1)
    // - 在末尾添加/删除元素: 平均 O(1)
    // - 在中间插入/删除元素: O(n)
    // 
    // String 性能特点:
    // - 追加字符串到末尾: 平均 O(1)
    // - 随机访问字符: O(n) (因为 UTF-8 编码)
    // 
    // HashMap 性能特点:
    // - 插入键值对: 平均 O(1)
    // - 查找键值对: 平均 O(1)
    // - 删除键值对: 平均 O(1)
    // - 遍历: O(n)
    // 
    // BTreeMap 性能特点:
    // - 插入键值对: O(log n)
    // - 查找键值对: O(log n)
    // - 删除键值对: O(log n)
    // - 有序遍历: O(n)
    // 
    // 选择集合的建议:
    // - 需要动态数组: 使用 Vector
    // - 需要键值对映射且不需要排序: 使用 HashMap
    // - 需要键值对映射且需要排序: 使用 BTreeMap
    // - 需要存储唯一值且不需要排序: 使用 HashSet
    // - 需要存储唯一值且需要排序: 使用 BTreeSet
}

// 演示集合的所有权问题
// 在 Rust 中使用集合时需要注意所有权问题
fn ownership_issues() {
    println!("\n--- 集合的所有权问题 ---");
    
    // Vector 的所有权
    let s1 = String::from("hello");
    let s2 = String::from("world");
    
    let mut v = Vec::new();
    v.push(s1); // s1 的所有权转移给了 Vector
    v.push(s2); // s2 的所有权转移给了 Vector
    
    println!("Vector 中的字符串: {:?}", v);
    
    // 以下代码会导致编译错误，因为 s1 和 s2 的所有权已经转移
    // println!("s1: {}", s1); // 错误
    // println!("s2: {}", s2); // 错误
    
    // 如果只想借用字符串，可以使用引用
    let s3 = String::from("rust");
    let s4 = String::from("programming");
    
    let mut v_refs = Vec::new();
    v_refs.push(&s3); // 借用 s3
    v_refs.push(&s4); // 借用 s4
    
    println!("Vector 中的字符串引用: {:?}", v_refs);
    println!("s3: {}, s4: {}", s3, s4); // 仍然可以使用 s3 和 s4
    
    // HashMap 的所有权
    let mut map = HashMap::new();
    let key1 = String::from("one");
    let value1 = String::from("一");
    
    map.insert(key1, value1); // key1 和 value1 的所有权转移给了 HashMap
    
    println!("HashMap: {:?}", map);
    
    // 以下代码会导致编译错误
    // println!("key1: {}", key1); // 错误
    // println!("value1: {}", value1); // 错误
    
    // 使用引用作为键和值
    let key2 = String::from("two");
    let value2 = String::from("二");
    
    let mut map_refs = HashMap::new();
    map_refs.insert(&key2, &value2); // 借用 key2 和 value2
    
    println!("HashMap with references: {:?}", map_refs);
    println!("key2: {}, value2: {}", key2, value2); // 仍然可以使用 key2 和 value2
    
    // 运行结果：
    // Vector 中的字符串: ["hello", "world"]
    // Vector 中的字符串引用: ["rust", "programming"]
    // s3: rust, s4: programming
    // HashMap: {"one": "一"}
    // HashMap with references: {"two": "二"}
    // key2: two, value2: 二
}

// 知识点总结：
// 1. Vector：动态数组，用于存储多个相同类型的元素，支持随机访问
// 2. String：可变字符串类型，支持各种字符串操作
// 3. HashMap：基于哈希表的键值对集合，提供平均 O(1) 的查找性能
// 4. HashSet：基于哈希表的唯一值集合，不允许重复元素
// 5. BTreeMap：基于 B 树的有序键值对集合，按键自动排序
// 6. BTreeSet：基于 B 树的有序唯一值集合，自动排序且不允许重复元素
// 7. 集合的遍历和迭代：所有集合都支持迭代操作，包括不可变引用、可变引用和消耗性迭代
// 8. 集合的常见操作：添加、删除、查找、插入等
// 9. 集合的性能特点：不同集合有不同的性能特性，应根据实际需求选择
// 10. 集合的所有权问题：使用集合时需要注意 Rust 的所有权规则，避免悬垂引用等问题