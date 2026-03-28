-- Add migration script here
CREATE TABLE users (
  id SERIAL PRIMARY KEY,
  username VARCHAR(8) NOT NULL,
  password_hash TEXT NOT NULL
);
CREATE TABLE messages (
  id SERIAL PRIMARY KEY,
  user_id INT REFERENCES users(id),
  content VARCHAR(600) NOT NULL,
  created_at TIMESTAMPTZ DEFAULT NOW()
);
