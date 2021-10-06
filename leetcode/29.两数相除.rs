/*
 * @lc app=leetcode.cn id=29 lang=rust
 *
 * [29] 两数相除
 */

// @lc code=start
impl Solution {
    pub fn divide(dividend: i32, divisor: i32) -> i32 {
         let mut ans:i64 = i64::default();
         //符号
         let mut sign:i64 = 1;

         if (dividend > 0 && divisor < 0) || (dividend < 0 && divisor > 0) {
             sign = -1;
         }
         //println!("{}",dividend);
         if dividend == i32::MIN && divisor == -1 {
             return i32::MAX;
         }
         let mut dividend:i64 = (dividend as i64).abs()&0xffffffff;
         let divisor:i64 = (divisor as i64).abs()&0xffffffff;
         //位操作除法
         for i in (0..=31).rev() {
             //从高位开始获取
            let m = dividend >> i;
            if m >= divisor {
                ans |= 1 << i;
                dividend -= divisor << i;
            }
         }
         
         
        (ans * sign) as i32
         
    }
}
// @lc code=end

