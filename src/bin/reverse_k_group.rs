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

/// 25. K 个一组翻转链表 (Reverse Nodes in k-Group)
///
/// 【Rust 面试八股文考点补充 - 可变借用指针的流转与多级解引用】
/// 1. **在 Rust 里移动游标指针为何困难？**
///    - 在 C++ 里面，`ListNode* curr = head` 随便乱指。但在 Rust，如果你通过 `.as_mut().unwrap()` 拿到一个可变引用 `&mut Box<ListNode>` 并赋给 `curr`，
///    - 当你试图 `curr = curr.next.as_mut().unwrap()` 推进它时，编译器会抱怨因为之前的借用期还没结束。
/// 2. **正确的游标推进模式**：
///    - 通过将 `curr` 定义为 `&mut Option<Box<ListNode>>` 这个大管家包裹。
///    - 但是**绝不要使用原生的引用赋值**，因为你不能修改原链表节点的绑定，我们通常会将借用在局部取出处理，然后再将游标赋为新的可变引用边界！
///    - 不过在这里，因为有断开反转再拼接的强要求，我们可以结合之前 `reverse_list` 的打断重组法 + 递归来极其优雅地实现。
pub fn reverse_k_group(head: Option<Box<ListNode>>, k: i32) -> Option<Box<ListNode>> {
    // 【八股：关于可变借用的推进验证】
    // 第一步：先不破坏源数据，仅用不可变借用 & 往前探路
    // 这里非常安全，单纯地确认是否有 k 个节点
    let mut check_curr = &head;
    for _ in 0..k {
        match check_curr {
            Some(node) => check_curr = &node.next, // 向前探路不转移所有权
            None => {
                // 如果不足 k 个，直接全量返回原 head 即可，无需翻转
                return head;
            }
        }
    }

    // 【第二部分：执行反转逻辑（与206题极其相似）】
    // 这个时候确认要反转了，需要获取节点的全部所有权来打散和重组
    let mut prev = None;
    let mut curr = head;

    for _ in 0..k {
        if let Some(mut node) = curr {
            let next_node = node.next.take(); // 打断当前节点与下一个节点的后继
            node.next = prev; // 翻转指针
            prev = Some(node); // 往前移位
            curr = next_node; // 循环下一轮
        }
    }

    // 经过 K 次反转后，此时 `prev` 变成了这 k 个节点反转后的【新头部】！
    // 可是，翻转后链表的【尾部】指向哪儿去了呢？
    // 注意！尾部在翻转完毕后，对应的就是原来的【第一个】节点，并且此时它的 next 是 None！
    //
    // 【极其关键的地道写法：如何在链表末尾续接】
    // 我们必须找到当前反转链表的极其“尾部”，也就是最初的第一个节点。

    let mut tail = &mut prev;
    // 不断地进行可变引用的接力，直到触及链表的末尾 None
    // 对于熟手开发者，这也是 Rust 面试中常用来展示水平的 "可变引用游标探底" 技巧
    while let Some(node) = tail {
        tail = &mut node.next;
    }

    // 第三步：基于已经前进到后面的剩余节点 (此时被 curr 捕获着)，做递归！
    // 将其连接到 `tail` (即现在的 None 的位置) 上。由于 tail 是指向上一节点 next 字段的 &mut ，解引用修改它就直接修改了上一个节点内部结构。
    *tail = reverse_k_group(curr, k);

    // prev 即为新链表的头
    prev
}

fn main() {
    println!("25. Reverse Nodes in k-Group ready!");
}
