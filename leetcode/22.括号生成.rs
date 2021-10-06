/*
 * @lc app=leetcode.cn id=22 lang=rust
 *
 * [22] 括号生成
 */

// @lc code=start
impl Solution {
    pub fn generate_parenthesis(n: i32) -> Vec<String> {

        let mut chs:Vec<char> = Vec::new();
        let mut res:Vec<String> = Vec::new();
        if n <= 0 {
            return Vec::new();
        }
        Self::backtrack(&mut chs,n,0,0,&mut res);

        res

    }
    //回溯法
    fn backtrack(chs:&mut Vec<char>,n:i32,left:i32,right:i32,res:&mut Vec<String>){
        //递归终止条件，当chs的长度等于2n时，把chs转成字符串，加到结果res中
        if (chs.len() as i32) == n*2 {
           res.push(chs.iter().collect::<String>());
           return;
       }
       //左括号不大于n时添加左括号，left加1，
       if left < n {
           chs.push('(');
           Self::backtrack(chs,n,left+1,right,res);
           //递归回到这里时要把刚才的左括号移出去，右括号同理
           chs.pop();
       }
       if right < left {
           chs.push(')');
           Self::backtrack(chs,n,left,right+1,res);
           chs.pop();
       }
    }
}
// @lc code=end

