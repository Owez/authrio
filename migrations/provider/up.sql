CREATE TABLE provider (
    id SERIAL PRIMARY KEY,
    client_id VARCHAR(64) NOT NULL,
    client_secret VARCHAR(64) NOT NULL,
    domain VARCHAR(2000) NOT NULL,
    redirect_uri VARCHAR(2000),
    scope VARCHAR(64),
    org_id UUID FOREIGN KEY REFERENCES org(id) NOT NULL,
);
