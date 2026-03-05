// 链表节点的标准定义
#[derive(PartialEq, Eq, Clone, Debug)]
pub struct ListNode {
    pub val: i32,
    pub next: Option<Box<ListNode>>,
}

impl ListNode {
    #[inline]
    fn new(val: i32) -> Self {
        ListNode { next: None, val }
    }
}

/// 206. 反转链表 (Reverse Linked List)
///
/// 题目描述：
/// 给你单链表的头节点 head ，请你反转链表，并返回反转后的链表。
/// 进阶：链表可以选用迭代或递归方式完成反转。你能否用两种方法解决这道题？
///
/// 【Rust 面试八股文考点补充 - 智能指针与所有权流转】
/// 1. **Option 枚举与绝对的空指针安全**：
///    - C/C++/Java 中的 `null` 随时会引发崩溃，被称为“十亿美元的错误”。
///    - Rust 在语言级别不存在 Null 控制流，必须用 `Option<T>` 枚举 (`Some` 或 `None`) 明确包装指针。在解包时，编译器强制要求穷尽匹配，彻底杜绝了 Null 崩溃。
///
/// 2. **Box<T> 的堆分配机制**：
///    - 为什么链表包含自己的结构必须加 `Box` (如 `next: Option<Box<ListNode>>`)？
///    - 因为 Rust 要求所有的 Struct 大小在**编译时必须明确计算出来 (Sized)**。如果一个 Struct 直接包含自己（Recursive struct），它的大小就是无限延伸的，无法放在栈上。
///    - `Box<T>` 会把数据申请在**堆 (Heap)** 上，而在栈上只留下一个固定大小的指针。
///
/// 3. **核心操作法宝：`take()` 方法 (权能剥夺)**：
///    - 链表翻转的核心是打断链接并拿走节点，但这会改变所有权。
///    - `Option::take()` 是在处理结构体内部成员（由于你只有 `&mut` 而没有完整所有权时）的神器。它会原封不动地返回原本内部的 `Some(val)`，同时把原地设置为空 `None`。完全符合链表截断指针的物理直觉！
pub fn reverse_list(mut head: Option<Box<ListNode>>) -> Option<Box<ListNode>> {
    let mut prev = None;

    // 【八股：while let 语法糖】
    // 当我们只需要循环处理 `Some` 而遇到 `None` 退出时，`while let` 是极其优雅的地道写法
    while let Some(mut curr) = head {
        // 通过 take() 将 curr.next 的所有权剥离出来，存放在临时变量中
        // 此时 curr.next 在原来的节点上变为了 None
        let next_node = curr.next.take();

        // 翻转指针，把当前节点的下一个节点指回之前的列表
        curr.next = prev;

        // 往前推进，当前节点包裹回 Some 就变成了下一轮的 prev
        prev = Some(curr);

        // 迭代头部
        head = next_node;
    }

    // 最终 prev 会成为反转后新的头节点
    prev
}

fn main() {
    println!("206. Reverse Linked List ready!");
}
