/*
 * @lc app=leetcode.cn id=213 lang=rust
 *
 * [213] 打家劫舍 II
 */

// @lc code=start
impl Solution {
    //dp[i],i>2 dp[i] = max(dp[i-1],dp[1-2]+nums)
    pub fn rob(nums: Vec<i32>) -> i32 {
        use std::cmp::max;
        let len = nums.len();
        let mut ans:i32 = 0;
        if len == 1 {
            return nums[0];
        }else if len == 2 {
            return max(nums[0], nums[1]);
        }
        let mut dp:[i32;2] = [nums[0],max(nums[0],nums[1])];
        //第1间房屋和最后一间房屋不能同时偷窃，可以分解成成从第1间到第n-1间和第2间到第n间
        //0..len-1不包含最后一间房 滚动数组
        for i in 2..len-1 {
            let tmp = dp[1];
            dp[1] = max(nums[i]+dp[0],dp[1]);
            dp[0] = tmp;
        }
        ans = dp[1];
        //println!("{:?}",dp);
        dp[0] = nums[1];
        dp[1] = max(nums[1],nums[2]);
        //1..len 不包含第一间房 滚动数组 
        for i in 3..len {
            let tmp = dp[1];
            dp[1] = max(nums[i]+dp[0],dp[1]);
            dp[0] = tmp;
        }
        //println!("{:?}",dp);
        return max(ans,dp[1]);
    }
}
// @lc code=end

