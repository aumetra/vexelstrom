CREATE TABLE users (
    id UUID PRIMARY KEY,
    username TEXT NOT NULL,
    email TEXT NOT NULL,
    password TEXT NOT NULL,

    created_at TIMESTAMPTZ NOT NULL DEFAULT NOW(),
    updated_at TIMESTAMPTZ NOT NULL DEFAULT NOW(),

    UNIQUE (username),
    UNIQUE (email)
);

CREATE INDEX "idx-users-username" ON users (username);
CREATE INDEX "idx-users-email" ON users (email);

SELECT diesel_manage_updated_at('users');
