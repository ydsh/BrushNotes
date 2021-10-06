/*
 * @lc app=leetcode.cn id=297 lang=java
 *
 * [297] 二叉树的序列化与反序列化
 */

// @lc code=start
/**
 * Definition for a binary tree node.
 * public class TreeNode {
 *     int val;
 *     TreeNode left;
 *     TreeNode right;
 *     TreeNode(int x) { val = x; }
 * }
 */
public class Codec {

    // Encodes a tree to a single string.
    public String serialize(TreeNode root) {
        return encodedTree(root,"");
    }

    // Decodes your encoded data to tree.
    public TreeNode deserialize(String data) {
        String[] strArr = data.split(",");
        List<String> list = new ArrayList<String>(Arrays.asList(strArr));
        return decodedTree(list);
    }
    //前序遍历二叉树，生成逗号分隔的字符串
    private String encodedTree(TreeNode root,String txt){
        if(root==null){
         txt += "null,";
        }else{

        txt += String.valueOf(root.val)+",";
        txt = encodedTree(root.left,txt);
        txt = encodedTree(root.right,txt);
        }
        return txt;
    }
    //从二叉树前序遍历的数组中生成二叉树
    private TreeNode decodedTree(List<String> list){
      if(list.get(0).equals("null")){
          list.remove(0);
         return null;
      }

      TreeNode root = new TreeNode(Integer.parseInt(list.get(0)));
      list.remove(0);
      root.left = decodedTree(list);
      root.right = decodedTree(list);
      return root;
    }
}

// Your Codec object will be instantiated and called as such:
// Codec ser = new Codec();
// Codec deser = new Codec();
// TreeNode ans = deser.deserialize(ser.serialize(root));
// @lc code=end

