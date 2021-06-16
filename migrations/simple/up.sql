CREATE TABLE simple (
    id UUID PRIMARY KEY,
    pw_hash BYTEA NOT NULL,
    pw_salt BYTEA NOT NULL,
    pw_created TIMESTAMP WITH TIME ZONE NOT NULL,
    created TIMESTAMP WITH TIME ZONE NOT NULL,
    org_id UUID FOREIGN KEY REFERENCES org(id)
);

-- notes:
-- auth is provided by the `simple_auth` table