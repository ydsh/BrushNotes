/*
 * @lc app=leetcode.cn id=6 lang=golang
 *
 * [6] Z 字形变换
 */

// @lc code=start
func convert(s string, numRows int) string {
    if len(s) < 2|| numRows< 2{
		return s
	}
    //map保存对应行的字符串
	var rowMap map[int]string = make(map[int]string, 0)
	//沿着列的方向，true为向下，false为向上
	var isDown bool = true;
	//行号，从0开始 
	var row int = 0
    //遍历字符
	for _,ch := range s{
		
		if _,ok := rowMap[row];ok{
			rowMap[row] += string(ch)
		}else{
			rowMap[row] = string(ch)
		}
		//row指向第一行时，沿着行增加的方向
		if row == 0{
			isDown = true
		}
		//row指向最后一行，沿着行减少的方向
		if row == numRows-1{
			isDown = false
		}
		//isDown为true表示行增加，false表示行减少
		if isDown  {
			row++
		}else {
			row--
		}

	}
	//将所有的行拼接起来
    for i:=1;i<numRows;i++{
		rowMap[0] += rowMap[i]
	}
   return rowMap[0]
}
// @lc code=end

