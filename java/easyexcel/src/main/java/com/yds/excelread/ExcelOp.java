package com.yds.excelread;

import java.util.ArrayList;
import java.util.List;

import com.alibaba.excel.EasyExcel;
import com.alibaba.excel.ExcelWriter;
import com.alibaba.excel.write.metadata.WriteSheet;
import com.yds.model.User;

public class ExcelOp {
	public static void read() {
		 final String fileName = "C:/Users/Disen/OneDrive/桌面/user.xlsx";
		 EasyExcel.read(fileName, User.class,new UserListener()).sheet().doRead();
	}
   public static <T> void write(List<T> list) {
	   final String fileName = "C:/Users/Disen/OneDrive/桌面/user123.xlsx";
	   //写法1
	   EasyExcel.write(fileName, User.class).sheet("用户模板").doWrite(list);
	   
	   //写法2 需要关闭流，如果不关闭流会出现意想不到的结果，比如文件损坏
	   ExcelWriter excelWriter = null;
	   try {
		   excelWriter = EasyExcel.write(fileName, User.class).build();
		   WriteSheet writeSheet= EasyExcel.writerSheet("用户模板123").build();
		   excelWriter.write(list, writeSheet);
	} catch (Exception e) {
		System.err.println("error");
	}finally {
		if(excelWriter != null) {
			excelWriter.finish();
		}
	}
   }
	public static void main(String[] args) {
		ExcelOp.read();
		List<User> list = new ArrayList<User>();
		for(int i=0;i<1000000;i++) {
			User user = new User();
			user.setUserId("1001");
			user.setUserName("小明");
			user.setPhone("12345678901");
			user.setEmail("123456@123.com");
			user.setJob("工程师");
			list.add(user);
		}
		ExcelOp.write(list);
	}
}
