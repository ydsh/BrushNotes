/*
 * @lc app=leetcode.cn id=297 lang=golang
 *
 * [297] 二叉树的序列化与反序列化
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

type Codec struct {
    Data string
	Stack []string
}

func Constructor() Codec {
   return Codec{
		Data:"",
		Stack:make([]string, 0),
	}
}

// Serializes a tree to a single string.
func (this *Codec) serialize(root *TreeNode) string {
    //根据前序遍历生成字符串
	if root == nil{
		this.Data += "nil,"
	}else{
		this.Data += strconv.Itoa(root.Val) + ","
		//根据前序遍历规则，先找左节点再找右节点
		this.Data = this.serialize(root.Left)
		this.Data = this.serialize(root.Right)
	}
	
	return this.Data
}

// Deserializes your encoded data to tree.
func (this *Codec) deserialize(data string) *TreeNode {   
	this.Stack = strings.Split(data,",")
   
	return this.recur()
}
//递归，每次从列表中取出第一个元素创建节点
func (this *Codec) recur() *TreeNode {
	
	if len(this.Stack)==0||this.Stack[0] == "nil"{
		//如果列表不为空，当前节点为空，移除列表第一个元素
		if len(this.Stack) > 0{
			this.Stack=this.Stack[1:]
		}
		return nil
	}
	//取出列表第一个元素转成数字
	value,_:=strconv.Atoi(this.Stack[0])
	//使用取出的value新建当前节点
	var root *TreeNode = &TreeNode{
		Val:value,
		Left:nil,
		Right:nil,
	}
	//移除列表第一个元素
	this.Stack = this.Stack[1:]
    //根据前序遍历规则，先找左节点再找右节点
	root.Left = this.recur()
	root.Right = this.recur()
   return root
}

/**
 * Your Codec object will be instantiated and called as such:
 * ser := Constructor();
 * deser := Constructor();
 * data := ser.serialize(root);
 * ans := deser.deserialize(data);
 */
// @lc code=end

