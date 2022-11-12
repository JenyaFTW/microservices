CREATE SCHEMA IF NOT EXISTS orders;

CREATE TABLE IF NOT EXISTS orders.order
(
    id INT NOT NULL,
    client_id INT,
    PRIMARY KEY (id)
    );

CREATE TABLE IF NOT EXISTS orders.order_item_id
(
    order_id INT NOT NULL,
    item_id INT
);

ALTER TABLE IF EXISTS orders.order_item_id
    ADD FOREIGN KEY (order_id) REFERENCES orders.order