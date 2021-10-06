/*
 * @lc app=leetcode.cn id=1028 lang=java
 *
 * [1028] 从先序遍历还原二叉树
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
    public TreeNode recoverFromPreorder(String traversal) {
        String[] strArr = traversal.split("-");
        return resolvTree(strArr);
    }
    //使用迭代法
    private TreeNode resolvTree(String[] strArr){
         //使用队列保存当前节点路径
         Deque<TreeNode> stack = new LinkedList<TreeNode>();
         //当前节点值的位置
         int  pos = 0;
         while(pos < strArr.length){
            //当前深度
            int deep = 0;
            while("".equals(strArr[pos])){
                deep++;
                pos++;
            }
            //因为字符串分割成数组时，减少了一个字符，所以深度要+1
            if(pos>0){
               deep++;
            }
            //创建当前节点
            TreeNode node = new TreeNode(Integer.parseInt(strArr[pos]));
            pos++;
            if(deep==stack.size()){
                //当前节点作为队列最后一个节点的下一个左节点
               if(!stack.isEmpty()){
                 stack.peek().left = node;
               }   
            }else{
                while(deep!=stack.size()){
                   stack.pop();
                }
                stack.peek().right = node;
            }
               stack.push(node);

         }
         //保留根节点，用于返回
         while(stack.size()>1){
            stack.pop();
         }
        return stack.pop();
    }
}
// @lc code=end

