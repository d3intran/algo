use std::collections::HashMap;
use std::io::{self, BufRead};

/// 计算无重复字符的最长子串长度
///
/// 【Rust 面试八股文考点补充】
/// 1. **参数传递 (`String` vs `&str`)**：
///    - 原代码接收 `s: String` 会发生**所有权转移 (Move)**。
///    - **优化建议**：若只需读取，最好传字符串切片 `s: &str`。`&str` 既可以直接接收字符串字面量，也可以通过隐式 `Deref` 接收 `&String`，更加灵活且无额外的堆栈拷贝/分配开销。
///
/// 2. **字符串的底层编码与访问**：
///    - Rust 的字符串是 **UTF-8 编码**的，这意味着字符长短不一（占用 1~4 字节）。
///    - **重要特性**：Rust **不允许**使用 `s[i]` 进行 `O(1)` 时间复杂度的按索引访问，以防止截断多字节字符引发崩溃。这是和 C++/Java 等语言的巨大差异。
///    - 若需要遍历字符，使用 `.chars()` 会依次解析并返回真正的 Unicode 字符（对应 `char` 类型，大小固定为 4 字节）。
pub fn find(s: String) -> i32 {
    // 【Rust 八股：HashMap 原理与性能】
    // Rust 标准库的 `HashMap` 默认使用了 `SipHash` 哈希算法。
    // - **优点**：防哈希碰撞攻击 (HashDoS)，在面对恶意构造数据时性能非常稳定。
    // - **缺点**：对于小对象（如短整型、字符），哈希计算相对更加耗时。
    // 在刷题/大厂机试中，若常数卡得紧，开发者常会引入 `rustc-hash` 库，换用 `FxHashMap` 来大幅提速。
    let mut map = HashMap::new();

    let mut max_len = 0;
    let mut left = 0;

    // `.enumerate()` 返回 `(索引, 元素)` 的元组。
    // 因为是 `.chars()` 生成的迭代器，这里的 `right` 代表的是“第几个字符”，而非字节索引。
    for (right, c) in s.chars().enumerate() {
        // 【Rust 八股：Option 的模式匹配与引用解构】
        // `map.get(&c)` 的返回值是 `Option<&usize>`。
        // `if let Some(&pre) = ...` 不仅实现了 Option 的解包，还利用 `&pre` 模式解构 (destructuring) 了底层借用。
        // 好处是：变量 `pre` 会直接按值复制 (usize 实现 Copy)，从而后面可以直接参与运算，免去了显式的解引用 `*pre`。
        if let Some(&pre) = map.get(&c) {
            // 滑动窗口的左边界只能向右滑，不能倒退
            left = left.max(pre + 1);
        }

        // 更新字符最后一次出现的位置
        map.insert(c, right);

        // 计算当前窗口大小
        max_len = max_len.max(right - left + 1);
    }

    // 【Rust 八股：表达式与隐式返回】
    // - Rust 中，代码块的最后一句如果不加分号，就是一个表达式 (Expression)，其计算结果会作为返回值（称为隐式返回）。
    // - `as i32` 做了数值截断或强制类型转换运算（`usize` -> `i32`）。需要注意对于极大数可能有损转换，但在本题场景是绝对安全的。
    max_len as i32
}

/*
 * 【面试进阶：ASCII 极其优化版 - 惊艳面试官】
 * 如果题目明确指出输入只包含英文字母、数字和符号（纯 ASCII 字符），
 * 把 `String` 降级为字节切片 `&[u8]`，并用定长数组替代 `HashMap` 是最优解，空间 O(1)，时间快几个数量级！
 *
pub fn find_optimized(s: &str) -> i32 {
    let mut pos = [-1_i32; 128]; // ASCII 共 128 种字符
    let mut max_len = 0;
    let mut left = 0;

    // .as_bytes() 获取 &[u8] 原生字节，在纯 ASCII 下它就是完美等价于字符的 O(1) 访问形式
    for (right, &c) in s.as_bytes().iter().enumerate() {
        let (right, c_idx) = (right as i32, c as usize);
        if pos[c_idx] >= left {
            left = pos[c_idx] + 1;
        }
        pos[c_idx] = right;
        max_len = max_len.max(right - left + 1);
    }
    max_len
}
*/

fn main() {
    // 【Rust 八股：标准输入的性能优化】
    // 很多新手用 `io::stdin().read_line()` 每次都在全局获取并释放互斥锁，带来额外开销。
    // idiomatic 解法是：显式调用 `stdin.lock()` 获取锁的所有权/生命周期，
    // 结合包含 `BufRead` 缓冲机制的 `.lines()` 迭代器执行，能极大加速频繁的 I/O。
    let stdin = io::stdin();
    for line in stdin.lock().lines() {
        // 【Rust 八股：错误处理与 unwrap】
        // `unwrap()` 语义是：我笃定这里是 OK，如果是 Err 则直接 panic。
        // 面试中可补充：实际工程中（而非刷题），由于健壮性要求高，我们会用 `?` 向上抛出错误，或用 `match` 妥善处理。
        let s = line.unwrap();

        // 【Rust 八股：变量遮蔽 (Shadowing)】
        // 可以在相同作用域内重复使用由 `let` 声明同名变量。这里的 `s` 重构成了新的不可变 `&str`。
        // 这样带来了很大便利：我们无需绞尽脑汁起类似于 `trimmed_s` 这样的变量名即可继续开发。
        let s = s.trim();
        if s.is_empty() {
            continue;
        }

        // 原函数强求所有权，所以将 `&str` 克隆一份到堆内存并转为 `String` 传入。
        println!("{}", find(s.to_string()));
    }
}
