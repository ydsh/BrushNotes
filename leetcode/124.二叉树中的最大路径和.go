/*
 * @lc app=leetcode.cn id=124 lang=golang
 *
 * [124] 二叉树中的最大路径和
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
 //初始化整数最小值
 func maxPathSum(root *TreeNode) int {
	var maxSum int = ^int(^uint(0) >> 1)   
	maxGain(root,&maxSum)
	return maxSum
}
func maxGain(root *TreeNode,maxSum *int) int{
    if root==nil{
		return 0
	}
    //左右节点最大贡献值
	leftMax := max(maxGain(root.Left,maxSum),0)
	rightMax := max(maxGain(root.Right,maxSum),0)
    //当前路径之和
	curMax := root.Val+leftMax+rightMax
    //更新结果
	*maxSum = max(*maxSum,curMax)
    
	//返回最大贡献值
	return root.Val + max(leftMax,rightMax) 

}
func max(x, y int) int {
    if x > y {
        return x
    }
    return y
}
// @lc code=end

