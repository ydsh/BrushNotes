/*
 * @lc app=leetcode.cn id=19 lang=rust
 *
 * [19] 删除链表的倒数第 N 个结点
 */

// @lc code=start
// Definition for singly-linked list.
// #[derive(PartialEq, Eq, Clone, Debug)]
// pub struct ListNode {
//   pub val: i32,
//   pub next: Option<Box<ListNode>>
// }
//
// impl ListNode {
//   #[inline]
//   fn new(val: i32) -> Self {
//     ListNode {
//       next: None,
//       val
//     }
//   }
// }
impl Solution {
    pub fn remove_nth_from_end(head: Option<Box<ListNode>>, n: i32) -> Option<Box<ListNode>> {
       use std::collections::VecDeque;
        //存储节点的栈
        let mut deque:VecDeque<Box<ListNode>> = VecDeque::new();
        let mut root = head;
        //先用一个栈顺序存储每个节点
        while let Some(mut v) = root {
            root = v.next;
            v.next=None;
            deque.push_back(v);
        }
        root = None;
        
        let mut i = 1;
        //从栈中依次弹出节点，并加入链表中，遇到倒数第n个节点就跳过
        while !deque.is_empty() {
             if i == n {
                 deque.pop_back();
             }else if let Some(mut v) = deque.pop_back(){
                  v.next=root;
                  root = Some(v);
             }
             i += 1;
        }
       root
    }
}
// @lc code=end

