/*
 * @lc app=leetcode.cn id=9 lang=rust
 *
 * [9] 回文数
 */

// @lc code=start
impl Solution {
    pub fn is_palindrome(x: i32) -> bool {
        //负数和最后一位为0的数都不能构成回文数，0除外
        if x < 0 || (x!=0&&x%10==0){
            return false;
        }
        let mut revese = 0;
        let mut num = x;
        
        //只反转一半整数
        while num > revese{
            revese = revese*10 + num%10;
            num /= 10;
        }
        //偶数为和奇数位反转一半后与另一半相等
        return revese==num || revese/10 == num;
    }
}
// @lc code=end

