CREATE TABLE auth (
    id SERIAL PRIMARY KEY,
    provider VARCHAR(40),
    token_access VARCHAR(40) NOT NULL,
    token_refresh VARCHAR(40),
    expiry TIMESTAMP WITH TIME ZONE,
    created TIMESTAMP WITH TIME ZONE,
    user_id FOREIGN KEY NOT NULL REFERENCES user(id)
);