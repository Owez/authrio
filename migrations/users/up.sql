-- User model for all user related storage
CREATE TABLE users (
    id UUID PRIMARY KEY,
    pw_hash BLOB NOT NULL,
    pw_salt BLOB NOT NULL,
    oa_internal INTEGER UNIQUE FOREIGN KEY REFERENCES oauth(id),
    created TIMESTAMP WITH TIME ZONE NOT NULL
);

-- notes:
-- oa_internal is us using oauth for internal client-us logins