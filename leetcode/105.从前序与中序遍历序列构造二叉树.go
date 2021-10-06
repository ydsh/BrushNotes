/*
 * @lc app=leetcode.cn id=105 lang=golang
 *
 * [105] 从前序与中序遍历序列构造二叉树
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
 //迭代法
func buildTree(preorder []int, inorder []int) *TreeNode {
	if preorder==nil||len(preorder)==0{
		return nil
	}
	var length int = len(preorder)
	var inIdx = 0
	var root *TreeNode = &TreeNode{
		Val:preorder[0],
		Left:nil,
		Right:nil,
	}
	//用slice当作栈，存放前序遍历的节点
	var stack []*TreeNode = make([]*TreeNode, 0)
	//push操作，将节点压入栈
	stack = append(stack,root)
	for i := 1; i < length; i++ {
		//peek操作，访问栈顶元素
		var node *TreeNode = stack[len(stack)-1]
		if node.Val != inorder[inIdx]{
           node.Left = &TreeNode{
			Val:preorder[i],
			Left:nil,
			Right:nil,
		   }
		   //push操作，将节点压入栈
		   stack = append(stack,node.Left)
		}else{
            for len(stack)!=0&&
			stack[len(stack)-1].Val == inorder[inIdx]{
              node = stack[len(stack)-1]
			  //pop操作，将栈顶元素弹出
			  stack = stack[0:len(stack)-1]
			  inIdx +=1
			}
            node.Right = &TreeNode{
				Val:preorder[i],
				Left:nil,
				Right:nil,
			   }
			   //节点入栈
			stack = append(stack,node.Right)
		}
	}
	return root
}

// @lc code=end

