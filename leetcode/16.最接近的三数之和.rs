 /*
 * @lc app=leetcode.cn id=16 lang=rust
 *
 * [16] 最接近的三数之和
 */

// @lc code=start
impl Solution {
    pub fn three_sum_closest(nums: Vec<i32>, target: i32) -> i32 {
        let mut nums = nums;
        let n = nums.len();
        nums.sort_by(|a,b| a.cmp(b));
        let mut ans:i32 = nums[0]+nums[1]+nums[2];
        let mut i=3;
        use std::cmp::min;
        
        for i in 0..n {
                let mut left = i+1;
                let mut right = n-1;
                while right > left{
                    
                   //比较结果与目标差的绝对值
                    if (ans-target).abs()>(nums[i]+nums[left]+nums[right]-target).abs() {
                        //取绝对值最小的结果
                        ans = nums[i] +nums[left]+nums[right];
                    };
                    
                    if target > (nums[i]+nums[left]+nums[right]) {
                        left += 1;
                    }else {
                        right -= 1;
                    }
                }
                
        }
      ans
    }
}
// @lc code=end

