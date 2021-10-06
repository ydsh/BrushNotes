/*
 * @lc app=leetcode.cn id=1079 lang=rust
 *
 * [1079] 活字印刷
 */

// @lc code=start
impl Solution {
    pub fn num_tile_possibilities(tiles: String) -> i32 {
        let len = tiles.len();
        let mut chars:Vec<char> = tiles.chars().collect();
        let mut used = vec![false;len];
        //一定要排序，方便过滤重复元素
        //chars.sort_by(|a,b| {a.cmp(b)});
        //Self::dfs(&chars, 0, len,&mut used);

        let mut letters:[i32;26] = [0;26];
        for i in 0..len {
            letters[(chars[i] as usize)-('A' as usize)] += 1;
        }

        Self::dfs2(&mut letters)
    }
    //深度优先
    fn dfs(chars:&Vec<char>,idx:usize,len:usize,used:&mut Vec<bool>)->i32{
      let mut sum = 0;
     // println!("idx = {}",idx);
      if idx == len {
          return 0;
      }
      for i in 0..len {
          //被使用过了或者跟之前的元素相同并且未被使用的都跳过
          //后面一个条件，以“AAB”来说，
          //       B
          //      / \x
          //     A   A 
          //遍历到二个A的时候，它与前面的A相同，所以右侧分支剪掉，
          //!used[i-1]这个怎么理解呢，因为同一层的遍历，标识元素
          //被使用了，后面有还原了，最后肯定是未被使用的。
          if used[i]||(i>0&&chars[i] == chars[i-1] && !used[i-1]) {
              continue;
          }else{
             used[i] = true;
             sum += Self::dfs(chars,idx+1,len,used) + 1;
             used[i] = false;
          }
      }
        return sum;
    }
    //深度优先统计字母出现的个数
    fn dfs2(letters:&mut [i32;26])->i32{
        let mut sum = 0;
       for i in 0..26 {
           //为零说明没有对应字符
           if letters[i] == 0 {
               continue;
           }
           //减去1表示已使用一次了
           letters[i] -= 1;
           sum += Self::dfs2(letters)+1;
           //还原
           letters[i] += 1;
       }
       return sum;
    }
}
// @lc code=end

