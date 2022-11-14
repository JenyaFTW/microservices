CREATE SCHEMA IF NOT EXISTS ware;

CREATE TABLE IF NOT EXISTS ware.ware (
    id SERIAL,
    name TEXT NOT NULL,
    price float8 NOT NULL,
    PRIMARY KEY (id)
);