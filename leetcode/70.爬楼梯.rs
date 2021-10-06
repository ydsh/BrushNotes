/*
 * @lc app=leetcode.cn id=70 lang=rust
 *
 * [70] 爬楼梯
 */

// @lc code=start
impl Solution {
    pub fn climb_stairs(n: i32) -> i32 {
        if n < 2 {
            return n;
        }
        //根据规律，爬n阶楼梯的方式有F(n) = F(n-1)+F(n-2)种方式，
        //与斐波那契数列非常相似，所以使用dp数组保存爬n-2阶和爬n-1
        //阶楼梯的方式，所以最总n阶楼梯的方式是dp[0] + dp[1]
        let mut dp:[i32;2] = [1,1];
        for i in 2..(n as usize){
           let t = dp[1];
           dp[1] = dp[0] + t;
           dp[0] = t;
        }
        return dp[0]+dp[1];
    }
}
// @lc code=end

