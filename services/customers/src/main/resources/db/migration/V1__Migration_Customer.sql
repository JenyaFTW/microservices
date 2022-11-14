CREATE SCHEMA IF NOT EXISTS customers;

CREATE TABLE IF NOT EXISTS customers.customer (
    id SERIAL not null,
    first_name varchar(255),
    last_name varchar(255),
    phone varchar(255),
    city varchar(255),
    address varchar(255),
    primary key (id)
);
