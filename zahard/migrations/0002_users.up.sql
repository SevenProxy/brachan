CREATE TABLE users (
    id            BIGINT GENERATED ALWAYS AS IDENTITY PRIMARY KEY,
    username      TEXT NOT NULL UNIQUE,
    password TEXT NOT NULL,
    role          TEXT NOT NULL DEFAULT 'user'
                  CHECK (role IN ('user', 'moderator', 'admin')),
    is_active     BOOLEAN NOT NULL DEFAULT TRUE,
    created_at    TIMESTAMPTZ NOT NULL DEFAULT now()
);
