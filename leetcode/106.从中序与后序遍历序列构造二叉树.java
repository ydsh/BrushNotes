/*
 * @lc app=leetcode.cn id=106 lang=java
 *
 * [106] 从中序与后序遍历序列构造二叉树
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
    
    public TreeNode buildTree(int[] inorder, int[] postorder) {
      if(postorder==null||inorder.length==0){
          return null;
      }
      int inIdx = inorder.length-1;
      int postIdx = postorder.length-1; 
      TreeNode root = new TreeNode(postorder[postIdx]);
      Deque<TreeNode> stack = new LinkedList<TreeNode>();
      stack.push(root);
      //迭代法构建二叉树
      for(int i = postIdx-1;i>=0;--i){
           TreeNode node = stack.peek();
           if(node.val!=inorder[inIdx]){
               node.right = new TreeNode(postorder[i]);
               stack.push(node.right);
           }else{
               while(!stack.isEmpty()&&stack.peek().val==inorder[inIdx]){
                  node = stack.pop();
                  inIdx-=1;
               }
               node.left = new TreeNode(postorder[i]);
               stack.push(node.left);
           }
      }
      return root;
    }

}
// @lc code=end

