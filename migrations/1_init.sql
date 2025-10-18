CREATE TABLE IF NOT EXISTS users (
	id BIGINT PRIMARY KEY NOT NULL UNIQUE,
	username varchar(32) NOT NULL UNIQUE,
	first_name varchar(64) NOT NULL,
	second_name varchar(64) NOT NULL,
	password_hash varchar NOT NULL,
	email varchar NOT NULL UNIQUE,
	school_name varchar,
	permissions BIGINT DEFAULT 0 NOT NULL
);

CREATE TABLE IF NOT EXISTS sessions (
	user_id BIGINT NOT NULL REFERENCES users(id) ON DELETE CASCADE,
	token_hash varchar(32) NOT NULL UNIQUE,
	ip_address varchar NOT NULL,
	created_at timestamp DEFAULT CURRENT_TIMESTAMP NOT NULL
);
