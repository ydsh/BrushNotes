/*
 * @lc app=leetcode.cn id=55 lang=rust
 *
 * [55] 跳跃游戏
 */

// @lc code=start
impl Solution {
    
    pub fn can_jump(nums: Vec<i32>) -> bool {
       //动态规划
       //Self::dp(nums)
       //贪心算法
       Self::greedy(nums)
    }
    //贪心算法
    fn greedy(nums:Vec<i32>)->bool{
      use std::cmp::max;
      let len = nums.len();
      //能跳越到的最大位置
      let mut arrive_max:i32 = 0;

      for i in 0..len {
          //arrive_max能到达或者超过当前位置i，增更新arrive_max
          if arrive_max >= i as i32 {
              arrive_max = max(arrive_max, nums[i] + i as i32);
          }else{
              return false;
          }
          
      }
      return true;
    }
    // dp[i]表示可以跳跃到最大的位置，
    //dp[i] = max(dp[i-1],i+nums[i]) ，dp[i-1] >= i ,
    //如果是 dp[i-1] >= i && nums[i] + i > length,
    //直接可以跳到终点
    fn dp(nums: Vec<i32>)->bool{
        use std::cmp::max;
     let len = nums.len();
     let mut dp:Vec<i32> = vec![0;len];
     dp[0] = nums[0];
     for i in 1..len {
         if dp[i-1] >= i as i32{
             dp[i] = max(dp[i-1],nums[i] + i as i32);
        }
        if dp[i-1] >= i as i32 && nums[i]+i as i32 >= (len-1) as i32 {
            return true;
        } 
     }
     return dp[len-1] >= (len-1) as i32;
    }
}
// @lc code=end

