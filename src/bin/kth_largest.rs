use std::cmp::Reverse;
use std::collections::BinaryHeap;

/// 215. 数组中的第K个最大元素 (Kth Largest Element in an Array)
///
/// 题目描述：
/// 给定整数数组 nums 和整数 k，请返回数组中第 k 个最大的元素。
/// 请注意，你需要找的是数组排序后的第 k 个最大的元素，而不是第 k 个不同的元素。
/// 你必须设计并实现时间复杂度为 O(n) 的算法解决此问题。
///
/// 【Rust 面试八股文考点补充 - 堆与切片的高级操作】
/// 1. **BinaryHeap 的逆序魔法 (Reverse)**：
///    - `std::collections::BinaryHeap` 底层使用 `Vec` 实现，并且默认是一个**最大堆 (Max Heap)**。
///    - 找第 K 个“最大”元素，最经典的时间 NlogK 解法是维护一个大小为 K 的**最小堆**，每次淘汰掉比堆顶元素还小的数，最后留下来的堆顶就是 K 大元素！
///    - 在 Rust 中，标准库没有 `MinHeap` 类型。相反，它提供了一个 wrapper 叫做 `std::cmp::Reverse<T>`。包裹后会颠倒默认的比较行为（Ord trait 的实装方向发生了翻转），从而让原本的最大堆变成最小堆。这体现了 Rust "Trait 重载机制" 的威力。
///
/// 2. **切片的就地不稳定的排序 (sort_unstable)**:
///    - 另外一种更常被采用的 O(NlogN)（极小常数）做法是使用 `nums.sort_unstable()`。
///    - 面试官非常爱考：为什么大多数时候不用底层的 `sort()`？因为 `sort` 内置的 TimSort 变种是 **稳定的(Stable)** ，会为了保留相同大小元素的相对次序而在堆上分配 O(N/2) 的额外空间。而 `sort_unstable` 使用 Pattern-defeating Quicksort（混合快速排序），属于**就在原地交换**，零内存占用且极快。因为基本数据类型（如数字1和数字1）调换循序也不影响任何物理含义，遇到基本类型请**无脑使用 `sort_unstable`**。
///
/// 3. **面试进阶 O(N) 快排划分 (Quick Select)**：
///    - 由于该题数据量会很大，字节更希望能看到手写的快速选择算法。这里我用原生的切片 (Slice) Swap 在安全 Rust 下做了实现。
///    - 特地避开了 `split_at_mut`，手动展示数组双向游标。
pub fn find_kth_largest_heap(nums: Vec<i32>, k: i32) -> i32 {
    // 解法一：最小堆方案 Time: O(N log k) Space: O(k)
    let k = k as usize;
    let mut min_heap = BinaryHeap::with_capacity(k);

    for x in nums {
        // 利用 Reverse 包裹形成小顶堆效果
        min_heap.push(Reverse(x));
        if min_heap.len() > k {
            min_heap.pop();
        }
    }

    // unwrap() 在这里绝对安全，因为外层限制了 1 <= k <= nums.len()
    min_heap.peek().unwrap().0
}

/// 解法二：基于快排原型的快速选择法 (QuickSelect)
/// 平均时间 O(N), 空间 O(1)，这是大厂最看重的解法。
pub fn find_kth_largest_quickselect(mut nums: Vec<i32>, k: i32) -> i32 {
    let target_idx = nums.len() - (k as usize);
    let n = nums.len();

    // 【八股：Rust 独有的作用域内闭包 (closure)】
    // 为了不污染作用域，我们可以在本方法内层定义一个小函数来执行切片的移动！
    // 甚至可以直接操纵传入在最外层的引用
    quick_select(&mut nums[..], target_idx)
}

// 递归的边界必须限定为切片 `&mut [i32]`。如果是 &mut Vec 在 Rust 中传递的话非常麻烦且不够灵活
fn quick_select(arr: &mut [i32], k: usize) -> i32 {
    if arr.len() <= 1 {
        return arr[0];
    }

    // 三数取中（优化快速排序退化到 O(N²) 的最差性能）等策略在这里省略，直接以第一个元素为枢轴
    let pivot_idx = partition(arr);

    if pivot_idx == k {
        arr[k]
    } else if k < pivot_idx {
        // Rust 切片的区间表示法 `[0..pivot_idx]`
        quick_select(&mut arr[0..pivot_idx], k)
    } else {
        // 在右半边查找，注意原数组的 k 对于右半边数组偏移量需要递减
        quick_select(&mut arr[(pivot_idx + 1)..], k - (pivot_idx + 1))
    }
}

// 标准的快排原地交换逻辑，无新内存开辟
fn partition(arr: &mut [i32]) -> usize {
    let pivot = arr[0];
    let mut i = 1;
    let mut j = arr.len() - 1;

    while i <= j {
        if arr[i] > pivot && arr[j] < pivot {
            arr.swap(i, j); // 切片的 swap 操作，安全极其高效
            i += 1;
            j -= 1;
        }
        if arr[i] <= pivot {
            i += 1;
        }
        if arr[j] >= pivot {
            j -= 1;
        }
    }

    // 把 pivot 换到它最终该呆的位置上
    arr.swap(0, j);
    j
}

fn main() {
    let res = find_kth_largest_quickselect(vec![3, 2, 1, 5, 6, 4], 2);
    println!("215. Kth Largest Element: {}", res); // 输出 5
}
