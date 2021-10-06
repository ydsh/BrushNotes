/*
 * @lc app=leetcode.cn id=345 lang=rust
 *
 * [345] 反转字符串中的元音字母
 */

// @lc code=start
impl Solution {
    pub fn reverse_vowels(s: String) -> String {
       const VOWEL:[char;5] = ['a','e','i','o','u'];
       let len = s.len();
       if len < 2 {
           return s;
       }
       let mut chars:Vec<char> = s.chars().collect::<Vec<char>>();
       let mut left = 0;
       let mut right = len-1;

       //双指针法遍历字符
       while left < right {
         while left < len&&!VOWEL.contains(&chars[left].to_ascii_lowercase()) {
            left += 1;
         }
         while right > 0 && !VOWEL.contains(&chars[right].to_ascii_lowercase()) {
            right -= 1;
        }
        if left < right {
            let tmp = chars[left];
            chars[left] = chars[right];
            chars[right] = tmp;
            left += 1;
            right -= 1;
        }
       }
       chars.iter().collect::<String>()
    }
}
// @lc code=end

