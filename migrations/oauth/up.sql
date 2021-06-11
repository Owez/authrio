-- Model for OAuth contents of one service which could be for to-us or to-service interactions
CREATE TABLE oauth (
    id SERIAL PRIMARY KEY,
    provider VARCHAR(20),
    token_access VARCHAR(40) NOT NULL,
    token_refresh VARCHAR(40),
    expiry TIMESTAMP WITH TIME ZONE,
    created TIMESTAMP WITH TIME ZONE NOT NULL,
);

-- notes:
-- provider may be authrio so can be nulled
-- token_refresh is optional in oauth to can be nulled
-- expiry is optional in oauth to can be nulled