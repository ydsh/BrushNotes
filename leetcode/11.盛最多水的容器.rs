/*
 * @lc app=leetcode.cn id=11 lang=rust
 *
 * [11] 盛最多水的容器
 */

// @lc code=start
impl Solution {
    pub fn max_area(height: Vec<i32>) -> i32 {
        use std::cmp::max;
        use std::cmp::min;
        let n = height.len();
        let mut cap:i32 = 0;
        //暴力解法
        //for i in 0..n {
        //    for j in 0..n {
        //      cap = max(cap,min(height[i],height[j])*((j-i) as i32));
        //    }
        //}

        //双指针解法（优化）
        let mut left=0;
        let mut right=n-1;

        while left < right {
            //println!("{},{}",left,right);
            cap = max(cap,min(height[left],height[right])*((right-left) as i32));
            //如果左边高度小，就移动左指针(向右移动)
            //如果右边高度小，就移动右指针(向左移动)
            if height[left] < height[right] { 
                left+=1;
            }else{
                right-=1;
            }
        } 
        cap
    }
    //
}
// @lc code=end

