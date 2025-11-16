CREATE TABLE users (
    id INTEGER PRIMARY KEY AUTOINCREMENT,
    username TEXT NOT NULL,
    email TEXT NOT NULL,
    password_hash TEXT NOT NULL,
    rank INTEGER NOT NULL DEFAULT 1200,
    created_at DATETIME NOT NULL DEFAULT CURRENT_TIMESTAMP,
    updated_at DATETIME,
    deleted_at DATETIME
);

CREATE UNIQUE INDEX idx_users_username_unique
    ON users (username)
    WHERE deleted_at IS NULL;


CREATE UNIQUE INDEX idx_users_email_unique
    ON users (email)
    WHERE deleted_at IS NULL;

CREATE TRIGGER users_set_updated_at
AFTER UPDATE ON users
FOR EACH ROW
WHEN NEW.updated_at IS NULL
BEGIN
    UPDATE users
    SET updated_at = CURRENT_TIMESTAMP
    WHERE id = NEW.id;
END;

CREATE TRIGGER users_soft_delete_timestamp
AFTER UPDATE ON users
FOR EACH ROW
WHEN OLD.deleted_at IS NULL AND NEW.deleted_at IS NOT NULL
BEGIN
    UPDATE users
    SET deleted_at = COALESCE(NEW.deleted_at, CURRENT_TIMESTAMP)
    WHERE id = NEW.id;
END;
