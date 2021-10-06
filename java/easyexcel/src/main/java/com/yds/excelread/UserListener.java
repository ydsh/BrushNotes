package com.yds.excelread;

import java.util.ArrayList;
import java.util.List;

import com.alibaba.excel.context.AnalysisContext;
import com.alibaba.excel.event.AnalysisEventListener;
import com.yds.model.User;

public class UserListener extends AnalysisEventListener<User> {
	//每次读取5条
	private static final int CACHE_COUNT = 5;
	private List<User> list = new ArrayList<User>(CACHE_COUNT);
    //解析数据
	public void invoke(User data, AnalysisContext context) {
		System.err.println("读取到一条数据："+data);
		//数据写入list
		list.add(data);
		//数据达到缓存量时保存数据
		if(list.size() >= CACHE_COUNT) {
			this.saveData();
			//保存完数据重新初始化list
			list = new ArrayList<User>(CACHE_COUNT);
		}
		
	}
    //解析完成后处理剩余的数据
	public void doAfterAllAnalysed(AnalysisContext context) {
		this.saveData();
		System.err.println("所有数据读取完成并保存成功");
	}
	private void saveData() {
		System.err.println("保存"+list.size()+"条数据");
	}

}
