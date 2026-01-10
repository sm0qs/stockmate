CREATE TABLE photo_directories (
	id INTEGER PRIMARY KEY AUTOINCREMENT,
	path TEXT NOT NULL UNIQUE
);

CREATE TABLE photos_metadata (
	id INTEGER PRIMARY KEY AUTOINCREMENT,
	directory_id INTEGER NOT NULL,
	filename TEXT NOT NULL,
	file_hash TEXT NOT NULL,
	file_size INTEGER NOT NULL,
	mod_time DATETIME NOT NULL,
	date_taken DATETIME,
	width INTEGER,
	height INTEGER,
	camera_model TEXT,
	FOREIGN KEY (directory_id) REFERENCES photo_directories (id) ON DELETE CASCADE,
	UNIQUE (directory_id, filename)
);

CREATE TABLE keywords (
	id INTEGER PRIMARY KEY AUTOINCREMENT,
	name TEXT NOT NULL UNIQUE
);

CREATE TABLE photo_keywords (
	photo_id INTEGER NOT NULL,
	keyword_id INTEGER NOT NULL,
	PRIMARY KEY (photo_id, keyword_id),
	FOREIGN KEY (photo_id) REFERENCES photos_metadata (id) ON DELETE CASCADE,
	FOREIGN KEY (keyword_id) REFERENCES keywords (id) ON DELETE CASCADE
);

CREATE TABLE IF NOT EXISTS upload_services (
	id INTEGER PRIMARY KEY AUTOINCREMENT,
	service_type TEXT NOT NULL,
	name TEXT NOT NULL,
	username TEXT,
	password TEXT,
	api_key TEXT,
	UNIQUE (service_type, username)
);

CREATE TABLE IF NOT EXISTS photo_uploads (
	photo_id INTEGER NOT NULL,
	service_id INTEGER NOT NULL,
	upload_date DATETIME DEFAULT CURRENT_TIMESTAMP,
	remote_url TEXT,
	remote_id TEXT,
	PRIMARY KEY (photo_id, service_id),
	FOREIGN KEY (photo_id) REFERENCES photos_metadata (id) ON DELETE CASCADE,
	FOREIGN KEY (service_id) REFERENCES upload_services (id) ON DELETE CASCADE
);
