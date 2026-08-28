CREATE TABLE customers (
  id bigint GENERATED ALWAYS AS IDENTITY PRIMARY KEY,
  email text NOT NULL,
  joined_at timestamptz NOT NULL DEFAULT now()
);
INSERT INTO customers (email) VALUES
  ('aria@example.test'), ('ben@example.test'), ('chen@example.test'),
  ('dev@example.test'), ('eli@example.test'), ('fatima@example.test');
CREATE INDEX customers_joined_at_idx ON customers (joined_at);
