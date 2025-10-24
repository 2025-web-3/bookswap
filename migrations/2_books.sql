CREATE TABLE IF NOT EXISTS books (
	id BIGINT UNIQUE NOT NULL PRIMARY KEY,
	title varchar(512) NOT NULL,
	description varchar(4048) NOT NULL,
	author varchar NOT NULL,
	subjects varchar(1024),
	pages BIGINT,
	cover_url varchar,
	publish_date TIMESTAMP
);
