CREATE TABLE accounts (
    id            UUID         PRIMARY KEY,
    email         VARCHAR(255) NOT NULL UNIQUE,
    password_hash VARCHAR(255) NOT NULL,

    first_name    VARCHAR(255) NOT NULL,
    family_name   VARCHAR(255) NOT NULL,
    last_name     VARCHAR(255) NOT NULL
);

CREATE TABLE restaurant_admins (
    id         UUID PRIMARY KEY,
    account_id UUID NOT NULL REFERENCES accounts
                                ON UPDATE CASCADE
                                ON DELETE CASCADE    
);
