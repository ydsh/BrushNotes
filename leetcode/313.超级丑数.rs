/*
 * @lc app=leetcode.cn id=313 lang=rust
 *
 * [313] 超级丑数
 */

// @lc code=start
impl Solution {
    pub fn nth_super_ugly_number(n: i32, primes: Vec<i32>) -> i32 {
      Self::dp(n, primes)
    }
    //动态规划寻找草鸡丑数
    fn dp(n: i32, primes: Vec<i32>)->i32{
      if n==1 {
          return 1;
      }
      use std::cmp::min;
      let ln = primes.len();
      //首先需要一个动态规划数组
      let mut dp:Vec<i32> = vec![0;(n+1) as usize];
      //初始化第二个元素为1，因为1是所有数的质因数，这样后面就可以得到
      //第n位超级丑数为dp[n]
      dp[1]=1;
      //这里还需要一个长度跟primes一样的索引数组（或者指针数组）ps，初始化
      //元素都为1，用来记录，dp[i]前面哪些数和primes相乘，从乘积的结果中
      //选取最小的值作为dp[i]的值。
      //例：primes=[2,3,7],ps=[1,1,1],dp[2]=Min(dp[ps[0]]*primes[0],
      //dp[ps[1]]*primes[1],dp[ps[2]]*primes[2]) = 2;
      let mut ps:Vec<usize> = vec![1;ln];
      for k in 2..=(n as usize) {
          dp[k] = dp[ps[0]]*primes[0];

          for i in 0..ln {
            dp[k] = min(dp[ps[i]]*primes[i], dp[k]);
          }
          //println!("{}",dp[k]);
          for j in 0..ln {
              //这里是最难理解的，其实这里就是记录下一个超级抽数，是从ps指向的
              //前面的超级丑数与primes乘积的结果中找出来的，这里的做法刚刚是
              //哪一个丑数与primes相乘得到最小的超级丑数，那么ps指向的那个超级
              //丑数就要+1了，因为我们在获取下一个超级丑数的时候，不可能再用ps
              //当前指向的丑数与primes相乘了，如果还用当前ps指向的丑数与primes
              //相乘，那我们得到的又是当前的超级丑数了，所以ps要指向下一位了，ps
              //的其他指向因为不是当前的超级丑数有关，所以变。
              if dp[k] == dp[ps[j]]*primes[j] {
                  ps[j] += 1;
              }
              
          }
      }
      dp[n as usize]
    }
    //其他解法还可以用最小堆、有序不重复集合
}
// @lc code=end
 
