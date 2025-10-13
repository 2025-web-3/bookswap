CREATE TABLE IF NOT EXISTS users (
	id BIGINT PRIMARY KEY NOT NULL UNIQUE,
	username varchar(32) NOT NULL UNIQUE,
	first_name varchar(64) NOT NULL,
	second_name varchar(64) NOT NULL,
	password_hash varchar NOT NULL,
	school_name varchar,
	permissions BIGINT DEFAULT 0 NOT NULL
);

CREATE TABLE IF NOT EXISTS sessions (
	id BIGINT PRIMARY KEY NOT NULL UNIQUE,
	user_id BIGINT NOT NULL REFERENCES users(id) ON DELETE CASCADE,
	token varchar(32),
	created_at timestamp DEFAULT CURRENT_TIMESTAMP NOT NULL
);
