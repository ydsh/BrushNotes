/*
 * @lc app=leetcode.cn id=222 lang=rust
 *
 * [222] 完全二叉树的节点个数
 */

// @lc code=start
// Definition for a binary tree node.
// #[derive(Debug, PartialEq, Eq)]
// pub struct TreeNode {
//   pub val: i32,
//   pub left: Option<Rc<RefCell<TreeNode>>>,
//   pub right: Option<Rc<RefCell<TreeNode>>>,
// }
//
// impl TreeNode {
//   #[inline]
//   pub fn new(val: i32) -> Self {
//     TreeNode {
//       val,
//       left: None,
//       right: None
//     }
//   }
// }
//这道题使用深度优先方法，广度优先方法也可以解决
use std::rc::Rc;
use std::cell::RefCell;
impl Solution {
    pub fn count_nodes(root: Option<Rc<RefCell<TreeNode>>>) -> i32 {
        //深度优先遍历，二叉树的前序遍历
        fn dfs(root: Option<Rc<RefCell<TreeNode>>>)->i32{
            let mut sum = 0;
            //当前节点不为空，节点数伽1
            if let Some(v) = root {
                sum += 1;
                let root = v.borrow();
                sum += dfs(root.left.clone());
                sum += dfs(root.right.clone());
            }else{
                sum +=0;
            }
            sum
        }
        //广度度优先遍历
        fn bfs(root:Option<Rc<RefCell<TreeNode>>>)->i32{
            use std::collections::VecDeque;
        //rust队列，有栈和队列性质，用来存放当前层的节点
         let mut deque:VecDeque<Option<Rc<RefCell<TreeNode>>>> = VecDeque::new();
         let mut count = 0;
         //判断根节点是否为空，如果为空直接返回0
         //不为空则将其放入队列
         if let Some(_) = root {
             deque.push_back(root);
             while !deque.is_empty() {
                if let Some(v) = deque.pop_front() {
                    count += 1;
                    if let Some(k) = v {
                        let root = k.borrow();
                        if let Some(_) = root.left {
                            deque.push_back(root.left.clone());
                        }
                        if let Some(_) = root.right {
                            deque.push_back(root.right.clone());
                        }
                    }
                   
                }
             }
         }
         count
        }
        //dfs(root)
        bfs(root)
    }
}
// @lc code=end

