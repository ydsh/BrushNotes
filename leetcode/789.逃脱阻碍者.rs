/*
 * @lc app=leetcode.cn id=789 lang=rust
 *
 * [789] 逃脱阻碍者
 */

// @lc code=start
impl Solution {
    pub fn escape_ghosts(ghosts: Vec<Vec<i32>>, target: Vec<i32>) -> bool {
          //谁到目的地的距离越短谁就先到达目的地，
          //因此玩家和ghost，玩家如果能逃脱，必须玩家位置到目的地的距离比所有ghost位置到达目的地的距离都小。

          let len = ghosts.len();
          //玩家到目的地的距离
          let dis_p = target[0].abs()+target[1].abs();
          for i in 0..len {
              //ghost到达目的地的距离
             let dis_g = (target[0]-ghosts[i][0]).abs() + (target[1]-ghosts[i][1]).abs();
             if dis_g <= dis_p {
                 return false;
             }
          }
          return true;
    }
}
// @lc code=end

