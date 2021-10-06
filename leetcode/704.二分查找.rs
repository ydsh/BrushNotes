/*
 * @lc app=leetcode.cn id=704 lang=rust
 *
 * [704] 二分查找
 */

// @lc code=start
impl Solution {
    pub fn search(nums: Vec<i32>, target: i32) -> i32 {
       let len = nums.len();
       let mut left:i32 = 0;
       let mut right:i32 = (len as i32) - 1;

       while left <= right {
           let mid = (left + right)/2;
           if target > nums[mid as usize] {
               left = mid + 1;
           }else if target < nums[mid as usize] {
               right = mid - 1;
           }else{
               return mid;
           }
       } 
      -1
    }
}
// @lc code=end

