/*
 * @lc app=leetcode.cn id=18 lang=rust
 *
 * [18] 四数之和
 */

// @lc code=start
impl Solution {
    pub fn four_sum(nums: Vec<i32>, target: i32) -> Vec<Vec<i32>> {
       //该题解与三数之和类似
        let ln = nums.len();
        let mut result:Vec<Vec<i32>> = Vec::new();

        if ln == 0 || ln < 0 {
            return result;
        }
        let mut nums = nums;
        nums.sort_by(|a,b| a.cmp(b));
        for i in 0..ln {
            //去重，防止出现重复的集合
            if i>0 && nums[i] == nums[i-1] {
                continue;
            }
            for j in i+1..ln {
                if j > i+1 && nums[j] == nums[j-1] {
                    continue;
                }
               for k in j+1..ln {
                   if k > j+1&& nums[k] == nums[k-1]{
                       continue;
                   }
                   let mut right:usize = ln-1;
                  // println!("{},{},{},{}",i,j,k,right);
                   while k < right &&  nums[i] + nums[j] + nums[k] + nums[right] > target{
                           right -= 1;
                   }
                   if k >= right {
                       break;
                    }
                    if nums[i] + nums[j] + nums[k] + nums[right] == target {
                       let mut tmp:Vec<i32> = Vec::new();
                       tmp.push(nums[i]);
                       tmp.push(nums[j]);
                       tmp.push(nums[k]);
                       tmp.push(nums[right]);
                       result.push(tmp);
                   }
               }

                
            }
        }
        result
    }
}
// @lc code=end

