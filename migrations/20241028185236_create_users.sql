CREATE TABLE IF NOT EXISTS users
(
    username        VARCHAR(50) NOT NULL UNIQUE,
    password        VARCHAR(100) NOT NULL
);