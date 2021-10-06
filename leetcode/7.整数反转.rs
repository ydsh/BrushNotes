/*
 * @lc app=leetcode.cn id=7 lang=rust
 *
 * [7] 整数反转
 */

// @lc code=start
impl Solution {
    pub fn reverse(x: i32) -> i32 {
      let mut ans = 0;
      let mut num = x;
      //有符号整数最大值，2_147_483_647i32
      let i32_max = std::i32::MAX;
      //有符号整数最小值，-2_147_483_648i32
      let i32_min = std::i32::MIN;
      while num != 0{
          if ans > i32_max/10 || (ans == i32_max/10 && num%10 > 7){
              return 0;
          }
          if ans < i32_min/10 || (ans == i32_min/10 && num%10 < -8) {
              return 0;
          }
          ans = ans*10 + num%10;
          num /= 10;

      }
      return ans;
    }
}
// @lc code=end

