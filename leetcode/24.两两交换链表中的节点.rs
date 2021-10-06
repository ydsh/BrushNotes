/*
 * @lc app=leetcode.cn id=24 lang=rust
 *
 * [24] 两两交换链表中的节点
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
    pub fn swap_pairs(head: Option<Box<ListNode>>) -> Option<Box<ListNode>> {
        let mut head = head;
        //链表挂载节点
        let mut root = Some(Box::new(ListNode::new(0)));
        //root = head; 
        //移动指针，初始指向挂载节点
        let mut cur = &mut root;
        //从链表头节点开始遍历
        while let Some(mut node1) = head {
           //取出下一个节点
           head = node1.next.take();
           if let Some(mut node2) = head {
               //头指针指向下一轮循环的开始节点
               head = node2.next.take();
               //节点修改
               node2.next = Some(node1);
               //连接节点，修改移动指针
               if let Some(ref mut v) = cur {
                   v.next = Some(node2);
                   //移动指针移动两个节点的距离
                   cur = &mut v.next;
                   if let Some(ref mut v) = cur {
                       cur = &mut v.next;
                    }
               }

           }else{
            if let Some(ref mut v) = cur {
                v.next = Some(node1);
            }
           }
        }
        root.unwrap().next
    }
}
// @lc code=end

