/*
 * @lc app=leetcode.cn id=13 lang=rust
 *
 * [13] 罗马数字转整数
 */

// @lc code=start
impl Solution {
    pub fn roman_to_int(s: String) -> i32 {
        let mut ans = 0;
        let mut chars = s.chars().rev();
        let mut prev:char = '0';
        for v in chars {
           if v == 'I' {
               if prev == 'V'||prev=='X' {
                   ans -= 1;
               }else{
                   ans += 1;
               }
           }
           if v == 'V' {
               ans += 5;
           }
           if v == 'X' {
               if prev=='L' || prev=='C'{
                   ans -= 10;
               }else{
                   ans += 10;
               }
           }
           if v == 'L' {
               ans += 50;
           }             
           if v == 'C' {
              if prev == 'D' || prev == 'M'{
                  ans -= 100;
              }else{
                  ans += 100;
              }
           }
           if v == 'D' {
               ans += 500;
           }
           if v == 'M' {
               ans += 1000;
            }
            prev = v;
        }
        ans
    } 
}
// @lc code=end

