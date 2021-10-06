/*
 * @lc app=leetcode.cn id=25 lang=rust
 *
 * [25] K 个一组翻转链表
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
    pub fn reverse_k_group(head: Option<Box<ListNode>>, k: i32) -> Option<Box<ListNode>> {
        let mut i = 1;
        let mut head = head;
        //链表挂载点
        let mut root:Option<Box<ListNode>> = Some(Box::new(ListNode::new(0)));
        //临时链表挂载节点
        let mut tmp:Option<Box<ListNode>> = Some(Box::new(ListNode::new(0)));
        //游标指针
        let mut cur = &mut tmp;
        while let Some(mut node) = head {
            head = node.next.take();
            //取出的节点node挂载到临时挂载节点上，
            //游标指针移动到当前节点
           if let Some(ref mut p) = cur {
               p.next = Some(node);
               cur = &mut p.next;
           }
           if i == k {
               let list = Self::reverse_list(tmp.unwrap().next);
               root = Self::connect_list(root,list);
               //临时链表挂载节点和游标指针重置
               tmp = Some(Box::new(ListNode::new(0)));
               cur = &mut tmp;
               i = 1;
           }else{
               i += 1;
           }
        }
        //剩余的节点直接连接起来
        if head.is_none() {
            root = Self::connect_list(root,tmp.unwrap().next);
        }
        root.unwrap().next

    }
    //连接两个链表，将list1的尾和list2的头链接
    fn connect_list(list1: Option<Box<ListNode>>,list2: Option<Box<ListNode>>)-> Option<Box<ListNode>>{
        let mut list1 = list1;
        let mut list2 = list2;
        let mut root:Option<Box<ListNode>> = Some(Box::new(ListNode::new(0))); 
        let mut cur = &mut root;
       
       while let Some(mut node) = list1 {
           list1 = node.next.take();
           if let Some(ref mut p) = cur {
               p.next = Some(node);
               cur = &mut p.next;
           }
           if list1.is_none() {
            if let Some(ref mut p) = cur { 
                p.next = list2.take();
            }
           }
       }
       root.unwrap().next
    }
    //反转链表
    fn reverse_list(head: Option<Box<ListNode>>) -> Option<Box<ListNode>> {
        let mut head = head;
       let mut root:Option<Box<ListNode>> = None;
       while let Some(mut node) = head {
           head = node.next.take();
          if let Some(mut p) = root {
              node.next = Some(p);
              root = Some(node);
          }else{
              root = Some(node);
          }
       }
       root
    }
}
// @lc code=end

