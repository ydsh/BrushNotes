/*
 * @lc app=leetcode.cn id=509 lang=rust
 *
 * [509] 斐波那契数
 */

// @lc code=start
impl Solution {
    ///```
    /// 快速幂矩阵：
    /// F0 = 0, F1 = 1,
    /// |1 1||F1|  |F1+F0|    即   |F2|
    /// |1 0||F0|  |F1   |         |F1|
    /// 
    /// 所以，
    /// |1 1|n |F1|   |Fn+1|
    /// |1 0|  |F0|   |Fn  |
    /// 令M=
    /// |1 1|
    /// |1 0|
    /// 只要求出M的n次幂就可以求出Fn的值
    /// ```
    pub fn fib(n: i32) -> i32 {
        if n < 2 {
            return n;
        }
        let m = vec![
          vec![1,1],
          vec![1,0]
        ];
      let res = Self::pow(m,n-1);
      //let ss = Self::mul(&vec![
      // vec![1,0],
      // vec![0,1]
      //],
      // &vec![
      // vec![1,1],
      // vec![1,0]
      //);
      //println!("ss={:?}",ss);
       res[0][0]
    }
    //矩阵的n次幂
    fn pow(m:Vec<Vec<i32>>,n:i32)->Vec<Vec<i32>>{
        let row = m.len();
        let col = m[0].len();
        
        //单位矩阵
        let mut um:Vec<Vec<i32>> = Vec::new();
        for i in 0..row {
            let mut tmp:Vec<i32> = Vec::with_capacity(col);
            tmp.resize(col, 0);
            for j in 0..col {
                if i==j {
                    tmp[j] = 1;
                }
            }
            um.push(tmp);
        }
        //矩阵n次乘法
        let mut n = n;
        let mut m = m;
       while n > 0 {
           if n&1 == 1 {
               um = Self::mul(&um,&m);
           }
           n >>= 1;
           m = Self::mul(&m,&m);
       }
       return um;
    }
    //矩阵乘法
    fn mul(m1:&Vec<Vec<i32>>,m2:&Vec<Vec<i32>>)->Vec<Vec<i32>> {
        let row1 = m1.len();
        //let row2 = m2.len();
        let col1 = m1[0].len();
        let col2 = m2[0].len();
        let mut res:Vec<Vec<i32>> = Vec::with_capacity(row1);
        
        for i in 0..row1 {
            let mut tmp:Vec<i32> = Vec::with_capacity(col2);
            //初始化，也可以使用vec!宏
            tmp.resize(col2, 0);
            for j in 0..col2 {
                let mut num:i32 = 0;
                for k in 0..col1 {
                   num += m1[i][k] * m2[k][j];
                }
                tmp[j] = num;
            }
            res.push(tmp);
        }

        return res;
    }
}
// @lc code=end

