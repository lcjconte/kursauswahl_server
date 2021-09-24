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

INSERT INTO users(username, isAdmin)
VALUES ('admin', true);
