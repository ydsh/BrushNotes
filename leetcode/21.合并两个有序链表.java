/*
 * @lc app=leetcode.cn id=21 lang=java
 *
 * [21] 合并两个有序链表
 */

// @lc code=start
/**
 * Definition for singly-linked list.
 * public class ListNode {
 *     int val;
 *     ListNode next;
 *     ListNode() {}
 *     ListNode(int val) { this.val = val; }
 *     ListNode(int val, ListNode next) { this.val = val; this.next = next; }
 * }
 */
class Solution {
    public ListNode mergeTwoLists(ListNode l1, ListNode l2) {
     //迭代法
     //拟定一个头节点，根据题意可以使用最小值
     ListNode preHead = new ListNode(Integer.MIN_VALUE);
     //当前指针指向头节点
     ListNode prev = preHead;
     //比较两个链表的值，当前指针指向链表较小的节点，
     //修改链表指针指向下一个节点
     while(l1!=null&&l2!=null){
         if(l1.val<=l2.val){
             prev.next = l1;
             l1 = l1.next;
         }else{
             prev.next = l2;
             l2 = l2.next;
         }
             prev = prev.next;
     }
     //最后将当前指针指向的节点接上剩余的节点
     prev.next = l1!=null?l1:l2;
     //前置头节点是自己定义的，真正的头节点是它的下一个节点
     return preHead.next;
    }
}
// @lc code=end

