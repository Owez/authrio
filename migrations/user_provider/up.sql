CREATE TABLE user_provider (
    id INTEGER PRIMARY KEY,
    provider_id SERIAL FOREIGN KEY REFERENCES provider(id) NOT NULL,
    created TIMESTAMP WITH TIME ZONE NOT NULL
);

-- notes:
-- id is randomly generated