/*
 * @lc app=leetcode.cn id=32 lang=rust
 *
 * [32] 最长有效括号
 */

// @lc code=start
impl Solution {
    pub fn longest_valid_parentheses(s: String) -> i32 {
        //暴力解法
        //Self::resolve1(s)
        //动态规划
        //Self::resolve2(s)
        //栈
        //Self::resolve3(s)
        //正向逆向结合
        Self::resolve4(s)
    }
    //1、暴力解法，从最大长度开始，找到第一个符合的就是最大长度
    fn resolve1(s:String)->i32{
        use std::collections::VecDeque;
        let len = s.len();
        let chars:Vec<char> = s.chars().collect();
        
        if len < 2 {
            return 0;
        }
        
        for i in (2..=len).rev() {
            if i&1==0 {
                //println!("{}",i);
                let mut left:usize = 0;
                let mut right:usize = i;
                //滑动窗口
                while right <= len {
                    let mut stack:VecDeque<char> = VecDeque::new();
                    //判断是否是有效括号
                    for k in left..right {
                        if stack.is_empty() {
                            stack.push_back(chars[k]);
                        }else{
                            //判断栈顶元素，当前是右括号并且栈顶元素是左括号，
                            //就弹出栈顶元素；否则当前元素入栈。
                            if let Some(v) = stack.pop_back() {
                                if chars[k] != ')' || v != '(' {
                                    stack.push_back(v);
                                    stack.push_back(chars[k]);
                                }
                            }
                        }
                    }
                    //println!("{:?}",stack);
                    //栈是空的说明该长度是最大长度
                     if stack.is_empty() {
                         return i as i32;
                     }
                    left += 1;
                    right += 1;
                }
            }
        }

        return 0;
    }
    //2、动态规划
    fn resolve2(s:String)->i32{
        use std::cmp::max;
        let len = s.len();
        let chars:Vec<char> = s.chars().collect();
        let mut dp:Vec<i32> = (0..len).map(|_a| 0).collect();
        let mut max_len:i32 = 0;
        if len < 2 {
            return 0;
        }
        for i in 1..len {
            if chars[i] == ')'{
                //dp[i]的前一个是左括号，
                //或者dp[i]前面的是一串有效括号，再看这一串括号之前的括号是否是左括号
                if i <= 2 && chars[i-1]=='('{
                    dp[i] = 2;
                }else if i-1>=(dp[i-1] as usize)&&chars[i-(dp[i-1] as usize)-1]=='('{
                     
                    dp[i] = 2 + dp[i-1] + if i-1>(dp[i-1] as usize) {
                        dp[i-(dp[i-1] as usize)-2] }else{0};
                }

                max_len = max(max_len, dp[i]);
            }
        }
        return max_len;
    }
    //3、使用栈（这是最开始想到的解法，但是写的有问题）
    fn resolve3(s:String)->i32{
        use std::collections::VecDeque;
        use std::cmp::max;
        let len = s.len();
        let chars:Vec<char> = s.chars().collect();
        let mut stack:VecDeque<i32> = VecDeque::new();
        let mut max_len:i32 = 0;
        let mut sub_len:i32 = 0;
        stack.push_back(-1);
        for i in 0..len {
           if chars[i] == '(' {
               stack.push_back(i as i32);
           }else{
               stack.pop_back();
               if stack.is_empty() {
                   stack.push_back(i as i32);
               }else{
                   let v = stack.pop_back().unwrap();
                   max_len = max(max_len, i as i32 - v);
                   stack.push_back(v);
               }
           }
        }
          return max_len;
    }
    //正向逆向结合
    fn resolve4(s:String)->i32{
        use std::cmp::max;
        let len = s.len();
        let chars:Vec<char> = s.chars().collect();
        let mut left:i32 = 0;
        let mut right:i32 = 0;
        let mut max_len:i32 = 0;
        //正向
        for i in 0..len {
            if chars[i] == '(' {
                left += 1;
            }else{
                right += 1;
            }
            if left == right {
                max_len = max(max_len, left*2);
            }else if right > left{
               left = 0;
               right = 0;
            }
        }
       //逆向
       left = 0;
       right = 0;
       for i in (0..len).rev() {
        if chars[i] == ')' {
            right += 1;
        }else{
            left += 1;
        }
        if left == right {
            max_len = max(max_len, left*2);
        }else if left > right{
           left = 0;
           right = 0;
        }
      }
        return max_len;
    }
}
// @lc code=end

