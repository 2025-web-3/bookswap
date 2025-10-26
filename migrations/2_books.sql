CREATE TABLE IF NOT EXISTS books (
	id BIGINT UNIQUE NOT NULL PRIMARY KEY,
	isbn varchar(13),
	title varchar(512) NOT NULL,
	description varchar(4048) NOT NULL,
	author varchar NOT NULL,
	subjects varchar(1024),
	pages BIGINT,
	cover_url varchar,
	publish_date TIMESTAMP
);

CREATE TABLE IF NOT EXISTS books_sharing (
	id BIGINT UNIQUE NOT NULL PRIMARY KEY,
	book_id BIGINT NOT NULL REFERENCES books(id) ON DELETE CASCADE,
	comment varchar(2048),
	holder_id BIGINT NOT NULL REFERENCES users(id) ON DELETE CASCADE,
	condition SMALLINT NOT NULL
);

CREATE TABLE IF NOT EXISTS books_requests (
	id BIGINT UNIQUE NOT NULL PRIMARY KEY,
	book_holding_id BIGINT NOT NULL,
	borrower_id BIGINT REFERENCES users(id) ON DELETE CASCADE,
	is_accepted BOOLEAN DEFAULT NULL,
	accepted_at TIMESTAMP,
	borrowed_at TIMESTAMP,
	return_at TIMESTAMP
);
