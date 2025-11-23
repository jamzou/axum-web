-- --------------------------------------------------------
-- 主机:                           192.168.145.128
-- 服务器版本:                        8.1.0 - MySQL Community Server - GPL
-- 服务器操作系统:                      Linux
-- HeidiSQL 版本:                  12.5.0.6677
-- --------------------------------------------------------

/*!40101 SET @OLD_CHARACTER_SET_CLIENT=@@CHARACTER_SET_CLIENT */;
/*!40101 SET NAMES utf8 */;
/*!50503 SET NAMES utf8mb4 */;
/*!40103 SET @OLD_TIME_ZONE=@@TIME_ZONE */;
/*!40103 SET TIME_ZONE='+00:00' */;
/*!40014 SET @OLD_FOREIGN_KEY_CHECKS=@@FOREIGN_KEY_CHECKS, FOREIGN_KEY_CHECKS=0 */;
/*!40101 SET @OLD_SQL_MODE=@@SQL_MODE, SQL_MODE='NO_AUTO_VALUE_ON_ZERO' */;
/*!40111 SET @OLD_SQL_NOTES=@@SQL_NOTES, SQL_NOTES=0 */;

-- 导出  表 mo_office.mo_app_user 结构
DROP TABLE IF EXISTS `mo_app_user`;
CREATE TABLE IF NOT EXISTS `mo_app_user` (
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
) ENGINE=InnoDB AUTO_INCREMENT=6 DEFAULT CHARSET=utf8mb4 COLLATE=utf8mb4_0900_ai_ci;

-- 正在导出表  mo_office.mo_app_user 的数据：~4 rows (大约)
INSERT INTO `mo_app_user` (`id`, `emp_id`, `user_name`, `org_id`, `age`, `birthday`, `create_time`, `creater_id`, `update_time`, `updater_id`) VALUES
	(2, 'jamzo22u', '', NULL, 13, '2024-08-07', '2024-08-07 14:31:58', NULL, NULL, NULL),
	(3, 'jamzo33', 'N3333', NULL, 13, '2024-08-07', '2024-08-07 15:15:18', NULL, NULL, NULL),
	(4, 'jamzo33', '我是懒鬼2', NULL, 13, '2024-08-07', '2024-08-07 15:16:12', NULL, '2024-08-07 15:17:45', NULL),
	(5, 'jamzo3ww', 'N33233', NULL, 9, '2024-08-07', '2024-08-07 15:22:02', NULL, NULL, NULL);

/*!40103 SET TIME_ZONE=IFNULL(@OLD_TIME_ZONE, 'system') */;
/*!40101 SET SQL_MODE=IFNULL(@OLD_SQL_MODE, '') */;
/*!40014 SET FOREIGN_KEY_CHECKS=IFNULL(@OLD_FOREIGN_KEY_CHECKS, 1) */;
/*!40101 SET CHARACTER_SET_CLIENT=@OLD_CHARACTER_SET_CLIENT */;
/*!40111 SET SQL_NOTES=IFNULL(@OLD_SQL_NOTES, 1) */;
