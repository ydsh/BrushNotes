/*
 * @lc app=leetcode.cn id=46 lang=rust
 *
 * [46] 全排列
 */

// @lc code=start
impl Solution {
    pub fn permute(nums: Vec<i32>) -> Vec<Vec<i32>> {
       let len = nums.len();
       let mut nums = nums;
       let mut res:Vec<Vec<i32>> = Vec::new();
       
       //Self::backtrack(&mut nums,len,0,&mut res);
       let mut used:Vec<bool> = vec![false;len];
       let mut tmp:Vec<i32> = Vec::new();
       //如果有重复元素则先对数组进行排序；
       //nums.sort_by(|a,b|{a.cmp(b)});
       Self::dfs(&mut nums,0,len,&mut used,&mut res,&mut tmp);
       return res;
    }
    //回溯，交换元素位置
    fn backtrack(nums: &mut Vec<i32>,len:usize,idx:usize,res:&mut Vec<Vec<i32>>){
       use std::ptr::swap;
       if idx == len {
           //在这里其实是一个循环结束，得到的一个排列结果
          res.push((*nums).clone());
           return;
       }
           //idx后面的元素跟idx位置元素交换
            for i in idx..len {
                unsafe{
                    //在自己位置上不用交换，直接进入下一个位置
                    //if i == idx {
                    //    Self::backtrack(nums, len,idx+1,res);
                    //}else{
                        swap(&mut nums[i], &mut nums[idx]);
                        
                        Self::backtrack(nums, len,idx+1,res);
                        swap(&mut nums[i], &mut nums[idx]);
                    //}
                    
                }
            }
        
       
    }
    //回溯选择元素，初始化一个记录元素是否被使用的数组，
    //如果有重复元素，要事先对元素组进行排序，可以方便去重
    fn dfs(nums:&mut Vec<i32>,idx:usize,len:usize,used:&mut Vec<bool>,res:&mut Vec<Vec<i32>>,tmp:&mut Vec<i32>){
        if idx == len {
             res.push(tmp.clone());
             return;
         }
         for i in 0..len {
             //如果元素被使用过则进入喜下一轮循环
             if used[i] {
                 continue;
             }
             //这里是去重
             //if i > 0 && nums[i] == nums[i-1] && !used[i-1] { 
             //    continue;
             //}
             //标记当前元素被使用过
            used[i] = true;
            tmp.push(nums[i].clone());
            Self::dfs(nums,idx+1,len,used,res,tmp);
            tmp.pop();
            used[i] = false;
         }
    }
}
// @lc code=end

