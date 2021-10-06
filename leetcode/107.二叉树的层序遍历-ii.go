/*
 * @lc app=leetcode.cn id=107 lang=golang
 *
 * [107] 二叉树的层序遍历 II
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
 //深度优先遍历，从列表头部添加层序
func levelOrderBottom(root *TreeNode) [][]int {
	var list [][]int = make([][]int, 0)
    dfs(root,0,&list)
	return list
}
func dfs(root *TreeNode,level int,list *[][]int){
	if root==nil {
		return
	}
	length:=len(*list)
	if level == length {
		ls:= make([]int, 0)
		//从前面追加元素
		*list = append([][]int{ls},*list...)
	}
    //当前list的长度为size,在list中序号为0，
	//应该为当前最后一层，
	//所以，第一层的序号为size-1-level,
	length=len(*list)
	lst:=(*list)[length-level-1] ;
	(*list)[length-level-1] = append(lst,root.Val)
	//从左到右
	dfs(root.Left,level+1,list)
	dfs(root.Right,level+1,list)
}
// @lc code=end

