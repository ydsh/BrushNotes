/*
 * @lc app=leetcode.cn id=552 lang=rust
 *
 * [552] 学生出勤记录 II
 */

// @lc code=start
impl Solution {
    //动态规划解法
    pub fn check_record(n: i32) -> i32 {
        const MOD:i32 = 1000000007;
        let mut res = 0;
       //动态规划数组
      let mut dp = vec![vec![vec![0;3];2];(n as usize +1)];
      
      //初始条件
      dp[0][0][0] = 1;
      for i in 1..=(n as usize) {
        //第i天是A（缺勤）的情况，也就是i之前的出勤不能有缺勤
        for k in 0..3 {
            //第i天是A（缺勤）,所以缺勤可以清零，只计算i之前的可能数据
            dp[i][1][0] = (dp[i][1][0] + dp[i-1][0][k])%MOD;
        }
        //第i天是L（迟到）的情况，也就是i之前的A（缺勤）必须小于2，L（迟到）不大于2次
        for j in 0..2 {
            for k in 1..3 {
                dp[i][j][k] = (dp[i][j][k] + dp[i-1][j][k-1])%MOD;
            }
        }
        //第i天是P（出勤）的情况，也就是i之前的A（缺勤）必须小于2，L（迟到）不大于3次
        for j in 0..2 {
            for k in 0..3 {
                //第i天是P（出勤）,所以缺勤可以清零，只计算i之前的可能数据
                dp[i][j][0] = (dp[i][j][0] + dp[i-1][j][k])%MOD;
            }
        }
        
      }
      //第n天
      for j in 0..2 {
          for k in 0..3 {
              res = (res+dp[n as usize][j][k])%MOD;
          }
      }
      return res;
    }
}
// @lc code=end

