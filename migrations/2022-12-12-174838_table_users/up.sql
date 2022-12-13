-- Your SQL goes here
CREATE TABLE users (
  user_id UUID PRIMARY KEY,
  username VARCHAR(30) NOT NULL,
  password VARCHAR(50) NOT NULL,
  changed_password_at TIMESTAMP DEFAULT now(),
  email VARCHAR(100) NOT NULL UNIQUE,
  profile_id UUID NOT NULL UNIQUE,
  created_at TIMESTAMP DEFAULT now(),
  updated_at TIMESTAMP DEFAULT now(),
  CONSTRAINT fk_profile 
    FOREIGN KEY(profile_id)
      REFERENCES profile_users(profile_id)
      ON DELETE CASCADE
      ON UPDATE CASCADE
)
