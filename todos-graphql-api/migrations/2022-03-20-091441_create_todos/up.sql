-- Your SQL goes here
CREATE TABLE todos(
                      id SERIAL PRIMARY KEY,
                      title VARCHAR NOT NULL,
                      completed BOOLEAN NOT NULL
);

INSERT INTO todos(title,completed) VALUES('Coding in Java-Rust',FALSE);

INSERT INTO todos(title,completed) VALUES('Cooking Supper',FALSE);









