/*
 * @lc app=leetcode.cn id=106 lang=golang
 *
 * [106] 从中序与后序遍历序列构造二叉树
 */

// @lc code=start
/**
 * Definition for a binary tree node.
 * type TreeNode struct {
 *     Val int
 *     Left *TreeNode
 *     Right *TreeNode
 * }
 */
func buildTree(inorder []int, postorder []int) *TreeNode {
  if postorder==nil||len(postorder)==0{
	 return nil
  }
 var inIdx int = len(inorder) - 1
 var postIdx int = len(postorder)-1
 var  root TreeNode = TreeNode{
	  Val:postorder[postIdx],
	  Left:nil,
	  Right:nil,
  }
  stack:=make([]*TreeNode, 0,postIdx+1)
  stack = append(stack,&root);
for i := postIdx-1; i>=0; i-- {
	 node := stack[len(stack)-1]
    if node.Val != inorder[inIdx] {
        node.Right = &TreeNode{
			Val:postorder[i],
	        Left:nil,
	        Right:nil,
		}
		stack = append(stack,node.Right)
	}else{
        for len(stack)>0&&stack[len(stack)-1].Val==inorder[inIdx] {
			node = stack[len(stack)-1]
			stack = stack[0:len(stack)-1]
			inIdx -= 1
		}
		node.Left = &TreeNode{
			Val:postorder[i],
			Left:nil,
			Right:nil,
		}
		stack = append(stack,node.Left)
	}
}
return &root
}
// @lc code=end

