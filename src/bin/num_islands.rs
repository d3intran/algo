/// 200. 岛屿数量 (Number of Islands)
///
/// 【Rust 面试八股文考点补充 - 多维数组可变借用与所有权】
/// 1. **在 Rust 中为何遍历修改 2D 数组非常棘手？**
///    - `grid` 类型通常传的是 `&mut Vec<Vec<char>>`（如果在闭包/递归外），但在本题，我们将获取传入 `Vec` 的排他所有权，并在原地标记为 '\0' 或 '0' 以表示访问过！
///    - 对于传统 C++：`dfs(grid, r, c)` 随便传不管怎么借用；
///    - 对于 Rust：若通过闭包捕获或者外部辅助函数传递，需要精确控制 `&mut grid` 的借用时间（生命周期不应相互重叠）。
///
/// 2. **如何优美地防止越界 panic**？
///    - C++/Java 里面对数组经常发生 OutOfBound 越界异常。而在 Rust，如果强行用 `grid[r][c]` 数组边界超出程序会立刻崩溃 Panic。
///    - Rust 的防越界神技是 `slice.get() / slice.get_mut()`。它会返回 `Option<&T>`，越界了就只会返回 `None` 而不会崩溃。我们可以完美利用它与 `if let` 的组合来实现极其安全的 DFS 范围搜索！不需要手写四个判断边界的恶心条件！🌟🌟🌟（这点能让面试官极其惊艳！）
pub fn num_islands(mut grid: Vec<Vec<char>>) -> i32 {
    let m = grid.len();
    if m == 0 {
        return 0;
    }
    let n = grid[0].len();

    let mut count = 0;

    for r in 0..m {
        for c in 0..n {
            // 如果遇到陆地，触发一次整体的下沉操作 (DFS)
            if grid[r][c] == '1' {
                count += 1;
                // 这次传入可变引用，并把相连的所有陆地 '1' 标记掉
                dfs(&mut grid, r, c);
            }
        }
    }

    count
}

fn dfs(grid: &mut Vec<Vec<char>>, r: usize, c: usize) {
    // 【八股：Rust 极度优雅及安全的无边界越界访问 (Out-Of-Bounds safe checks)】
    // 通过 .get_mut(r) 拿行，如果存在行，再 .get_mut(c) 拿列的字符引用！
    // 没拿到直接 return 脱离当前函数，无需提前判断 r >= 0 && r < row_len！
    // 只要有任何一环越界或者值匹配不上，这段 if 判断就不会进去！
    if let Some(row) = grid.get_mut(r) {
        if let Some(cell) = row.get_mut(c) {
            if *cell == '1' {
                // 原地覆写借用指针内部的值为已访问 '0'
                *cell = '0';

                // Rust 中 usize - 1 当 usize 是 0 时会发生数值溢出崩溃 (Underflow)。
                // 所以我们需要使用 `.wrapping_sub(1)` 防止溢出（哪怕溢出了越发成了超大整数，接下来的 `get_mut` 也会把它当越界安全拦截成 None！）
                dfs(grid, r.wrapping_sub(1), c); // 上
                dfs(grid, r + 1, c); // 下
                dfs(grid, r, c.wrapping_sub(1)); // 左
                dfs(grid, r, c + 1); // 右
            }
        }
    }
}

fn main() {
    let grid = vec![
        vec!['1', '1', '0', '0', '0'],
        vec!['1', '1', '0', '0', '0'],
        vec!['0', '0', '1', '0', '0'],
        vec!['0', '0', '0', '1', '1'],
    ];
    let res = num_islands(grid);
    println!("200. Number of Islands: {}", res); // 输出 3
}
