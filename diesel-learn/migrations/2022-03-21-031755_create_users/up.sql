-- Your SQL goes here

CREATE TABLE users (
                       id INTEGER PRIMARY KEY AUTO_INCREMENT,
                       name TEXT NOT NULL,
                       hair_color TEXT,
                       created_at TIMESTAMP NOT NULL DEFAULT CURRENT_TIMESTAMP,
                       updated_at TIMESTAMP NOT NULL DEFAULT CURRENT_TIMESTAMP ON UPDATE CURRENT_TIMESTAMP
);