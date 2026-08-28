CREATE TABLE customers (id UInt64, email String, joined_at DateTime) ENGINE = MergeTree ORDER BY id;
INSERT INTO customers VALUES (1, 'aria@example.test', now()), (2, 'ben@example.test', now());
