-- migrations/20210825231258_create_subscriptions_table.sql
-- Create "subscriptions" table

CREATE TABLE subscriptions (
  id uuid NOT NULL,
  PRIMARY KEY (id),
  email TEXT UNIQUE NOT NULL,
  name TEXT NOT NULL,
  created_at timestamptz NOT NULL
);
