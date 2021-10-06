/*
 * @lc app=leetcode.cn id=12 lang=rust
 *
 * [12] 整数转罗马数字
 */

// @lc code=start
impl Solution {
    pub fn int_to_roman(num: i32) -> String {
        use std::collections::HashMap;
        let mut result:String = String::from("");
        let mut tmp = num;
        let nums = vec![1000,900,500,400,100,90,50,40,10,9,5,4,1];
        let maps = vec!["M","CM","D","CD","C","XC","L","XL","X","IX","V","IV","I"];
        while tmp > 0 {
            for n in 0..nums.len() {
                if tmp >= nums[n] {
                    result.push_str(maps[n]);
                    tmp -= nums[n];
                    break;
                }
            }
        }
        result
    }
}
// @lc code=end

