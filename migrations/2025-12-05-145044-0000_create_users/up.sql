-- Your SQL goes here
CREATE TABLE IF NOT EXISTS `mo_app_user`(
  `id` int unsigned NOT NULL AUTO_INCREMENT,
  `emp_id` varchar(10) NOT NULL COMMENT '工号（6位数字）',
  `user_name` varchar(50) NOT NULL COMMENT '用户名',
  `org_id` bigint DEFAULT NULL COMMENT '部门id',
  `age` tinyint unsigned DEFAULT NULL COMMENT '年龄',
  `birthday` varchar(20) CHARACTER SET utf8mb4 COLLATE utf8mb4_0900_ai_ci DEFAULT NULL COMMENT '生日，格式yyyy-MM-dd HH:mm:ss',
  `create_time` datetime DEFAULT NULL COMMENT '创建时间',
  `creater_id` varchar(10) DEFAULT NULL COMMENT '创建者工号',
  `update_time` datetime DEFAULT NULL COMMENT '更新时间',
  `updater_id` varchar(10) DEFAULT NULL COMMENT '更新者工号',
  PRIMARY KEY (`id`)
) ENGINE=InnoDB DEFAULT CHARSET=utf8mb4 COLLATE=utf8mb4_0900_ai_ci;