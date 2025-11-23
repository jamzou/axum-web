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

-- 导出  表 mo_office.mo_gl_org 结构
DROP TABLE IF EXISTS `mo_gl_org`;
CREATE TABLE IF NOT EXISTS `mo_gl_org` (
  `id` bigint unsigned NOT NULL AUTO_INCREMENT COMMENT '主键',
  `org_code` varchar(50) NOT NULL COMMENT '组织架构编码',
  `org_name` varchar(50) NOT NULL COMMENT '部门名称',
  `update_time` datetime DEFAULT NULL COMMENT '更新时间',
  PRIMARY KEY (`id`)
) ENGINE=InnoDB AUTO_INCREMENT=3 DEFAULT CHARSET=utf8mb4 COLLATE=utf8mb4_0900_ai_ci COMMENT='组织架构表';

-- 正在导出表  mo_office.mo_gl_org 的数据：~2 rows (大约)
DELETE FROM `mo_gl_org`;
INSERT INTO `mo_gl_org` (`id`, `org_code`, `org_name`, `update_time`) VALUES
	(1, '11-', '南方航空公司', '2025-11-02 13:36:14'),
	(2, '11-12', '信息中心', '2025-11-02 13:36:37');

/*!40103 SET TIME_ZONE=IFNULL(@OLD_TIME_ZONE, 'system') */;
/*!40101 SET SQL_MODE=IFNULL(@OLD_SQL_MODE, '') */;
/*!40014 SET FOREIGN_KEY_CHECKS=IFNULL(@OLD_FOREIGN_KEY_CHECKS, 1) */;
/*!40101 SET CHARACTER_SET_CLIENT=@OLD_CHARACTER_SET_CLIENT */;
/*!40111 SET SQL_NOTES=IFNULL(@OLD_SQL_NOTES, 1) */;
