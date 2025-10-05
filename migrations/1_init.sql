CREATE TABLE IF NOT EXISTS users (
	id BIGINT PRIMARY KEY NOT NULL UNIQUE,
	username varchar(32) NOT NULL UNIQUE,
	first_name varchar(64) NOT NULL,
	second_name varchar(64) NOT NULL
)
