import java.util.HashMap;
import java.util.Map;
/*
 * @lc app=leetcode.cn id=8 lang=java
 *
 * [8] 字符串转换整数 (atoi)
 */

// @lc code=start
class Solution {
    private  Map<String, String[]> map = new HashMap<>();
    private long ans = 0;
    private int sign = 1;
    private String state = "start";

     {
       map.put("start", new String[]{"start","sign","number","end"});
       map.put("sign", new String[]{"end","end","number","end"});
       map.put("number", new String[]{"end","end","number","end"});
       map.put("end", new String[]{"end","end","end","end"});
    }
    public long getAns() {
        return ans;
    }
    public int getSign() {
        return sign;
    }
    public String getState() {
        return state;
    }
    //根据字符获取数组索引
    private int getArrayIndex(char c){
        
        if(c==' '){
          return 0;
        }else if(c=='-'||c=='+'){
          return 1;
        }else if(Character.isDigit(c)){
          return 2;
        }
        return 3;
    }
    private void setData(char c){
        state = map.get(state)[getArrayIndex(c)];
        if("sign".equals(state)){
           sign = c=='-'?-1:1;
        }else if("number".equals(state)){
           ans = ans*10 + (c-'0');
           if(sign==-1&&ans >= -(long)Integer.MIN_VALUE){
              ans = -(long)Integer.MIN_VALUE;
           }else if(sign==1&&ans >= (long)Integer.MAX_VALUE){
              ans = (long)Integer.MAX_VALUE;
           }

        }
    }

    public int myAtoi(String s) {
        int len = s.length();
        for(int i=0;i<len;i++){
             setData(s.charAt(i));
        }

        return (int) (sign*ans);
    }
}
// @lc code=end
