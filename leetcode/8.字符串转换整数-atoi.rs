/*
 * @lc app=leetcode.cn id=8 lang=rust
 *
 * [8] 字符串转换整数 (atoi)
 */

// @lc code=start
use std::collections::HashMap;
impl Solution {
    pub fn my_atoi(s: String) -> i32 {
       let chars:Vec<char> = s.chars().collect();
       let mut atoi = Atoi::new();
       for ch in chars {
           atoi.set_data(ch);
       }
       atoi.get_ans()
    }
}
struct Atoi<'a>{
ans:i32,
sign:i32,
state:&'a str,
table:HashMap<&'a str,[&'a str;4]>
}
impl <'a> Atoi<'a>{
  fn new()->Self{
    let mut table:HashMap<&str,[&str;4]> = HashMap::new();
     table.insert("start", ["start","sign","number","end"]);
     table.insert("sign", ["end","end","number","end"]);
     table.insert("number", ["end","end","number","end"]);
     table.insert("end", ["end","end","end","end"]);
     Atoi{
          ans:0,
          sign:1,
          state:"start",
          table:table
      }
  }
  fn get_ans(&self)->i32{
   self.ans
  }
  fn get_index(&self,ch:char)->usize{
     if ch==' ' {
         return 0;
     }else if ch=='+'||ch=='-' {
         return 1;
     }else if ch<='9'&&ch>='0' {
         return 2;
     }
         return 3;
  }
  fn set_data(&mut self,ch:char){
    let max = std::i32::MAX;
    let min = std::i32::MIN;
    self.state = if let Some(v) = self.table.get(self.state) {
        v[self.get_index(ch)]
    }else{
        "end"
    };
    if "sign"==self.state {
        self.sign = if ch=='-' {
            -1
        }else{
            1
        };
    }else if "number"==self.state {
        let num = ch as i32 - 48;
        if self.sign == 1 && (self.ans > max/10||(self.ans == max/10 && num > max%10)) {
            self.ans = max;
        }else if self.sign == -1 && (self.ans < min/10||(self.ans == min/10 && -num < min%10)) {
            self.ans = min;
        }else{
            self.ans = self.ans*10 + self.sign*(ch as i32 - 48);
        }
    }
  }
}
// @lc code=end

