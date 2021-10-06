/*
 * @lc app=leetcode.cn id=47 lang=rust
 *
 * [47] 全排列 II
 */

// @lc code=start
impl Solution {
    pub fn permute_unique(nums: Vec<i32>) -> Vec<Vec<i32>> {
       let len = nums.len();
       let mut nums =nums;
       let mut res:Vec<Vec<i32>> = Vec::new();
       //Self::dfs(&mut nums,len,0,&mut res);
       
       let mut used:Vec<bool> = vec![false;len];
       let mut tmp: Vec<i32> = Vec::new();
       nums.sort_by(|a,b| {a.cmp(b)});
       Self::dfs2(&mut nums,0,len,&mut used,&mut res,&mut tmp);
       return res;
    }
    fn dfs(nums:&mut Vec<i32>,len:usize,idx:usize,res:&mut Vec<Vec<i32>>){
        use std::ptr::swap;
        let mut idx = idx;
        if idx == len {
            res.push(nums.clone());
            return;
        }
        for i in idx..len {
            //检查idx到i之间的元素包括idx，和i位置的元素比较
            //有相同的则不交换，也不进入下一个分支
           if Self::is_swap(nums,idx,i) {
                unsafe{
                  swap(&mut nums[i], &mut nums[idx]);
                  //println!("{:?}",nums);
                  Self::dfs(nums,len,idx + 1,res);
                  swap(&mut nums[i], &mut nums[idx]);
                }
            }
        }
    }
    //检查元素是否可以交换,根据start到end之间的元素，有与end位置的元素相同则返回false
    fn is_swap(nums:&Vec<i32>,start:usize,end:usize)->bool{
            for i in start..end {
                if nums[i] == nums[end] {
                    return false;
                }
            }
        return true;
    }
    //回溯选择元素，不过要事先对元素进行排序方便去重，还要初始化一个记录元素是否被使用的数组
    fn dfs2(nums:&mut Vec<i32>,idx:usize,len:usize,used:&mut Vec<bool>,res:&mut Vec<Vec<i32>>,tmp:&mut Vec<i32>){
        if idx == len {
             res.push(tmp.clone());
             return;
         }
         for i in 0..len {
             //如果元素被使用过或者元素与全面的元素相则进入喜下一轮循环
             if used[i] || (i > 0 && nums[i] == nums[i-1] && !used[i-1]) {
                 continue;
             }
             //标记当前元素被使用过
            used[i] = true;
            tmp.push(nums[i].clone());
            Self::dfs2(nums,idx+1,len,used,res,tmp);
            tmp.pop();
            used[i] = false;
         }
    }
}
// @lc code=end

