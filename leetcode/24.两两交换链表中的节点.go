/*
 * @lc app=leetcode.cn id=24 lang=golang
 *
 * [24] 两两交换链表中的节点
 */

// @lc code=start
/**
 * Definition for singly-linked list.
 * type ListNode struct {
 *     Val int
 *     Next *ListNode
 * }
 */
func swapPairs(head *ListNode) *ListNode {
   //新建一个挂载点，链表挂在在上面
   var root *ListNode = &ListNode{
	   Val:0,
	   Next:nil,
   }
   root.Next = head;
   //移动指针，初始指向挂载点
   var cur *ListNode = root

   for cur.Next!=nil && cur.Next.Next!=nil {
	   var node1 *ListNode = cur.Next
	   var node2 *ListNode = cur.Next.Next
       
	   node1.Next = node2.Next
	   node2.Next = node1
       cur.Next = node2
	   cur = node1

   }
 return root.Next
}
// @lc code=end

