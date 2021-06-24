-- Your SQL goes here
CREATE TABLE users (
  id SERIAL NOT NULL PRIMARY KEY,
  email VARCHAR(255) UNIQUE NOT NULL,
  full_name VARCHAR(255) NOT NULL,
  password TEXT NOT NULL,
  role_id INTEGER NOT NULL,
  address TEXT NOT NULL,
  phone_number VARCHAR(20) UNIQUE NOT NULL
);