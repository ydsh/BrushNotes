/*
 * @lc app=leetcode.cn id=165 lang=rust
 *
 * [165] 比较版本号
 */

// @lc code=start
impl Solution {
    pub fn compare_version(version1: String, version2: String) -> i32 {
        //普通解法
        //let arr1:Vec<i32> = version1.split(".").map(|s|s.parse::<i32>().unwrap()).collect();
        //let arr2:Vec<i32> = version2.split(".").map(|s|s.parse::<i32>().unwrap()).collect();
        //let len1 = arr1.len();
        //let len2 = arr2.len();
        //let n = if len1 > len2 {len1} else{len2};
        //
        //for i in 0..n {
        //    let mut a1 = 0;
        //    let mut a2 = 0;
        //  if i < len1 {
        //    a1 = arr1[i];
        //  }
        //  if i < len2 {
        //    a2 = arr2[i];
        //  }
        //  if a1 > a2 {
        //      return 1;
        //  }else if a1 < a2 {
        //      return -1;
        //  }
        //}
        // return 0;
        //双指针法
        let len1 = version1.len();
        let len2 = version2.len();
        let mut n1:usize = 0;
        let mut n2:usize = 0;
        while n1 < len1 || n2 < len2 {
            let mut a1 = 0;
            let mut a2 = 0;
             while let Some(v) = version1.get(n1..n1+1) {
                  n1 += 1;
                 if v == "." {
                     break;
                 }
                a1 += a1*10 + v.parse::<i32>().unwrap();
             }
             while let Some(v) = version2.get(n2..n2+1) {
                n2 += 1;
               if v == "." {
                   break;
               }
              a2 += a2*10 + v.parse::<i32>().unwrap();
           }
          if a1 > a2 {
              return 1;
          }else if a1 < a2 {
              return -1;
          }
        }

        return 0;
    }
}
// @lc code=end

