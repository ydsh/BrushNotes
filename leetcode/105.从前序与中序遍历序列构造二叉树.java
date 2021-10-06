/*
 * @lc app=leetcode.cn id=105 lang=java
 *
 * [105] 从前序与中序遍历序列构造二叉树
 */

// @lc code=start
/**
 * Definition for a binary tree node.
 * public class TreeNode {
 *     int val;
 *     TreeNode left;
 *     TreeNode right;
 *     TreeNode() {}
 *     TreeNode(int val) { this.val = val; }
 *     TreeNode(int val, TreeNode left, TreeNode right) {
 *         this.val = val; 
 *         this.left = left;
 *         this.right = right;
 *     }
 * }
 */

class Solution {
    int[] preorder;
    int preIdx;
    Map<Integer,Integer> map = new HashMap<>();
    public TreeNode buildTree(int[] preorder, int[] inorder) {
     this.preorder = preorder;
     this.preIdx = 0;
     for(int i=0,len=inorder.length;i<len;++i){
         map.put(inorder[i],i);
     }
     return build(0,inorder.length);
    }
    //递归函数
    private TreeNode build(int left,int right){
        if (left >= right){
		return null;
	}
	int root_val = preorder[preIdx];
	TreeNode root = new TreeNode(root_val);
	int mid = map.get(root_val);
	preIdx++;
	root.left = build(left,mid);
	root.right = build(mid+1,right);
	return root;
    }
}
// @lc code=end

