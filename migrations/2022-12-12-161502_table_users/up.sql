-- Your SQL goes here
CREATE TABLE users (
  user_id UUID PRIMARY KEY DEFAULT uuid_generate_v4(),
  username VARCHAR(30) NOT NULL,
  password VARCHAR NOT NULL,
  changed_password_at TIMESTAMP NOT NULL DEFAULT now(),
  email VARCHAR(100) NOT NULL UNIQUE,
  created_at TIMESTAMP NOT NULL DEFAULT now(),
  updated_at TIMESTAMP NOT NULL DEFAULT now()
)
