/*
 * @lc app=leetcode.cn id=541 lang=rust
 *
 * [541] 反转字符串 II
 */

// @lc code=start
impl Solution {
    pub fn reverse_str(s: String, k: i32) -> String {
        let len = s.len();
        if len < 2 || k < 0{
            return s;
        }
        let mut chars:Vec<char> = s.chars().collect();
        //let mut start:usize = 0;
        //let mut end:usize = 0;
        //for i in 0..len {
        //    end += 1;
        //    if (end as i32)%(2*k) == 0 {
        //        Self::reverse(&mut chars, start, start+(k as usize)-1);
        //        start = end;
        //        
        //    }else if end==len&&(end as i32)%(2*k) >= k {
        //       
        //        Self::reverse(&mut chars, start, start+(k as usize)-1);
        //    }else if end==len&&(end as i32)%(2*k) < k {
        //        
        //        Self::reverse(&mut chars, start, start+(end%(2*(k as usize)))-1);
        //    }
        //}
        let mut i:usize = 0;
        use std::cmp::min;
        while i < len {
            Self::reverse(&mut chars, i, min(i+(k as usize),len)-1);
            i += (2*k) as usize;
        }
        chars.iter().collect::<String>()
    }
    //翻转字符
    fn reverse(chars:&mut Vec<char>,start:usize,end:usize){
          let mut left = start;
          let mut right = end;

          while left < right {
              unsafe{
                  use std::ptr::swap;
                  swap(&mut chars[left], &mut chars[right]);
                  left += 1;
                  right -= 1;
              }
          }
    }
}
// @lc code=end

