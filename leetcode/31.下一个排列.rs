/*
 * @lc app=leetcode.cn id=31 lang=rust
 *
 * [31] 下一个排列
 */

// @lc code=start
impl Solution {
    pub fn next_permutation(nums: &mut Vec<i32>) {
        let len = nums.len();
        let mut left:i32 = (len as i32) -2;
        //从后往前找找到降序开始的位置，
        while left >= 0 && nums[left as usize] >= nums[(left+1) as usize] {
            left -= 1;
        }
        let mut right = len-1;
        //如果left等于-1，说明整个数组是降序的
        if left > -1 {
            //从后往前找找到第一个比left位置大的元素，
            while nums[right] <= nums[left as usize] {
                right -= 1;
            }
            //交换left和right位置元素
            unsafe{
                use std::ptr::swap;
                swap(&mut nums[left as usize], &mut nums[right]);
            }
        }
    
        //反转left位置后面降序的元素
        Self::reverse(nums,&mut ((left+1) as usize),&mut (len-1));

    }
    fn reverse(nums:&mut Vec<i32>,left:&mut usize,right:&mut usize){
        while left < right {
            unsafe{
                use std::ptr::swap;
                swap(&mut nums[*left], &mut nums[*right]);
            }
            *left += 1;
            *right -= 1;
        }
    }
}
// @lc code=end

