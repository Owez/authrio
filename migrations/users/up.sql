-- User model for all user related storage
CREATE TABLE users (
    id UUID PRIMARY KEY
    pw_hash BLOB NOT NULL,
    pw_salt BLOB NOT NULL,
    oa_internal INTEGER UNIQUE FOREIGN KEY REFERENCES oauth(id)
);