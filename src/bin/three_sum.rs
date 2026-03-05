use std::cmp::Ordering;

/// 15. 三数之和 (3Sum)
///
/// 【Rust 面试八股文考点补充 - 模式匹配与 Ordering】
/// 1. **切片原生排序策略之最**：
///    - 与第 215 题提到的一样，排序数字或字符请**永远优先使用 `sort_unstable`**。稳定排序不仅慢，更重要是在不必要的情况下平白消耗了堆内存的开辟与垃圾回收，背离我们手动写系统级代码的初衷。
///
/// 2. **优美的 `std::cmp::Ordering` 模式匹配**：
///    - 在 C++ 里如果要比较数字大小并三段分流，经常会写很长的 `if (sum > 0) ... else if (sum < 0) ... else ...`。
///    - 在 Rust 里，标准库的 `.cmp()` 方法返回一个 `Ordering` 枚举（包含 `Less`, `Equal`, `Greater`）。
///    - 我们结合 `match` 匹配块不仅能让代码具有无解的美学，还要求开发者**必须穷尽**所有的枚举类型。漏写哪一个分支编译器都会打回，极其安全！
pub fn three_sum(mut nums: Vec<i32>) -> Vec<Vec<i32>> {
    let mut res = Vec::new();
    if nums.len() < 3 {
        return res;
    }

    // 【八股：In-place 不稳定排序的原地交换】
    nums.sort_unstable();

    for i in 0..nums.len() - 2 {
        // 去重 1：跳过相同基准值的循环
        if i > 0 && nums[i] == nums[i - 1] {
            continue;
        }

        // 剪枝强化：此时如果基于排序连首位的值都 >0，往后全是大数，不可能加出 0
        if nums[i] > 0 {
            break;
        }

        let mut left = i + 1;
        let mut right = nums.len() - 1;

        while left < right {
            let sum = nums[i] + nums[left] + nums[right];

            // 【八股：极其地道（Idiomatic）的三路模式匹配】
            match sum.cmp(&0) {
                Ordering::Less => left += 1,
                Ordering::Greater => right -= 1,
                Ordering::Equal => {
                    res.push(vec![nums[i], nums[left], nums[right]]);
                    left += 1;
                    right -= 1;

                    // 去重 2 与去重 3：滑动双指针，跳过已重复收集的数字
                    while left < right && nums[left] == nums[left - 1] {
                        left += 1;
                    }
                    while left < right && nums[right] == nums[right + 1] {
                        right -= 1;
                    }
                }
            }
        }
    }

    res
}

fn main() {
    let res = three_sum(vec![-1, 0, 1, 2, -1, -4]);
    println!("15. 3Sum: {:?}", res);
    // expected: [[-1, -1, 2], [-1, 0, 1]]
}
