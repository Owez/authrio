-- Model for external OAuth services which can link to users
CREATE TABLE extauth (
    id SERIAL PRIMARY KEY,
    provider VARCHAR(20),
    token_access VARCHAR(40) NOT NULL,
    token_refresh VARCHAR(40),
    expiry TIMESTAMP WITH TIME ZONE,
    created TIMESTAMP WITH TIME ZONE NOT NULL,
    FOREIGN KEY user_id REFERENCES user(id) NOT NULL,
);

-- notes:
-- `provider` may be authrio so can be nulled
-- `token_refresh` is optional in oauth to can be nulled
-- `expiry` is optional in oauth to can be nulled