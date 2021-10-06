/*
 * @lc app=leetcode.cn id=17 lang=rust
 *
 * [17] 电话号码的字母组合
 */

// @lc code=start
impl Solution {
    pub fn letter_combinations(digits: String) -> Vec<String> {
       let mut arr:Vec<&str> = vec!["","","abc","def","ghi","jkl","mno","pqrs","tuv","wxyz"];
       let letters:Vec<String> = arr.iter().map(|v| v.to_string()).collect();
       let chars:Vec<char> = digits.chars().collect();
       let mut ans:Vec<String> = Vec::new();
       let mut buf:String = String::from("");
       //深度优先
       Self::dfs(&chars,0,&letters,&mut ans,&mut buf);  
       ans
       //广度优先
       //Self::bfs(&chars,&letters)
    }
    //深度优先(看成树形结构)
    //"23":
    //  [a       b        c]
    //   |       |        |
    //[d e f] [d e f]  [d e f]
    fn dfs(chs:&Vec<char>,index:usize,letters:&Vec<String>,ans:&mut Vec<String>,buf:&mut String) {
        
        if chs.len()==0 {
            return;
        }
        if index==chs.len() {
           ans.push(buf.to_string());
            return;
        }
        let i = (chs[index] as i32 - 48) as usize;
        let cs:Vec<char> = letters[i].chars().collect();
        for c in cs {
            buf.push(c);
            Self::dfs(chs,index+1,letters,ans,buf);
            buf.pop();
        }
    }
    //广度优先
    fn bfs(digit_chars:&Vec<char>,letters:&Vec<String>)->Vec<String>{
        use std::collections::VecDeque;
        if digit_chars.len() ==0 {
            return Vec::new();
        }
        let mut deque:VecDeque<String> = VecDeque::new();
        let i = (digit_chars[0] as i32 - 48) as usize;
        let chars:Vec<char> = letters[i].chars().collect();
        
        let les:Vec<String> = chars.iter().map(|v| (*v).to_string()).collect();
        //第一层的元素放入队列
        deque = VecDeque::from(les);
         let m = digit_chars.len();
        let mut j = 1;//从第二层开始循环
        let mut n = deque.len();
        let mut k = 0;
             while k < n && m > 1{
                 let i = (digit_chars[j] as i32 - 48) as usize;
                 let chars:Vec<char> = letters[i].chars().collect();
                 let mut st:String = deque.pop_front().unwrap();
                 for c in chars {
                     st.push(c);
                     deque.push_back(st.clone());
                     st.pop();
                 }
                 k += 1;
                 //当前层循环完时进入下一层，即j+1
                 if k == n && j < m-1 {
                     j += 1;
                     k = 0;
                     n = deque.len();
                 }
             }
         Vec::from(deque)
    }
}
// @lc code=end

