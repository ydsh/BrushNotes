/*
 * @lc app=leetcode.cn id=746 lang=rust
 *
 * [746] 使用最小花费爬楼梯
 */

// @lc code=start
impl Solution {
    pub fn min_cost_climbing_stairs(cost: Vec<i32>) -> i32 {
           //可以从0或者1开始，那么到达2的花费就有cost[0]+cost[1]或者cost[0]或者cost[1]
    //那么到达2的最小花费为min(cost[0],cost[1]),因此，到达i的最小花费就有
    //min(cost[i-2]+min(i-2),cost[i-1]+min[i-1]),动态规划就有
    //dp[i] = min(cost[i-2]+dp[i-1],cost[i-1]+dp[i-1])
        use std::cmp::min;
        let len = cost.len();
       //初始化动态规划数组
       let mut dp = vec![0i32;len+1];
       //因为可以从0或者1开始，所以0或1的最小花费应该是0，
       dp[0] = 0;
       dp[1] = 0;
       for i in 2..=len {
           dp[i] = min(cost[i-2]+dp[i-2],cost[i-1]+dp[i-1]);
       }
       return dp[len];
    }
}
// @lc code=end

