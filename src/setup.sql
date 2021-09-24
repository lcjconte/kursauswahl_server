DROP TABLE IF EXISTS courses;
DROP TABLE IF EXISTS users;
DROP TABLE IF EXISTS choices;

CREATE TABLE courses  (
    id SERIAL PRIMARY KEY,
    uname varchar(3) UNIQUE,
    fullname varchar(40)
);

CREATE TABLE users (
    id SERIAL PRIMARY KEY,
    username varchar(40),
    isAdmin bool
);

CREATE TABLE choices (
    id SERIAL PRIMARY KEY,
    user INTEGER FOREIGN KEY,
    course INTEGER FOREIGN KEY,
);

INSERT INTO users(username, isAdmin)
VALUES ('admin', true);
