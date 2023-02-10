CREATE DATABASE respite;

\c respite;

CREATE TABLE IF NOT EXISTS owners (
  id VARCHAR(64) PRIMARY KEY,
  email VARCHAR(128) NOT NULL,
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
