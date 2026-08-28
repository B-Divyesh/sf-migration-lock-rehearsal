-- Deliberately simple sample: observe an ACCESS EXCLUSIVE DDL path on Postgres.
ALTER TABLE customers ADD COLUMN migration_flag boolean NOT NULL DEFAULT false;
