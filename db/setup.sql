DROP DATABASE IF EXISTS respite;
CREATE DATABASE respite;

\c respite;

CREATE TABLE IF NOT EXISTS owners (
  id VARCHAR(64) PRIMARY KEY,
  email VARCHAR(128) NOT NULL UNIQUE,
  password VARCHAR(128) NOT NULL
);

CREATE TABLE IF NOT EXISTS restaurants (
  id VARCHAR(64) PRIMARY KEY,
  name VARCHAR(128) NOT NULL,
  description text,
  owner_id VARCHAR(64) REFERENCES owners (id),
  logo VARCHAR(128)
);

CREATE TABLE IF NOT EXISTS items (
  id VARCHAR(64) PRIMARY KEY,
  name VARCHAR(128) NOT NULL,
  description text,
  category VARCHAR(128),
  rest_id VARCHAR(64) REFERENCES restaurants (id),
  image VARCHAR(128)
);

CREATE TABLE IF NOT EXISTS orders (
  id VARCHAR(64) PRIMARY KEY,
  item_id VARCHAR(64) REFERENCES items(id),
  requested_at DECIMAL NOT NULL,
  completed BOOLEAN NOT NULL,
  table_number SMALLINT,
  description text
);
