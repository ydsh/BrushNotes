/*
 * @lc app=leetcode.cn id=224 lang=rust
 *
 * [224] 基本计算器
 */

// @lc code=start
//栈保存符号，
//转换为加法
impl Solution {
    pub fn calculate(s: String) -> i32 {
        let chars:Vec<char> = s.chars().collect();
        //根据当前符号或者操作符变换栈顶元素正负栈
        let mut ops:Vec<i32> = Vec::new();
        //默认为正
        ops.push(1);
        let mut result:i32 = 0;
        let mut sign:i32 = 1;
        let n = chars.len();
        let mut i = 0;
        while i < n {
            match chars[i] {
                '+' => {
                    //遇到+,查看栈顶符号，符号不变
                    if let Some(v) = ops.pop() {
                        sign = v;
                        ops.push(v);
                        i += 1;
                    }
                   },
                 '-' => {
                     //遇到-,查看栈顶符号，符号取反
                        if let Some(v) = ops.pop() {
                            sign = -v;
                            ops.push(v);
                            i += 1;
                        }
                 },
                 '(' => {
                     //遇到（,将之前的符号入栈，相当性的表达式开始
                      ops.push(sign);
                      i += 1;
                 },
                 ')' => {
                     //遇到),弹出栈顶元素，变回之前的表达式
                      ops.pop();
                      i += 1;
                 },
            '0'..='9' => {
                    //遇到数字,这里要使用循环，检查字符是否是连续数字
                    let mut num = 0;
                    while i < n && chars[i] >= '0' && chars[i] <= '9' {
                        num = num * 10 + (chars[i] as i32 - 48);
                        i += 1;
                    }
                    //累加每一项带符号的数字
                    result += sign*num;
                },
                _ => {
                    i += 1;
                }
            };
        }
     result
    }
}
// @lc code=end

