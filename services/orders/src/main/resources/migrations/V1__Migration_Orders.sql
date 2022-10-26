CREATE SCHEMA IF NOT EXISTS orders;

CREATE TABLE IF NOT EXISTS orders.order
(
    "id" INT8 NOT NULL,
    "client_id" INT8,
    PRIMARY KEY ("id")
);

CREATE TABLE IF NOT EXISTS orders.order_item_id
(
    "order_id" INT8 NOT NULL,
    "item_id" INT8
);

ALTER TABLE IF EXISTS orders.order_item_id
    ADD FOREIGN KEY ("order_id") REFERENCES "order"

