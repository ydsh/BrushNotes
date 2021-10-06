/*
 * @lc app=leetcode.cn id=516 lang=rust
 *
 * [516] 最长回文子序列
 */

// @lc code=start
impl Solution {
    //最长回文子串同样是经典的动态规划问题，给定一个字符串的长度，
    //要找出最长的回文子序列的长度,我们可以建立一个二维动态数组dp,
    //那么dp[i][j]就表示字符串s[i]到s[j]的回文串的长度，那么接下来
    //就是明确状态转移方程了，
    //我们判断dp[i][j]是否是回文串：
    //1、如果s[i]==s[j],在dp[i+1][j-1]是回文串的前提下，
    //那么dp[i][j] = dp[i+1][j-1] + 2;
    //2、如果s[i]!=s[j],s[i]和s[j]就不能同时作为一个回文穿的首尾，
    //那么dp[i][j] = Max(dp[i][j-1],dp[i+1][j]);
    //还有一点，单个字符时也是一个回文串，即dp[i][i]=1;
    pub fn longest_palindrome_subseq(s: String) -> i32 {
      let ln = s.len();
      if ln == 0 {
          return 0;
      }
      let chars:Vec<char> = s.chars().collect();
      //初始化dp数组
      let mut dp:Vec<Vec<i32>> = (0..ln).map(|i| (0..ln).map(|j| {if i==j {1}else{0}}).collect::<Vec<i32>>()).collect();
      //遍历的方向不同，结果在dp数组的位置就不一样，
      //这里是最后的结果在d[ln-1][0]位置
      for i in 1..ln {
        for j in (0..i).rev() {
            //println!("{},{}",i,j);
            //由于遍历方向不同所以加减要变换
            if chars[i] == chars[j] {
                dp[i][j] = dp[i-1][j+1] + 2;
            }else{
                use std::cmp::max;
                dp[i][j] = max(dp[i-1][j],dp[i][j+1]);
            }
        }
    }
    //println!{"{:?}",dp};
    dp[ln-1][0]
    }
}
// @lc code=end

