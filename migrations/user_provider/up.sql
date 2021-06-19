CREATE TABLE user_provider (
    id INTEGER PRIMARY KEY,
    token_access VARCHAR(64) NOT NULL,
    token_refresh VARCHAR(64),
    token_expires TIMESTAMP WITH TIME ZONE,
    provider_id SERIAL FOREIGN KEY REFERENCES provider(id) NOT NULL,
    created TIMESTAMP WITH TIME ZONE NOT NULL
);

-- notes:
-- id is randomly generated