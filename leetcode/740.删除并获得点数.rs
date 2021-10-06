/*
 * @lc app=leetcode.cn id=740 lang=rust
 *
 * [740] 删除并获得点数
 */

// @lc code=start
impl Solution {
    
    pub fn delete_and_earn(nums: Vec<i32>) -> i32 {
        use std::cmp::max;
        use std::collections::HashMap;
        let mut len = nums.len();
        let mut map:HashMap<i32,i32> = HashMap::new();
        let mut arr:Vec<i32> = Vec::new();
        arr.push(0);
        for i in 0..len {
            if map.contains_key(&nums[i]) {
              //统计元素个数
            *map.entry(nums[i]).or_insert(0) += 1;
          }else{
              map.insert(nums[i], 1);
              arr.push(nums[i]);
          }
        }
        //排序数组
        arr.sort_by(|a,b| a.cmp(b));
        len = arr.len();
        
        let mut dp:[i32;2] = [arr[0],arr[1]*map.get(&arr[1]).unwrap()];
        //println!("{:?}",dp);
        for k in 2..len {
           let tmp = dp[1];
           let v = map.get(&arr[k]).unwrap();
           if arr[k]-1 == arr[k-1] {
               dp[1] = max(arr[k]*v+dp[0],dp[1]);
           }else{
               dp[1] = dp[1]+arr[k]*v;
           }
           dp[0] = tmp;
        }
        return dp[1];
    }
}
// @lc code=end

