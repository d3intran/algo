use std::cell::RefCell;
use std::collections::VecDeque;
use std::rc::Rc;

#[derive(Debug, PartialEq, Eq)]
pub struct TreeNode {
    pub val: i32,
    pub left: Option<Rc<RefCell<TreeNode>>>,
    pub right: Option<Rc<RefCell<TreeNode>>>,
}

impl TreeNode {
    #[inline]
    pub fn new(val: i32) -> Self {
        TreeNode {
            val,
            left: None,
            right: None,
        }
    }
}

/// 103. 二叉树的锯齿形层序遍历 (Binary Tree Zigzag Level Order Traversal)
///
/// 【Rust 面试八股文考点补充 - 树结构的智能指针】
/// 1. **为什么 LeetCode 用 `Rc<RefCell<TreeNode>>` 来表示树节点？**
///    - `Rc` (Reference Counted) 允许多个指针拥有同一个节点（多所有权）。虽然在标准的树里只有父节点指向子节点，但在测试/构建套件里，LeetCode 框架为了方便在外面保留节点的句柄进行快速组装，妥协使用了多所有权的引用计数方案。
///    - `RefCell` 提供了**内部可变性 (Interior Mutability)**。它把编译期（静态）的借用规则检查推迟到了运行期（动态）。这意味着你可以传入不可变的 `Rc` 引用，但在运行时依旧可以通过 `.borrow_mut()` 修改它内部的字段！
///
/// 2. **`Rc` 克隆的零成本与所有权防范**：
///    - `Rc::clone(&node)` 只是增加了引用计数的 integer 计数器（非常轻量），并**不会**深度拷贝整个树节点。这一点区别于 `node.clone()`（在没有 derive Copy 的情况下也会调用对应的 clone实现如果是 Rc 则等价）。
///    - 在遍历的时候不要害怕通过 `clone` 把它入队，这不仅是安全的，而且是符合 Rust 设计哲学的 O(1) 浅拷贝。
///
/// 3. **双向队列 `VecDeque` 及其环形缓冲 (Ring Buffer)**：
///    - Rust 的标准库提供了基于环形缓冲区实现的 `VecDeque`，支持 O(1) 的两端增删 `push_front`/`push_back`/`pop_front`/`pop_back`。
///    - 当遇到这道题需要的“从左往右”与“从右往左”交替时，我们不仅用它做 BFS 遍历的队列，还可以用来做每一层收集结果时的双端缓冲！
pub fn zigzag_level_order(root: Option<Rc<RefCell<TreeNode>>>) -> Vec<Vec<i32>> {
    let mut res = Vec::new();
    if root.is_none() {
        return res;
    }

    // 初始化 BFS 使用的队列
    let mut queue = VecDeque::new();
    queue.push_back(root.unwrap()); // 解包安全

    let mut left_to_right = true;

    while !queue.is_empty() {
        let level_size = queue.len();
        // 专门为本层的结果集分配双端队列
        let mut row = VecDeque::with_capacity(level_size);

        for _ in 0..level_size {
            // 出队当前层的节点
            let node_rc = queue.pop_front().unwrap();
            let node = node_rc.borrow(); // 这里是关键：动态拿到 RefCell 里面的不可变借用

            // 根据方向决定向行缓冲区的哪一端推入数字
            if left_to_right {
                row.push_back(node.val);
            } else {
                row.push_front(node.val);
            }

            // 将子节点入队用于下一层
            // 注意传递的是 Rc 的 clone()，引用计数加 1
            if let Some(left_child) = &node.left {
                queue.push_back(Rc::clone(left_child));
            }
            if let Some(right_child) = &node.right {
                queue.push_back(Rc::clone(right_child));
            }
        }

        // 双端队列转化为 Vec 载入结果
        res.push(row.into());
        left_to_right = !left_to_right;
    }

    res
}

fn main() {
    println!("103. Binary Tree Zigzag Level Order Traversal ready!");
}
