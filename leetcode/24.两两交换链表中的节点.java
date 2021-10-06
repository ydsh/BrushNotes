/*
 * @lc app=leetcode.cn id=24 lang=java
 *
 * [24] 两两交换链表中的节点
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
    public ListNode swapPairs(ListNode head) {

        //新建一个挂载点，链表挂在在上面
        ListNode root = new ListNode(Integer.MIN_VALUE);
        root.next = head;
        //移动指针，开始时指向挂载点
        ListNode cur = root;

        while(cur.next!=null&&cur.next.next!=null){
           ListNode node1 = cur.next;
           ListNode node2 = cur.next.next;
           node1.next = node2.next;
           node2.next = node1;
           cur.next = node2;
           cur = node1;
        }
        return root.next;
    }
}
// @lc code=end

