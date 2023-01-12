CREATE EXTENSION IF NOT EXISTS "uuid-ossp";

CREATE TABLE users (
    id uuid DEFAULT uuid_generate_v4(),
    email VARCHAR NOT NULL,
    password VARCHAR NOT NULL,
    code VARCHAR NOT NULL,
    verified BOOL NOT NULL,
    updated_at TIMESTAMPTZ NOT NULL DEFAULT current_timestamp,
    created_at TIMESTAMPTZ NOT NULL DEFAULT current_timestamp,
    PRIMARY KEY (id)
);
