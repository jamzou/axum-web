-- 单点登录系统用户表结构
DROP TABLE IF EXISTS `mo_app_user`;
CREATE TABLE IF NOT EXISTS `mo_app_user` (
  `id` int unsigned NOT NULL AUTO_INCREMENT,
  `emp_id` varchar(10) NOT NULL COMMENT '工号（6位数字）',
  `user_name` varchar(50) NOT NULL COMMENT '用户名',
  `password` varchar(255) NOT NULL COMMENT '加密后的密码',
  `email` varchar(100) DEFAULT NULL COMMENT '邮箱',
  `phone` varchar(20) DEFAULT NULL COMMENT '手机号',
  `org_id` bigint DEFAULT NULL COMMENT '部门id',
  `role` varchar(50) DEFAULT 'user' COMMENT '用户角色，如admin,user等',
  `status` tinyint DEFAULT 1 COMMENT '用户状态：1-激活，0-禁用',
  `last_login_time` datetime DEFAULT NULL COMMENT '最后登录时间',
  `create_time` datetime DEFAULT CURRENT_TIMESTAMP COMMENT '创建时间',
  `creater_id` varchar(10) DEFAULT NULL COMMENT '创建者工号',
  `update_time` datetime DEFAULT CURRENT_TIMESTAMP ON UPDATE CURRENT_TIMESTAMP COMMENT '更新时间',
  `updater_id` varchar(10) DEFAULT NULL COMMENT '更新者工号',
  PRIMARY KEY (`id`),
  UNIQUE KEY `uk_emp_id` (`emp_id`),
  UNIQUE KEY `uk_user_name` (`user_name`),
  UNIQUE KEY `uk_email` (`email`)
) ENGINE=InnoDB AUTO_INCREMENT=6 DEFAULT CHARSET=utf8mb4 COLLATE=utf8mb4_0900_ai_ci;

-- 插入初始管理员用户
INSERT INTO `mo_app_user` (`id`, `emp_id`, `user_name`, `password`, `email`, `phone`, `org_id`, `role`, `status`, `create_time`, `creater_id`) VALUES
(1, 'admin001', 'admin', '$2b$12$D6Y5JyQ.zHdLsC5kRzGhKeFtXpVnWmJlOiGcBfArDsEuFvHjIgKq', 'admin@example.com', '13800138000', 1, 'admin', 1, NOW(), 'admin001');