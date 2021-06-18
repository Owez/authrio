CREATE TABLE provider (
    id VARCHAR(64) PRIMARY KEY,
    domain VARCHAR(2000) NOT NULL,
    redirect_uri VARCHAR(2000),
    scope VARCHAR(64),
    org_id UUID FOREIGN KEY REFERENCES org(id) NOT NULL,
);

-- notes:
-- the `id` is the client_id