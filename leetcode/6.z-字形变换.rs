/*
 * @lc app=leetcode.cn id=6 lang=rust
 *
 * [6] Z 字形变换
 */
//根据组成Z字形字符的规律，
//第一行和最后一行（即rows-1行）的字符在原字符串的
//位置为k*(rows-1)*2+r,其中在Z字形里k为列索引，r为行索引；
//除第一行和最后一行外，其它中间行的字符在原字符串的位置
//为k*(rows-1)*2-r 或 k*(rows-1)*2+r;
//例如，第二行，r=1，
//第一列字符的位置0*(rows-1)*2+1;
//第二列字符的位置1*(rows-1)*2-1;
//第三列字符的位置1*(rows-1)*2+1;
//第四列字符的位置2*(rows-1)*2-1;
//...
// @lc code=start
impl Solution {
    pub fn convert(s: String, num_rows: i32) -> String {
        let n = s.len();
        if n < 2||num_rows<2{
            return s;
        }
        let chars : Vec<char> = s.chars().collect();
        let rows : usize = num_rows as usize; 
        let cycle : usize = (rows-1)*2;
        let mut result : String = String::from("");
        for i in 0..rows{
           let mut idx : usize = i;
           //k为行元素索引
           let mut k : usize = 0;
           while idx < n {
               //第一行和最后一行
               if i==0||i==rows-1{
                  result.push(chars[idx]);
                  k+=1;
                  idx = k*cycle+i;
                }else{//其他中间行
                    result.push(chars[idx]);
                    k+=1;
                    idx = k*cycle - i;
                    if idx < n{
                        result.push(chars[idx]);
                    }
                    idx = k*cycle + i;
                }
           }
        }
     return result;
    }
}
// @lc code=end

