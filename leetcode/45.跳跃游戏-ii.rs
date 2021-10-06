/*
 * @lc app=leetcode.cn id=45 lang=rust
 *
 * [45] 跳跃游戏 II
 */

// @lc code=start
impl Solution {
    pub fn jump(nums: Vec<i32>) -> i32 {
      Self::greedy(nums)
    }
    //贪心算法
     fn greedy(nums: Vec<i32>) -> i32 {
      use std::cmp::max;
      let len =nums.len();
      let mut steps:i32 = 0;
      let mut idx_max:i32 = 0;
      let mut pos:usize = 0;
      //注意len-1边界
      for i in 0..len-1 {
          //能跳跃最远的距离
          idx_max = max(idx_max, nums[i] + i as i32);
          //表示从当前位置跳跃
          if i == pos {
              pos = idx_max as usize;
              steps += 1;
          }
      }
      return steps;
    }
}
// @lc code=end

