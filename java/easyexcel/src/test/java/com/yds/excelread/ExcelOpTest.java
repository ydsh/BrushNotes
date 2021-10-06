package com.yds.excelread;

import static org.junit.jupiter.api.Assertions.*;

import java.util.ArrayList;
import java.util.List;

import org.junit.jupiter.api.Test;

import com.yds.model.User;

class ExcelOpTest {

	@Test
	void test() {
		//fail("Not yet implemented");
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
		assertTrue(true);
	}

}
