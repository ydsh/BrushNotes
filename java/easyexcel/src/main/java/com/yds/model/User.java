package com.yds.model;

import com.alibaba.excel.annotation.ExcelProperty;

public class User {
	/**
	 * 用户ID
	 */
	@ExcelProperty(value = "用户ID")
	private String userId;
	/**
	 * 用户名称
	 */
	@ExcelProperty(value = "用户名称")
	private String userName;
	/**
	 * 电话号码
	 */
	@ExcelProperty(value = "电话号码")
	private String phone;
	/**
	 * 邮箱地址
	 */
	@ExcelProperty(value = "邮箱地址")
	private String email;
	/**
	 * 职业
	 */
	@ExcelProperty(value = "职业")
	private String job;

	public String getUserId() {
		return userId;
	}

	public void setUserId(String userId) {
		this.userId = userId;
	}

	public String getUserName() {
		return userName;
	}

	public void setUserName(String userName) {
		this.userName = userName;
	}

	public String getPhone() {
		return phone;
	}

	public void setPhone(String phone) {
		this.phone = phone;
	}

	public String getEmail() {
		return email;
	}

	public void setEmail(String email) {
		this.email = email;
	}

	public String getJob() {
		return job;
	}

	public void setJob(String job) {
		this.job = job;
	}

	@Override
	public String toString() {
		return "User [userId=" + userId + ", userName=" + userName + ", phone=" + phone + ", email=" + email + ", job="
				+ job + "]";
	}

}
