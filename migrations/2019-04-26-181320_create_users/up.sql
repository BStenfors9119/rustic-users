CREATE TABLE `users` (
  `id` int(11) NOT NULL AUTO_INCREMENT,
  `uname` varchar(350) DEFAULT NULL,
  `email` varchar(45) DEFAULT NULL,
  `active` int(11) NOT NULL DEFAULT 0,
  PRIMARY KEY (`id`)
) ENGINE=InnoDB AUTO_INCREMENT=2 DEFAULT CHARSET=utf8mb4 COLLATE=utf8mb4_0900_ai_ci;