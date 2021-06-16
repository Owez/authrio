CREATE TABLE simple_auth (
    id UUID PRIMARY KEY FOREIGN KEY REFERENCES simple(id),
    token_access VARCHAR(64) NOT NULL,
    token_refresh VARCHAR(64),
    expiry TIMESTAMP WITH TIME ZONE,
    provider FOREIGN KEY REFERENCES provider(id),
);

-- notes:
-- the `id` is directly linked to a simple id
-- if `provider` is empty, this is a local auth