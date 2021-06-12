CREATE TABLE user (
    id UUID PRIMARY KEY,
    pw_hash BYTEA NOT NULL,
    pw_salt BYTEA NOT NULL,
    pw_created TIMESTAMP WITH TIME ZONE NOT NULL,
    token_access VARCHAR(40),
    expiry TIMESTAMP WITH TIME ZONE,
    created TIMESTAMP WITH TIME ZONE NOT NULL,
);

-- notes:
-- `token_access` and `expiry` should both be present or not