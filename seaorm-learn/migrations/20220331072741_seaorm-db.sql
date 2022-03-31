
-- mysql

use `seaorm-db`;

DROP TABLE IF EXISTS `todos`;
CREATE TABLE `todos` (
                         `todo_id` bigint NOT NULL AUTO_INCREMENT,
                         `todo_name` varchar(60) NOT NULL,
                         `todo_description` varchar(150) DEFAULT NULL,
                         `todo_date` datetime NOT NULL DEFAULT CURRENT_TIMESTAMP,
                         PRIMARY KEY (`todo_id`),
                         KEY `idx_todo_name_key` (`todo_name`)
) ENGINE=InnoDB AUTO_INCREMENT=2 DEFAULT CHARSET=utf8mb3;

insert into todos(todo_name, todo_description,todo_date) values('my road to a java', 'hello a java trait',now());

DROP TABLE IF EXISTS `cake`;

CREATE TABLE `cake` (
                        `id` int NOT NULL AUTO_INCREMENT,
                        `name` varchar(255) NOT NULL,
                        `desc` varchar(500),
                        PRIMARY KEY (`id`)
);

INSERT INTO `cake` (`id`, `name`,`desc`) VALUES
                                      (1, 'New York Cheese','a cake made by New York Cheese, very yummy'),
                                      (2, 'Chocolate Forest',null);

DROP TABLE IF EXISTS `fruit`;

CREATE TABLE `fruit` (
                         `id` int NOT NULL AUTO_INCREMENT,
                         `name` varchar(255) NOT NULL,
                         `cake_id` int DEFAULT NULL,
                         PRIMARY KEY (`id`),
                         CONSTRAINT `fk-fruit-cake` FOREIGN KEY (`cake_id`) REFERENCES `cake` (`id`)
);

INSERT INTO `fruit` (`id`, `name`, `cake_id`) VALUES
                                                  (1, 'Blueberry', 1),
                                                  (2, 'Rasberry', 1),
                                                  (3, 'Strawberry', 2);

INSERT INTO `fruit` (`name`, `cake_id`) VALUES
                                            ('Apple', NULL),
                                            ('Banana', NULL),
                                            ('Cherry', NULL),
                                            ('Lemon', NULL),
                                            ('Orange', NULL),
                                            ('Pineapple', NULL);

DROP TABLE IF EXISTS `filling`;

CREATE TABLE `filling` (
                           `id` int NOT NULL AUTO_INCREMENT,
                           `name` varchar(255) NOT NULL,
                           PRIMARY KEY (`id`)
);

INSERT INTO `filling` (`id`, `name`) VALUES
                                         (1, 'Vanilla'),
                                         (2, 'Lemon'),
                                         (3, 'Mango');

DROP TABLE IF EXISTS `cake_filling`;

CREATE TABLE `cake_filling` (
                                `cake_id` int NOT NULL,
                                `filling_id` int NOT NULL,
                                PRIMARY KEY (`cake_id`, `filling_id`),
                                CONSTRAINT `fk-cake_filling-cake` FOREIGN KEY (`cake_id`) REFERENCES `cake` (`id`),
                                CONSTRAINT `fk-cake_filling-filling` FOREIGN KEY (`filling_id`) REFERENCES `filling` (`id`)
);

INSERT INTO `cake_filling` (`cake_id`, `filling_id`) VALUES
                                                         (1, 1),
                                                         (1, 2),
                                                         (2, 2),
                                                         (2, 3);
