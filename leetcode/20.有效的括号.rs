/*
 * @lc app=leetcode.cn id=20 lang=rust
 *
 * [20] 有效的括号
 */

// @lc code=start
impl Solution {
    pub fn is_valid(s: String) -> bool {
      let n = s.len();
      if n&1 == 1 {
          return false;
      }
      let chs:Vec<char> = s.chars().collect();
      let mut stack:Vec<char> = Vec::new();
      use std::collections::HashMap;
      let mut map:HashMap<char,char> = HashMap::new();
      map.insert(')','(');
      map.insert(']','[');
      map.insert('}','{'); 
      for ch in chs {
          //判断字符时是否是右边括号，
          //如果不是就将该字符入栈
          if let Some(_) = map.get(&ch) {
              let n = stack.len();
              //比较栈顶字符是否是对应的左括号，
              //如果是就将栈顶元素弹出
              if n==0||stack[n-1]!=*map.get(&ch).unwrap_or(&'_'){
                  return false;
              }else{
                  stack.pop();
              }
          }else{
              stack.push(ch);
          }
      }
      if stack.is_empty(){
          true
      }else{
          false
      }
    }
}
// @lc code=end

