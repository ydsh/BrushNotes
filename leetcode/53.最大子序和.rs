/*
 * @lc app=leetcode.cn id=53 lang=rust
 *
 * [53] 最大子序和
 */

// @lc code=start
impl Solution {
    pub fn max_sub_array(mut nums: Vec<i32>) -> i32 {
        let ln = nums.len();
        if ln==1 {
            return nums[0];
        }
        let mut max:i32 = nums[0];
        for i in 1..ln{
            if nums[i-1] > 0{
                nums[i] += nums[i-1]; 
            }
            if max < nums[i] {
                max = nums[i];
            }
        }
        return max;
    }
}
// @lc code=end

