CREATE SCHEMA IF NOT EXISTS customer;

create table IF NOT EXISTS customer.customers(
    "id" int8 not null,
    "firstName" varchar(255),
    "lastName" varchar(255),
    "phone" varchar(255),
    "city" varchar(255),
    "address" varchar(255),
    primary key ("id")
);
