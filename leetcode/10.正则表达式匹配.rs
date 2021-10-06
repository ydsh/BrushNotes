/*
 * @lc app=leetcode.cn id=10 lang=rust
 *
 * [10] 正则表达式匹配
 */

// @lc code=start
impl Solution {
    //动态规划
    pub fn is_match(s: String, p: String) -> bool {
       let m = s.len();
       let n = p.len();
       let s_chars:Vec<char> = s.chars().collect();
       let p_chars:Vec<char> = p.chars().collect();
       let mut dp:Vec<Vec<bool>> = (0..=m).map(|_| (0..=n).map(|_| false).collect::<Vec<bool>>()).collect();
       dp[0][0] = true;
       for i in 0..=m {
           for j in 1..=n {
               if p_chars[j-1]=='*' {
                  dp[i][j] = dp[i][j-2];
                  if i!=0&&(s_chars[i-1]==p_chars[j-2]||p_chars[j-2]=='.') {
                   dp[i][j] = dp[i-1][j]||dp[i][j-1]||dp[i][j-2];
                  }
               }else if i!=0&&(s_chars[i-1]==p_chars[j-1]||p_chars[j-1]=='.') {
                   dp[i][j] = dp[i-1][j-1];
               }
           }
       }
      // println!("{:?}",dp);
       return dp[m][n];
    }

  
}
// @lc code=end

