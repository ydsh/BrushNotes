/*
 * @lc app=leetcode.cn id=21 lang=rust
 *
 * [21] 合并两个有序链表
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
    pub fn merge_two_lists(l1: Option<Box<ListNode>>, l2: Option<Box<ListNode>>) -> Option<Box<ListNode>> {
        let mut l1 = l1;
        let mut l2 = l2;
        //头节点
        let mut head = Some(Box::new(ListNode::new(i32::MIN)));
        //当前指针
        let mut cur = &mut head;
        loop {
            match (l1.take(),l2.take()) {
                (None,Some(v))|(Some(v),None) => {
                    if let Some(ref mut p) = cur {
                        p.next = Some(v);
                    }
                    break;
                },
                //(Some(v),None) => {
                //    if let Some(ref mut p) = cur {
                //        p.next = Some(v);
                //    }
                //    break;
                //},
                (Some(mut v1),Some(mut v2)) => {
                    let mut node = None;
                   if v1.val >= v2.val {
                       //l2指向下一个，这里要take出来，
                       //因为v2要移动到Some里
                       l2 = v2.next.take();
                       node = Some(v2);
                       //l1被take了这里要还原
                       l1 = Some(v1);
                   }else{
                       //l1指向下一个，这里要take出来，
                       //因为v1要移动到Some里
                       l1 = v1.next.take();
                       node = Some(v1);
                       //l2被take了这里要还原
                       l2 = Some(v2);
                   }
                   if let Some(ref mut p) = cur {
                      p.next = node;
                      cur = &mut p.next;
                   }
                },
                (None,None) => {
                    break;
                },
            }
        }

        head.unwrap().next
    }
}
// @lc code=end

