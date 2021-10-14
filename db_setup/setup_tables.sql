DROP TABLE IF EXISTS courses;
DROP TABLE IF EXISTS users;
DROP TABLE IF EXISTS choices;

CREATE TABLE courses  (
    id SERIAL,
    uname varchar(3) UNIQUE,
    fullname varchar(40),
    PRIMARY KEY(id)
);

CREATE TABLE users (
    id SERIAL,
    username varchar(40) UNIQUE,
    pwdhash varchar(60),
    isAdmin bool,
    PRIMARY KEY(id)
);

CREATE TABLE choices (
    id SERIAL,
    made_by int,
    course integer,
    PRIMARY KEY(id),
    CONSTRAINT fk_user
      FOREIGN KEY(made_by) 
	  REFERENCES users(id),
    CONSTRAINT fk_course
      FOREIGN Key(course)
      REFERENCES courses(id)
);

INSERT INTO users VALUES (default, 'admin', '$2a$06$WxeH6I2S2zG7xG1dDWR7veLhiEy5f8c6jAvJtY.Vi5eReXNcjE.m.', true);