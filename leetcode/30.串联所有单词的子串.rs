/*
 * @lc app=leetcode.cn id=30 lang=rust
 *
 * [30] 串联所有单词的子串
 */

// @lc code=start
impl Solution {
    pub fn find_substring(s: String, words: Vec<String>) -> Vec<i32> {
        use std::collections::HashMap;
       //原字符串的长度
       let s_len = s.len();
       //单词的数量
       let ws_len = words.len();
       //每个单词的长度
       let w_len = words[0].len();
       //字串的长度
       let sub_len = ws_len*w_len;
       let mut res:Vec<i32> = Vec::new();
       if sub_len > s_len {
           
          return res;
       }

       //用于记录words中每个单词出现的次数
       let mut map:HashMap<String,i32> = HashMap::new();
       //统计words中每个单词出现的次数
       for w in &words {
           *map.entry(w.clone()).or_insert(0) += 1;
       }
       //保存字串开始索引对应的字串出现次数
       let mut indexs:Vec<HashMap<String,i32>> = Vec::new();
       let mut i:usize = 0;
       //保存各个起点的滑动窗口
       //"abcxxxx" -> abc\bcx\cxx
       while i < w_len&&i+sub_len<=s_len {
           let mut j = i;
           let mut tmp:HashMap<String,i32> = HashMap::new();
           while j < i + sub_len {
               let sub = &s[j..(j+w_len)];
            *tmp.entry(String::from(sub)).or_insert(0) += 1;
            j += w_len;
           }
           //println!("12344");
           indexs.push(tmp);
           //当前窗口的单词出现次数匹配，就将起点放入结果中
           if indexs[i] == map {
               res.push(i as i32);
           }
           i+=1;
       }
       //滑动窗口移动一个单词的长度，只要统计移除的单词和新入的单词
       for i in w_len..=(s_len-sub_len) {
            let left:usize = i%w_len;
            //移出的单词
            let sub1 = &s[i-w_len..i];
            //新入的单词
            let sub2 = &s[i+sub_len-w_len..i+sub_len];
            //移出的单词减1
            let count = indexs[left].entry(String::from(sub1)).or_insert(0);
            *count -= 1;
            //移出的单词小于等于0，直接从map中移除
            if *count < 1 {
                indexs[left].remove(&String::from(sub1));
            }
            //新入的单词加1
            *(indexs[left].entry(String::from(sub2)).or_insert(0)) += 1;
            //当前窗口的单词出现次数匹配，就将起点放入结果中
            if indexs[left] == map {
                res.push(i as i32);
            }
       }
     res
    }
}
// @lc code=end

