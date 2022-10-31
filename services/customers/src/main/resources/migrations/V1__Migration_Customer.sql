CREATE SCHEMA IF NOT EXISTS customer;

create table IF NOT EXISTS customer.customers(
    "id" int8 not null,
    "address" varchar(255),
    "city" varchar(255),
    "first_name" varchar(255),
    "last_name" varchar(255),
    "phone" varchar(255),
    primary key ("id")
);