/*
 * @lc app=leetcode.cn id=15 lang=rust
 *
 * [15] 三数之和
 */

// @lc code=start
impl Solution {
    pub fn three_sum(nums: Vec<i32>) -> Vec<Vec<i32>> {
        let mut result:Vec<Vec<i32>> = Vec::new();
        let ln = nums.len();
        let mut nums = nums;
        //排序
        nums.sort_by(|a,b| a.cmp(b));
        for i in 0..ln {
            //去重
            if i > 0 && nums[i]==nums[i-1] {
                continue;
            }
            for j in i+1..ln {
                //去重
                if j > i + 1 && nums[j]==nums[j-1] {
                    continue;
                }
                let mut right = ln-1;
                //两数之和大于第三个数时，right向左移动
                while j < right && nums[j] + nums[right] > -nums[i]{
                    right -= 1;
                }
                //边界判断，退出当前循环
                if right <= j {
                    break;
                }
                let mut tmp:Vec<i32> = Vec::new();
                //三数之和为0，放入Vec中，将Vec放入结果中 
                if nums[i] + nums[j] + nums[right]  == 0 {
                        tmp.push(nums[i]);
                        tmp.push(nums[j]);
                        tmp.push(nums[right]);
                        result.push(tmp);
                }
            }
            
        }
       result
    }
    
}
// @lc code=end

