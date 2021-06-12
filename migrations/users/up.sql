-- Model for all user related storage
CREATE TABLE users (
    id UUID PRIMARY KEY,
    pw_hash BYTEA NOT NULL,
    pw_salt BYTEA NOT NULL,
    token_access VARCHAR(40),
    expiry TIMESTAMP WITH TIME ZONE,
    created TIMESTAMP WITH TIME ZONE NOT NULL,
);

-- notes:
-- `token_access` and `expiry` control local OAuth usage
-- the `oauth` model has many-to-one relationship to here