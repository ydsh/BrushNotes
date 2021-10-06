/*
 * @lc app=leetcode.cn id=1591 lang=rust
 *
 * [1591] 奇怪的打印机 II
 */

// @lc code=start
impl Solution {
    //拓扑排序，看是否有环
    pub fn is_printable(target_grid: Vec<Vec<i32>>) -> bool {
        use std::collections::HashMap;
        use std::cmp::min;
        use std::cmp::max;
        let row = target_grid.len();
        let col = target_grid[0].len();
       //保存每种颜色的的矩形范围,其中元组里的四个值可以认为是(x1,y1,x2,y2)最小值与最大值
       let mut rects:HashMap<i32,(i32,i32,i32,i32)> = HashMap::new();
       //遍历每单元格，获取每种颜色的矩形范围，
       for i in 0..row {
           for j in 0..col {
               let t = target_grid[i][j];
               if let (x1,y1,x2,y2) = rects.entry(t).or_insert((61,61,-1,-1)) {
                    *x1 = min(*x1, i as i32);
                    *y1 = min(*y1, j as i32);
                    *x2 = max(*x2, i as i32);
                    *y2 = max(*y2, j as i32);
               }
  
           }
        }
       //println!("{:?}",rects);
       //图的方向，比如先打印颜色A再打印颜色B,则图的方向为A->B
       //图的入度，比如A->B->C,A->C这里A的入度都是0，B的入度是1，C的入度是2
       //\\\\\\\\\\\\\\\\\\\\\\\\\\\\\\\\\\
       //保存拓扑图分方向
       let mut top_direction:Vec<i32> = Vec::new();
       //保存图的变，A->B也就是A到B的边,因为一个图可能与多跟图有边，
       //所以这里用一对多的关系
       let mut edges:HashMap<i32,Vec<i32>> = HashMap::new();
       //保存颜色的入度，（注意这里所有地方图和颜色是一个意思）
       let mut penetration:HashMap<i32,i32> = HashMap::new();
       //初始化所有颜色的入度
       for (k,_) in &rects {
        penetration.entry(*k).or_insert(0);
       }
        for i in 0..row {
            for j in 0..col {
                //从原矩形中获取一个颜色t
                let t = target_grid[i][j];
                //遍历每个颜色获得颜色值k和它的矩阵范围x1,y1,x2,y2
                for (k,(x1,y1,x2,y2)) in rects.iter() {
                    //当前颜色矩阵的范围
                   if x1 <= &(i as i32) && &(i as i32) <= x2 && y1 <= &(j as i32) && &(j as i32) <= y2 {
                       //原矩形的颜色t和当前颜色k不相同，因为是在颜色k的矩阵内，
                       //则说明颜色k先打印，颜色t后打印，即图的方向k->t。
                       if &t != k {
                           let set = edges.entry(*k).or_insert(vec![]);
                           //这里主要是去重，防止重复建边
                           if !set.contains(&t) {
                               (*set).push(t);
                               //颜色t的入度加1
                               let count = penetration.entry(t).or_insert(0);
                               *count += 1;
                           }
                       }
                   }
                }
            }
        }
        //上面拓扑图已经建立好了，现在可以对拓扑图进行排序
        //先建立一个队列存放入度为0的颜色
        let mut deque:Vec<i32> = Vec::new();
        for (k,v) in &penetration {
            if v == &0 {
                deque.push(*k);
                break;
            }
        }
        //println!("{:?}",penetration);
        while let Some(v) = deque.pop() {
            //拿到当前颜色指向的颜色，也就是边
            if let Some(es) = edges.get(&v) {
                //这些颜色的入度都减1
                for t in es {
                    if let Some(k) = penetration.get_mut(t) {
                        *k -= 1;
                    } 
                }
            }
            //删除当前入度为零的颜色；
            penetration.remove(&v);
            //重新找入度为0的颜色
            for (k,v) in &penetration {
                if v == &0 {
                    deque.push(*k);
                    break;
                }
            }
            
        }
        //还存在入度不为0的图，说明存在环，所以没法打印这样的矩形
        if !penetration.is_empty() {
            return false;
        }
        
        return true;
    }
    
}
// @lc code=end

