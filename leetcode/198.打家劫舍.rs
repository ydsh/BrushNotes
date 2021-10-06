/*
 * @lc app=leetcode.cn id=198 lang=rust
 *
 * [198] 打家劫舍
 */

// @lc code=start
impl Solution {
    //一夜不能偷窃相邻的两家，AB相邻，一夜就只能偷窃A或者B
    //如果两家都偷窃就会触发警报。给定一组数据arr=[1,2,3]，
    //偷窃arr[1]相邻的arr[2]就不能偷窃，但是能偷窃arr[3]，
    //如果按顺序进行偷窃，那么偷窃到arr[3]所能获取最大收益
    //max(arr[3]+arr[1],arr[2])，这就可以看出在偷窃第n家时获取最大收益就是
    //max(arr[n]+max[n-2],max[n-1]),所以就有动态转移方程
    //dp[i] = max(arr[i]+dp[i-2],dp[i-1])其中i表示沿街的房屋数量减去1，
    //因为数组的索引是从0开始的。
    pub fn rob(nums: Vec<i32>) -> i32 {
      use std::cmp::max;
        let len = nums.len();
      let mut dp = vec![0i32;len];
      
      for i in 0..len {
          if i==0 {
            dp[0] = nums[0]
          }else if i == 1 {
            dp[1] = max(nums[1], nums[0]);
          }else{
            dp[i] = max(nums[i]+dp[i-2], dp[i-1]);
          }
      }
      return dp[len-1];
    }
}
// @lc code=end

