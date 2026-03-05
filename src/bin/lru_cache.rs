use std::collections::HashMap;

/// 146. LRU 缓存 (LRU Cache)
///
/// 【Rust 面试八股文考点补充 - 链表在 Rust 中的抉择】
/// 1. **为什么在 Rust 中写标准双向链表极其困难？**
///    - 传统 C++/Java 中的双向链表充满了别名（aliasing，多个指针指向同一个节点）和并发写。
///    - Rust 的核心内存安全规则是：**共享不可变，可变不共享**。如果有多个指针（prev 和 next）指向同一节点并试着去修改，编译器会直接拒绝。
///
/// 2. **常见妥协方案与其代价 (`Rc<RefCell<Node>>`)**：
///    - 很多人会用引用计数 `Rc` 和运行期借用检查 `RefCell` 来包住节点。
///    - **缺点**：带来了运行时开销（引用计数增减、动态借用状态检查），丧失了 Rust 的“零成本抽象”优势。
///
/// 3. **🌟 终极大招 / 最佳工程实践：基于数组的内存池 (Arena Allocator)**：
///    - 我们不使用系统的堆指针，而是直接开辟一个扁平的 `Vec<Node>` 数组。
///    - 用 **数组的索引 (`usize`)** 来代替底层的内存指针！
///    - **优点**：完美绕过了所有权和生命周期检查；内存绝对连续存放，**CPU 缓存命中率 (Cache Locality)** 达到了极致，性能暴打传统的指针链表方案！在大厂（如字节）系统编程机试中写出这个方案，面试官会眼前一亮。

#[derive(Clone)]
struct Node {
    key: i32,
    value: i32,
    prev: usize, // 存储的是基于 vec 数组的索引，等价于指针
    next: usize,
}

pub struct LRUCache {
    capacity: usize,
    map: HashMap<i32, usize>,
    pool: Vec<Node>, // 🌟 内存池 Arena
    head: usize,     // 虚拟头节点索引
    tail: usize,     // 虚拟尾节点索引
}

impl LRUCache {
    pub fn new(capacity: i32) -> Self {
        let capacity = capacity as usize;
        // 分配 capacity + 2 个空间（包含虚拟头和虚拟尾）
        let mut pool = vec![
            Node {
                key: 0,
                value: 0,
                prev: 0,
                next: 0
            };
            capacity + 2
        ];
        let head = 0;
        let tail = capacity + 1;

        // 链接虚拟头尾
        pool[head].next = tail;
        pool[tail].prev = head;

        Self {
            capacity,
            map: HashMap::with_capacity(capacity), // 提前分配好哈希表容量，防止扩容开销
            pool,
            head,
            tail,
        }
    }

    // 【八股：关于可变借用 &mut self】
    // 取数据并移动节点，必须是排他性的可变引用 &mut self
    pub fn get(&mut self, key: i32) -> i32 {
        if let Some(&node_idx) = self.map.get(&key) {
            let val = self.pool[node_idx].value;
            self.unlink(node_idx);
            self.push_front(node_idx);
            return val;
        }
        -1
    }

    pub fn put(&mut self, key: i32, value: i32) {
        if let Some(&node_idx) = self.map.get(&key) {
            // 节点存在，更新值并移到头部
            self.pool[node_idx].value = value;
            self.unlink(node_idx);
            self.push_front(node_idx);
        } else {
            // 节点不存在
            let new_node_idx = if self.map.len() == self.capacity {
                // 容量已满，复用淘汰的尾部节点
                let lru_idx = self.pool[self.tail].prev;
                self.unlink(lru_idx);
                let lru_key = self.pool[lru_idx].key;
                self.map.remove(&lru_key);
                lru_idx
            } else {
                // 容量未满，分配新节点
                self.map.len() + 1
            };

            // 写入数据并建立映射
            self.pool[new_node_idx].key = key;
            self.pool[new_node_idx].value = value;
            self.map.insert(key, new_node_idx);
            self.push_front(new_node_idx);
        }
    }

    // 从链表中切断当前节点
    fn unlink(&mut self, idx: usize) {
        let prev = self.pool[idx].prev;
        let next = self.pool[idx].next;
        self.pool[prev].next = next;
        self.pool[next].prev = prev;
    }

    // 将节点插入到虚拟头节点之后（表示最近访问）
    fn push_front(&mut self, idx: usize) {
        let first = self.pool[self.head].next;
        self.pool[idx].prev = self.head;
        self.pool[idx].next = first;
        self.pool[first].prev = idx;
        self.pool[self.head].next = idx;
    }
}

fn main() {
    let mut cache = LRUCache::new(2);
    cache.put(1, 1);
    cache.put(2, 2);
    println!("get 1: {}", cache.get(1)); // 返回  1
    cache.put(3, 3); // 该操作会使得密钥 2 作废
    println!("get 2: {}", cache.get(2)); // 返回 -1 (未找到)
}
