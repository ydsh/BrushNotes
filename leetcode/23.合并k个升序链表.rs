/*
 * @lc app=leetcode.cn id=23 lang=rust
 *
 * [23] 合并K个升序链表
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
    pub fn merge_k_lists(lists: Vec<Option<Box<ListNode>>>) -> Option<Box<ListNode>> {
       let k = lists.len();
       let mut i:usize = 1;
       let mut j:usize = 0;
       let mut lists = lists;
       if k == 0 {
           return None;
       }
       //分治法，两两连接，放到原数组中
       //[1,2,3,4,5] 12->[0],34->[2],5->[4]
       while i < k {
           while j < k - i {
               lists[j] = Self::merge_2_lists(lists[j].clone(),lists[j+i].clone());
               j += 2*i;
            }
           
           j = 0;
           i *= 2;
       }
           
       lists[0].clone()
    }
    //合并两个有序链表
    fn merge_2_lists(l1:Option<Box<ListNode>>, l2:Option<Box<ListNode>>)->Option<Box<ListNode>>{
       let mut l1 = l1;
       let mut l2 = l2;
        //前置节点，用来连接后边的链表
        let mut head = Some(Box::new(ListNode::new(i32::MIN)));
        //移动指针
        let mut root = &mut head;
        loop {
            match (l1.take(),l2.take()) {
                (Some(mut v1),Some(mut v2)) => {
                   let mut node = None;
                     if v1.val <= v2.val {
                         //取出节点，不影响所有权
                         l1 = v1.next.take();
                         //所有权移动到这里
                         node = Some(v1);
                         //l2还原，因为之前被取出了
                         l2 = Some(v2);
                     }else{
                         //取出节点，不影响所有权
                         l2 = v2.next.take();
                         //所有权移动到这里
                         node = Some(v2);
                         //l1还原，因为之前被取出了
                         l1 = Some(v1);
                     }

                     if let Some(ref mut p) = root {
                         p.next = node;
                         root = &mut p.next;
                     }
                },
                (Some(mut v),None)|(None,Some(mut v)) => {
                    if let Some(ref mut p) = root {
                        p.next = Some(v);
                    }
                    break;
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

